use super::{
    action_abstraction::{ActionAbstraction},
    card_abstraction::CardAbstraction,
    game::{GameInfo, GameState},
    node::{Node, NodeId},
};

use std::collections::BTreeMap;

pub struct AbstractGame {
    game_info: GameInfo,
    nodes: BTreeMap<NodeId, Node>,
    root: NodeId,
    action_abstraction: ActionAbstraction,
    card_abstraction: CardAbstraction,
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
}
