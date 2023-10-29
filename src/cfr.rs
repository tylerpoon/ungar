use super::{
    abstract_game::AbstractGame,
    game::{Action, PlayerId, MAX_PLAYERS},
    strategy::{ Strategy, Regrets },
    node::NodeId,
};

use std::{collections::BTreeMap, io::BufWriter};
use std::cmp::max;
use std::fs;
use std::path::Path;
use rand::Rng;
use rand::prelude::*;

use log::info;

use serde::{Serialize, Deserialize};

use poker::{Card, Evaluator};

#[derive(Debug, Serialize, Deserialize)]
pub struct CFRConfig {
    rounds_update_average_strategy: u8,
}

impl CFRConfig  {
    pub fn new(rounds_update_average_strategy: u8) -> CFRConfig {
        CFRConfig {
            rounds_update_average_strategy,
        }
    }

    pub fn from_config(path: &Path) -> CFRConfig {
        let cfr_config = serde_json::from_str(&fs::read_to_string(path).expect("failed to read cfr config")).expect("failed to deserialize cfr config");
        cfr_config
    }
}

pub struct CFREngine {
    abstract_game: AbstractGame,
    average_strategy: Strategy,
    regrets: Regrets,
    evaluator: Evaluator,
    config: CFRConfig,
}

impl CFREngine {
    pub fn new(abstract_game: AbstractGame, config: CFRConfig) -> CFREngine {
        CFREngine {
            abstract_game,
            average_strategy: Strategy::new(),
            regrets: Regrets::new(), 
            evaluator: Evaluator::new(),
            config,
        }
    }

    pub fn print_average_strategy(&self) {
        println!("{:?}", self.average_strategy);
    }

    pub fn save_average_strategy(&self, path: &Path) {
        let mut f = BufWriter::new(fs::File::create(path).unwrap());
        bincode::serialize_into(&mut f, &self.average_strategy).unwrap();
    }

    pub fn save_nodes(&self, path: &Path) {
        self.abstract_game.nodes.save(path);
    }

    pub fn print_regrets(&self) {
        println!("{:?}", self.regrets);
    }

    pub fn mccfr_p(&mut self, ticks: u32, strategy_interval: u32, prune_threshold: u32, lcfr_threshold: u32, discount_interval: u32) {
        let num_players = self.abstract_game.game_info.num_players();

        for t in 0..ticks {
            info!("Iteration {:?}", t);
            for i in 0..num_players {
                if t % strategy_interval == 0 {
                    let (hole_cards, board_cards) = self.abstract_game.game_info.deal_hole_cards_and_board_cards();
                    self.update_strategy(self.abstract_game.nodes.get_root_node_id(), &board_cards, self.abstract_game.game_info.total_board_cards(0) as usize, &hole_cards, i);
                }
                if t > prune_threshold {
                    let mut rng = rand::thread_rng();
                    if rng.gen::<f32>() < 0.05 {
                        let (hole_cards, board_cards) = self.abstract_game.game_info.deal_hole_cards_and_board_cards();
                        self.traverse_mccrfr(self.abstract_game.nodes.get_root_node_id(), &board_cards, self.abstract_game.game_info.total_board_cards(0) as usize, &hole_cards, i);
                    } else {
                        let (hole_cards, board_cards) = self.abstract_game.game_info.deal_hole_cards_and_board_cards();
                        self.traverse_mccrfr_p(self.abstract_game.nodes.get_root_node_id(), &board_cards, self.abstract_game.game_info.total_board_cards(0) as usize, &hole_cards, i);
                    }
                } else {
                        let (hole_cards, board_cards) = self.abstract_game.game_info.deal_hole_cards_and_board_cards();
                        self.traverse_mccrfr(self.abstract_game.nodes.get_root_node_id(), &board_cards, self.abstract_game.game_info.total_board_cards(0) as usize, &hole_cards, i);
                }
            }

            if t < lcfr_threshold && t % discount_interval == 0 {
                let d: f32 = (t as f32 / discount_interval as f32) / ((t as f32 / discount_interval as f32) + 1.);

                //CHECK: this type of multiplication tends to give overflow errors!
                for regrets in self.regrets.values_mut() {
                    for v in regrets.values_mut() {
                        *v = ((*v as f32) * d).round() as i32;
                    }
                }
                for strategy in self.average_strategy.0.values_mut() {
                    for v in strategy.values_mut() {
                        *v = ((*v as f32) * d).round() as i32;
                    }
                }
            }
        }
    }

    pub fn calculate_strategy(regrets: &BTreeMap<Action, i32>) -> BTreeMap<Action, f32> {
        let mut sum = 0;
        for v in regrets.values() {
            if *v > 0 {
                sum += *v;
            }
        }

        let mut strategy: BTreeMap<Action, f32> = BTreeMap::new();
        let length = regrets.len();
        for (k, v) in regrets.iter() {
            if sum > 0 {
                strategy.insert(*k, max(*v, 0) as f32 / sum as f32);
            } else {
                strategy.insert(*k, 1. / length as f32);
            }
        }

        strategy
    }

