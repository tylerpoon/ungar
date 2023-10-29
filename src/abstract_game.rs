use super::{
    action_abstraction::{ActionAbstraction},
    card_abstraction::{BucketId, CardAbstraction},
    game::{Action, GameInfo, GameState, MAX_PLAYERS},
    node::{Nodes, Node, NodeId},
};

use poker::Card;

use std::path::Path;

pub struct AbstractGame {
    pub game_info: GameInfo,
    pub nodes: Nodes,
    pub action_abstraction: ActionAbstraction,
    pub card_abstraction: CardAbstraction,
}

impl AbstractGame {
    pub fn new(game_info: GameInfo, state: GameState, action_abstraction: ActionAbstraction, card_abstraction: CardAbstraction) -> AbstractGame {
        AbstractGame {
            game_info,
            nodes: Nodes::new(state),
            action_abstraction,
            card_abstraction,
        }
    }

    pub fn load_nodes(game_info: GameInfo, path: &Path, action_abstraction: ActionAbstraction, card_abstraction: CardAbstraction) -> AbstractGame {
        AbstractGame {
            game_info,
            nodes: Nodes::from_file(path),
            action_abstraction,
            card_abstraction,
        }
    }

    pub fn get_actions(&self, game_state: &GameState) -> Vec<Action> {
        self.action_abstraction.get_actions(&self.game_info, game_state)
    }

    pub fn get_bucket(&self, round:u8, board_cards: &Vec<Card>, hole_cards: &Vec<Card>) -> BucketId {
        self.card_abstraction.get_bucket(round, board_cards, hole_cards)
    }

    pub fn apply_action_to_node(&mut self, node_id: NodeId, board_cards_i: &mut usize, action: Action) -> NodeId {
        //TODO: deal with nolimit situations where actions get rounded or make new action in tree
        let current_node = self.nodes.get_node(node_id).unwrap();
        let child = match current_node.children.get(&action) {
            Some(child_node_id) => {
                *board_cards_i = self.game_info.total_board_cards(self.nodes.get_node(*child_node_id).unwrap().state.current_round()) as usize;
                
                *child_node_id
            },
            None => {
                let new_node = Node::new(current_node.state.apply_action_no_cards(&self.game_info, action).unwrap());

                *board_cards_i = self.game_info.total_board_cards(new_node.state.current_round()) as usize;

                let child_node_id = self.nodes.add_node(new_node);
                self.nodes.nodes_map.get_mut(&node_id).unwrap().children.insert(action, child_node_id);
                child_node_id
            }
        };

        child
    }
}
