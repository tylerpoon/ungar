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

    pub fn calculate_strategy(regrets: BTreeMap<Action, i32>) -> BTreeMap<Action, f32> {
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

    pub fn update_strategy(&mut self, node_id: NodeId, board_cards: Vec<Card>, hole_cards: [Vec<Card>; MAX_PLAYERS], player: PlayerId) {
        let current_node = self.abstract_game.get_node(node_id).unwrap();

        if current_node.state.is_finished() || current_node.state.has_folded(player) || current_node.state.current_round() > 0 {
            return;
        } else if current_node.state.current_player().unwrap() == player {
            //TODO sampling actions and all that good stuff
        } else {
            let actions = self.abstract_game.get_actions(&current_node.state);
            for action in actions {
                match current_node.children.get(&action) {
                    Some(child_node_id) => {
                        //TODO make sure to handle adding cards, ie update board cards when
                        //necessary after round changes(might be easier to make some apply_action
                        //function just for this that updates cards as necessary, although kind of
                        //a clunky design)
                    },
                    None => {
                    }
                }
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