    fn sample_strategy(sigma: &BTreeMap<Action, f32>) -> Action {
        let mut rng = rand::thread_rng();
        *sigma.iter().collect::<Vec<(&Action, &f32)>>().choose_weighted(&mut rng, |item| item.1).unwrap().0
    }

    pub fn update_strategy(&mut self, node_id: NodeId, board_cards: &Vec<Card>, board_cards_i: usize, hole_cards: &[Vec<Card>; MAX_PLAYERS], player: PlayerId) {
        let current_node = self.abstract_game.nodes.get_node(node_id).unwrap();
        debug!("Updating strategy of node {node_id}");

        // CHECK: Doesn't generate average strategy past first betting round
        if current_node.state.is_finished() || current_node.state.has_folded(player) || current_node.state.current_round() > self.config.rounds_update_average_strategy {
            return;
        } else if current_node.state.current_player().unwrap() == player {
            let bucket_id = self.abstract_game.get_bucket(current_node.state.current_round(), &board_cards, &hole_cards[player as usize]);
            let regrets = self.regrets.entry((node_id, bucket_id))
                .or_insert_with(|| {
                    let mut regrets_map: BTreeMap<Action, i32> = BTreeMap::new();
                    for a in self.abstract_game.get_actions(&current_node.state) {
                        regrets_map.insert(a, 0); // inserts with uniform distribution
                    }
                    regrets_map
                });
            let sigma = CFREngine::calculate_strategy(regrets);
            let action = CFREngine::sample_strategy(&sigma);

            // Add one to action counter
            self.average_strategy.0.entry((node_id, bucket_id))
                .and_modify(|s| { let _ = *s.entry(action).and_modify(|x| *x += 1).or_insert(0); })
                .or_insert_with(|| {
                    let mut action_map: BTreeMap<Action, i32> = BTreeMap::new();

                    for a in self.abstract_game.get_actions(&current_node.state) {
                        action_map.insert(a, 0); // inserts with uniform distribution
                    }
                    action_map.insert(action, 1);
                    action_map
                });

            let mut child_board_cards_i = board_cards_i;
            let child_node_id = self.abstract_game.apply_action_to_node(node_id, &mut child_board_cards_i, action);
            self.update_strategy(child_node_id, board_cards, child_board_cards_i, hole_cards, player);

        } else {
            let actions = self.abstract_game.get_actions(&current_node.state);
            for action in actions {
                let mut child_board_cards_i = board_cards_i;
                let child_node_id = self.abstract_game.apply_action_to_node(node_id, &mut child_board_cards_i, action);
                self.update_strategy(child_node_id, board_cards, child_board_cards_i, hole_cards, player);
            }
        }

    }

    pub fn traverse_mccrfr(&mut self, node_id: NodeId, board_cards: &Vec<Card>, board_cards_i: usize, hole_cards: &[Vec<Card>; MAX_PLAYERS], player: PlayerId) -> i32 {
        let current_node = self.abstract_game.nodes.get_node(node_id).unwrap();

        debug!("traverse_mccfr at node {node_id}");

        if current_node.state.is_finished() {
            return current_node.state.get_payout(&self.abstract_game.game_info, &self.evaluator, &board_cards, &hole_cards, player);
        } else if current_node.state.has_folded(player) {
            //CHECK: this is what they do in paper return traverse_mccfr(h*0, P_i), but I think
            //this makes more sense
            return current_node.state.get_payout(&self.abstract_game.game_info, &self.evaluator, &board_cards, &hole_cards, player);
        } else if current_node.state.current_player().unwrap() == player {
            let bucket_id = self.abstract_game.get_bucket(current_node.state.current_round(), &board_cards, &hole_cards[player as usize]);
            let regrets = self.regrets.entry((node_id, bucket_id))
                .or_insert_with(|| {
                    let mut regrets_map: BTreeMap<Action, i32> = BTreeMap::new();
                    for a in self.abstract_game.get_actions(&current_node.state) {
                        regrets_map.insert(a, 0); // inserts with uniform distribution
                    }
                    regrets_map
                });
            let sigma = CFREngine::calculate_strategy(regrets);

            let mut v = 0.;
            let mut value_map: BTreeMap<Action, i32> = BTreeMap::new();

            let actions = self.abstract_game.get_actions(&current_node.state);
            for action in &actions {
                let mut child_board_cards_i = board_cards_i;
                let child_node_id = self.abstract_game.apply_action_to_node(node_id, &mut child_board_cards_i, *action);
                value_map.insert(*action, self.traverse_mccrfr(child_node_id, board_cards, child_board_cards_i, hole_cards, player));
                v += *sigma.get(action).unwrap_or(&0.) * (*value_map.get(action).unwrap() as f32);
            }
            let v = v.round() as i32;

            self.regrets.entry((node_id, bucket_id))
                .and_modify(|r| {
                    for action in actions {
                        r.entry(action).and_modify(|regret| {
                            *regret += *value_map.get(&action).unwrap_or(&0) - v;
                        });
                    }
                });

            return v;
        } else {
            let bucket_id = self.abstract_game.get_bucket(current_node.state.current_round(), &board_cards, &hole_cards[player as usize]);

            let regrets = self.regrets.entry((node_id, bucket_id))
                .or_insert_with(|| {
                    let mut regrets_map: BTreeMap<Action, i32> = BTreeMap::new();
                    for a in self.abstract_game.get_actions(&current_node.state) {
                        regrets_map.insert(a, 0); // inserts with uniform distribution
                    }
                    regrets_map
                });
            let sigma = CFREngine::calculate_strategy(regrets);
            let action = CFREngine::sample_strategy(&sigma);

            let mut child_board_cards_i = board_cards_i;
            let child_node_id = self.abstract_game.apply_action_to_node(node_id, &mut child_board_cards_i, action);
            return self.traverse_mccrfr(child_node_id, board_cards, child_board_cards_i, hole_cards, player);
        }
    }


