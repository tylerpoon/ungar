//CHECK: very up for debate whether this hashmap focus is space efficient

use super::{
    card_abstraction::BucketId,
    game::Action,
    node::NodeId,
};

use serde::{Serialize, Deserialize};

use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize, Serialize)]
pub struct Strategy(pub BTreeMap<(NodeId, BucketId), BTreeMap<Action, i32>>);

impl Strategy {
    pub fn new() -> Strategy {
        Strategy(BTreeMap::new())
    }

    pub fn from_file(path: &Path) -> Strategy {
        let mut r = BufReader::new(File::open(path).unwrap());
        bincode::deserialize_from(&mut r).unwrap()
    }
}


pub type Regrets = BTreeMap<(NodeId, BucketId), BTreeMap<Action, i32>>;
