use super::{
    game::{GameInfo, GameState},
    node::{Node, NodeId},
};

use std::collections::BTreeMap;

pub struct AbstractGame {
    game_info: GameInfo,
    nodes: BTreeMap<NodeId, Node>,
    root: NodeId,
}

impl AbstractGame {
    pub fn new(game_info: GameInfo, state: GameState) -> AbstractGame {
        let mut nodes = BTreeMap::new();
        let node = Node::new(state);
        nodes.insert(0, node);

        AbstractGame {
            game_info: game_info,
            nodes: nodes,
            root: 0,
        }
    }
}
