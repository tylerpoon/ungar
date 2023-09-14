use super::{
    abstract_game::AbstractGame,
    card_abstraction::BucketId,
    game::{Action, PlayerId, MAX_PLAYERS},
    strategy::{ Strategy, Regrets },
    node::NodeId,
};

use std::collections::BTreeMap;
use rand::Rng;

use poker::Card;

pub struct CFREngine {
    abstract_game: AbstractGame,
    strategy: Strategy,
    regrets: Regrets,
}

impl CFREngine {
    pub fn new(abstract_game: AbstractGame) -> CFREngine {
        CFREngine {
            abstract_game,
            strategy: Strategy::new(),
            regrets: Regrets::new(), 
        }
    }

    pub fn mccfr_p(&mut self, ticks: u32, strategy_interval: u32, prune_threshold: u32, lcfr_threshold: u32, discount_interval: u32) {
        //TODO: determine if this is needed with how this is programmed
        // for player in 0..self.abstract_game.game_info.num_players() {
        //     //initialize regrets to 0?
        // }
        
        let num_players = self.abstract_game.game_info.num_players();

        for t in 0..ticks {
            for i in 0..num_players {
                if t % strategy_interval == 0 {
                    //self.update_strategy();
                }
                if t > prune_threshold {
                    let mut rng = rand::thread_rng();
                    if rng.gen::<f32>() < 0.05 {
                        //self.traverse_mccrfr(node_id, board_cards, hole_cards, player);
                    } else {
                        //self.traverse_mccfr_p();
                    }
                } else {
                    //self.traverse_mccrfr(node_id, board_cards, hole_cards, player);
                }
            }

            if t < lcfr_threshold && t % discount_interval == 0 {
                let d: f32 = (t as f32 / discount_interval as f32) / ((t as f32 / discount_interval as f32) + 1.);

                //TODO: discount all regrets and strategies
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
        let length = strategy.len();
        for (k, v) in regrets.iter() {
            if sum > 0 {
                strategy.insert(*k, *v as f32 / sum as f32);
            } else {
                strategy.insert(*k, 1. / length as f32);
            }
        }

        strategy
    }

    fn sample_strategy(strategy: BTreeMap<Action, f32>) -> Action {
        let mut rng = rand::thread_rng();
        let num = rng.gen::<f32>();
        let mut total = 0.;

        for (k, v) in strategy.iter() {
            if total <= num && num <= total + *v {
                return *k;
            }
            total += *v;
        }

        Action::Fold
    }

    pub fn update_strategy(&mut self, node_id: NodeId, board_cards: Vec<Card>, hole_cards: &[Vec<Card>; MAX_PLAYERS], player: PlayerId) {
        let current_node = self.abstract_game.get_node(node_id).unwrap();

        if current_node.state.is_finished() || current_node.state.has_folded(player) || current_node.state.current_round() > 0 {
            return;
        } else if current_node.state.current_player().unwrap() == player {
            let bucket_id = self.abstract_game.get_bucket(current_node.state.current_round(), &board_cards, &hole_cards[player as usize]);
            let sigma = CFREngine::calculate_strategy(self.regrets.get_infoset(node_id, bucket_id).unwrap());
            let action = CFREngine::sample_strategy(sigma);

            // Add one to action counter
            self.strategy.entry_infoset(node_id, bucket_id)
                .and_modify(|s| { let _ = *s.entry(action).and_modify(|x| *x += 1).or_insert(0); })
                .or_insert_with(|| {
                    let mut action_map: BTreeMap<Action, i32> = BTreeMap::new();

                    for a in self.abstract_game.get_actions(&current_node.state) {
                        action_map.insert(a, 0); // inserts with uniform distribution
                    }
                    action_map.insert(action, 1);
                    action_map
                });

            let mut child_board_cards = board_cards.clone();
            let child_node_id = self.abstract_game.apply_action_to_node(node_id, &mut child_board_cards, hole_cards, action);
            self.update_strategy(child_node_id, child_board_cards, hole_cards, player);
     
        } else {
            let actions = self.abstract_game.get_actions(&current_node.state);
            for action in actions {
                let mut child_board_cards = board_cards.clone();
                let child_node_id = self.abstract_game.apply_action_to_node(node_id, &mut child_board_cards, hole_cards, action);
                self.update_strategy(child_node_id, child_board_cards, hole_cards, player);
            }
        }

    }

    pub fn traverse_mccrfr(&mut self, node_id: NodeId, board_cards: Vec<Card>, hole_cards: [Vec<Card>; MAX_PLAYERS], player: PlayerId) {
        // TODO: don't unwrap, actually handle errors properly
        let current_node = self.abstract_game.get_node(node_id).unwrap();

        if current_node.state.is_finished() {
            //return terminal value
        } else if current_node.state.has_folded(player) {
            //return traverse_mccfr(h*0, P_i)
        } else if current_node.state.current_player().unwrap() == player {
            //TODO
        } else {
            //TODO
        }
    }


    pub fn traverse_mccrfr_p(&mut self, node_id: NodeId, board_cards: Vec<Card>, hole_cards: [Vec<Card>; MAX_PLAYERS], player: PlayerId) {
        let current_node = self.abstract_game.get_node(node_id).unwrap();

        if current_node.state.is_finished() {
            //return reminal value
        } else if current_node.state.has_folded(player) {
            //return traverse_mccfr_p(h*0, P_i)
        } else if current_node.state.current_player().unwrap() == player {
            //TODO
        } else {
            //TODO
        }
    }
}
