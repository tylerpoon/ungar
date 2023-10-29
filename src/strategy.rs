//CHECK: very up for debate whether this hashmap focus is space efficient

use super::{
    abstract_game::AbstractGame,
    card_abstraction::BucketId,
    game::Action,
    node::NodeId,
};

use serde::{Serialize, Deserialize};

use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use rand::prelude::*;

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

    pub fn sample(&self, abstract_game: &AbstractGame, node_id: NodeId, bucket_id: BucketId) -> Action {
        let mut rng = rand::thread_rng();
        let sigma = match self.0.get(&(node_id, bucket_id)) {
            Some(s) => s.clone(),
            None => {
                let mut action_map: BTreeMap<Action, i32> = BTreeMap::new();

                let current_node = abstract_game.nodes.get_node(node_id).unwrap();
                for a in abstract_game.get_actions(&current_node.state) {
                    action_map.insert(a, 0); // inserts with uniform distribution
                }
                action_map
            },
        };
        println!("({}, {}): {:?}", node_id, bucket_id, sigma);

        *sigma.iter().collect::<Vec<(&Action, &i32)>>().choose_weighted(&mut rng, |item| item.1).unwrap().0
    }
}


pub type Regrets = BTreeMap<(NodeId, BucketId), BTreeMap<Action, i32>>;
