use super::{
    game::{GameInfo, GameState},
    node::{Node, NodeId},
};

use std::collections::BTreeMap;

pub struct AbstractGame {
    gameinfo: GameInfo,
    state: GameState,
    nodes: BTreeMap<NodeId, Node>,
    root: NodeId,
}
