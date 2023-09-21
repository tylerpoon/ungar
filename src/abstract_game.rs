use super::{
    action_abstraction::{ActionAbstraction},
    card_abstraction::{BucketId, CardAbstraction},
    card_util::deal_without,
    game::{Action, GameInfo, GameState, MAX_PLAYERS},
    node::{Node, NodeId},
};

use poker::Card;

use std::collections::BTreeMap;

pub struct AbstractGame {
    pub game_info: GameInfo,
    nodes: BTreeMap<NodeId, Node>,
    root: NodeId,
    next_node_id: NodeId,
    pub action_abstraction: ActionAbstraction,
    pub card_abstraction: CardAbstraction,
}

impl AbstractGame {
    pub fn new(game_info: GameInfo, state: GameState, action_abstraction: ActionAbstraction, card_abstraction: CardAbstraction) -> AbstractGame {
        let mut nodes = BTreeMap::new();
        let node = Node::new(state);
        nodes.insert(0, node);

        AbstractGame {
            game_info,
            nodes,
            root: 0,
            next_node_id: 1,
            action_abstraction,
            card_abstraction,
        }
    }

    pub fn get_root_node_id(&self) -> NodeId {
        self.root
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    pub fn get_actions(&self, game_state: &GameState) -> Vec<Action> {
        self.action_abstraction.get_actions(&self.game_info, game_state)
    }

    pub fn get_bucket(&self, round:u8, board_cards: &Vec<Card>, hole_cards: &Vec<Card>) -> BucketId {
        self.card_abstraction.get_bucket(round, board_cards, hole_cards)
    }

    pub fn add_node(&mut self, node: Node) -> NodeId {
        let node_id = self.next_node_id;
        self.nodes.insert(self.next_node_id, node);

        self.next_node_id += 1;

        node_id
    }

    pub fn apply_action_to_node(&mut self, node_id: NodeId, board_cards: &mut Vec<Card>, hole_cards: &[Vec<Card>; MAX_PLAYERS], action: Action) -> NodeId {
        //TODO: not sure if updating cards here is better than elsewhere. in fact, might be better
        //to generate ALL board cards in advance, and then just control access based on round, in
        //this way extra clones of board cards are avoided.

        let current_node = self.nodes.get(&node_id).unwrap();
        let child = match current_node.children.get(&action) {
            Some(child_node_id) => {
                let num_new_cards = self.game_info.total_board_cards(self.nodes.get(child_node_id).unwrap().state.current_round()) - board_cards.len() as u8;
                
                if num_new_cards > 0 {
                    let dealt = [board_cards.clone(), hole_cards.concat()].concat();
                    let mut new_board_cards = deal_without(num_new_cards as usize, &dealt);
                    board_cards.append(&mut new_board_cards);
                }

                *child_node_id
            },
            None => {
                let new_node = Node::new(current_node.state.apply_action_no_cards(&self.game_info, action).unwrap());

                let num_new_cards = self.game_info.total_board_cards(new_node.state.current_round()) - board_cards.len() as u8;

                if num_new_cards > 0 {
                    let dealt = [board_cards.clone(), hole_cards.concat()].concat();
                    let mut new_board_cards = deal_without(num_new_cards as usize, &dealt);
                    board_cards.append(&mut new_board_cards);
                }
                let child_node_id = self.add_node(new_node);
                self.nodes.get_mut(&node_id).unwrap().children.insert(action, child_node_id);
                child_node_id
            }
        };

        child
    }
}
