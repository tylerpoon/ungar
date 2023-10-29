use super::{
    game::{ Action, GameState },
};

use serde::{Serialize, Deserialize};

use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub type NodeId = usize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub children: BTreeMap<Action, NodeId>,
    pub state: GameState,
}

impl Node {
    pub fn new(state: GameState) -> Node {
        Node {
            children: BTreeMap::new(),
            state,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nodes {
    pub nodes_map: BTreeMap<NodeId, Node>,
    root: NodeId,
    next_node_id: NodeId,
}

impl Nodes {
    pub fn new(state: GameState) -> Nodes {
        let mut nodes = BTreeMap::new();
        let node = Node::new(state);
        nodes.insert(0, node);

        Nodes {
            nodes_map: nodes,
            root: 0,
            next_node_id: 1,
        }
    }

    pub fn from_file(path: &Path) -> Nodes {
        let mut r = BufReader::new(File::open(path).unwrap());
        bincode::deserialize_from(&mut r).unwrap()
    }

    pub fn save(&self, path: &Path) {
        let mut f = BufWriter::new(File::create(path).unwrap());
        bincode::serialize_into(&mut f, self).unwrap();
    }

    pub fn get_root_node_id(&self) -> NodeId {
        self.root
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes_map.get(&node_id)
    }

    pub fn add_node(&mut self, node: Node) -> NodeId {
        let node_id = self.next_node_id;
        self.nodes_map.insert(self.next_node_id, node);

        self.next_node_id += 1;

        node_id
    }
}
