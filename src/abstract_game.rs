use super::{
    action_abstraction::{ActionAbstraction},
    card_abstraction::CardAbstraction,
    game::{Action, GameInfo, GameState},
    node::{Node, NodeId},
};

use std::collections::BTreeMap;

pub struct AbstractGame {
    pub game_info: GameInfo,
    nodes: BTreeMap<NodeId, Node>,
    root: NodeId,
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
}
