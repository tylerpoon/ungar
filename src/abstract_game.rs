use super::{
    action_abstraction::{ActionAbstraction},
    card_abstraction::CardAbstraction,
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

    pub fn get_node(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    pub fn get_actions(&self, game_state: &GameState) -> Vec<Action> {
        self.action_abstraction.get_actions(&self.game_info, game_state)
    }

    pub fn add_node(&mut self, node: Node) -> NodeId {
        let node_id = self.next_node_id;
        self.nodes.insert(self.next_node_id, node);

        self.next_node_id += 1;

        node_id
    }

    pub fn apply_action_to_node(&mut self, node_id: NodeId, board_cards: &mut Vec<Card>, hole_cards: &mut [Vec<Card>; MAX_PLAYERS], action: Action) -> NodeId {
        //TODO not sure if updating cards here is better than elsewhere

        let current_node = self.get_node(node_id).unwrap();
        let child = match current_node.children.get(&action) {
            Some(child_node_id) => {
                //TODO make sure to handle adding cards, ie update board cards when
                //necessary after round changes(might be easier to make some apply_action
                //function just for this that updates cards as necessary, although kind of
                //a clunky design)

                *child_node_id
            },
            None => {
                let new_node = Node::new(current_node.state.apply_action_no_cards(action));
                //TODO handle adding cards
                let node_id = self.add_node(new_node);
                node_id
            }
        };

        child
    }
}
