use super::{
    game::{ ActionId, GameState },
};

use std::collections::BTreeMap;

pub type NodeId = usize;

pub struct Node {
    children: BTreeMap<ActionId, NodeId>,
    state: GameState,
}

impl Node {
    pub fn new(state: GameState) -> Node {
        Node {
            children: BTreeMap::new(),
            state: state
        }
    }
}