    pub fn traverse_mccrfr_p(&mut self, node_id: NodeId, board_cards: &Vec<Card>, board_cards_i: usize, hole_cards: &[Vec<Card>; MAX_PLAYERS], player: PlayerId) -> i32 {
        let current_node = self.abstract_game.nodes.get_node(node_id).unwrap();

        debug!("traverse_mccfr_p at node {node_id}");

        if current_node.state.is_finished() {
            return current_node.state.get_payout(&self.abstract_game.game_info, &self.evaluator, &board_cards, &hole_cards, player);
        } else if current_node.state.has_folded(player) {
            //CHECK: this is what they do in paper return traverse_mccfr(h*0, P_i), but I think
            //this makes more sense
            return current_node.state.get_payout(&self.abstract_game.game_info, &self.evaluator, &board_cards, &hole_cards, player);
        } else if current_node.state.current_player().unwrap() == player {
            let bucket_id = self.abstract_game.get_bucket(current_node.state.current_round(), &board_cards, &hole_cards[player as usize]);
            let regrets = self.regrets.entry((node_id, bucket_id))
                .or_insert_with(|| {
                    let mut regrets_map: BTreeMap<Action, i32> = BTreeMap::new();
                    for a in self.abstract_game.get_actions(&current_node.state) {
                        regrets_map.insert(a, 0); // inserts with uniform distribution
                    }
                    regrets_map
                });
            let sigma = CFREngine::calculate_strategy(regrets);

            let mut v = 0.;
            let mut value_map: BTreeMap<Action, i32> = BTreeMap::new();

            // CHECK: way to do this without clone and the borrow rules working
            let regrets = regrets.clone();

            let actions = self.abstract_game.get_actions(&current_node.state);
            for action in &actions {
                if *regrets.get(action).unwrap_or(&0) > -300000000 {
                    let mut child_board_cards_i = board_cards_i;
                    let child_node_id = self.abstract_game.apply_action_to_node(node_id, &mut child_board_cards_i, *action);
                    value_map.insert(*action, self.traverse_mccrfr_p(child_node_id,board_cards, child_board_cards_i, hole_cards, player));
                    v += *sigma.get(action).unwrap_or(&0.) * (*value_map.get(action).unwrap() as f32);
                }
            }

            let v = v.round() as i32;

            self.regrets.entry((node_id, bucket_id))
                .and_modify(|r| {
                    for action in actions {
                        r.entry(action).and_modify(|regret| {
                            if let Some(x) = value_map.get(&action) {
                                *regret += x - v;
                            }
                        });
                    }
                });
            
            return v;
        } else {
            let bucket_id = self.abstract_game.get_bucket(current_node.state.current_round(), &board_cards, &hole_cards[player as usize]);

            let regrets = self.regrets.entry((node_id, bucket_id))
                .or_insert_with(|| {
                    let mut regrets_map: BTreeMap<Action, i32> = BTreeMap::new();
                    for a in self.abstract_game.get_actions(&current_node.state) {
                        regrets_map.insert(a, 0); // inserts with uniform distribution
                    }
                    regrets_map
                });
            let sigma = CFREngine::calculate_strategy(regrets);
            let action = CFREngine::sample_strategy(&sigma);

            let mut child_board_cards_i = board_cards_i;
            let child_node_id = self.abstract_game.apply_action_to_node(node_id, &mut child_board_cards_i, action);
            return self.traverse_mccrfr_p(child_node_id, board_cards, child_board_cards_i, hole_cards, player);
        }
    }
}
