use super::{
    game::{ ActionId, GameState },
};

use std::collections::BTreeMap;

pub type NodeId = usize;

pub struct Node {
    children: BTreeMap<ActionId, NodeId>,
    state: GameState,
}
