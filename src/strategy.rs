//CHECK: very up for debate whether this hashmap focus is space efficient

use super::{
    card_abstraction::BucketId,
    game::Action,
    node::NodeId,
};

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

pub struct Strategy {
    strategy_map: BTreeMap<(NodeId, BucketId), BTreeMap<Action, i32>>,
}

impl Strategy {
    pub fn new() -> Strategy {
        Strategy { strategy_map: BTreeMap::new() }
    }

    //CHECK: maybe add is too ambigious name
    pub fn add_infoset(&mut self, node_id: NodeId, bucket_id: BucketId, actions: Vec<Action>) {
        let mut action_map: BTreeMap<Action, i32> = BTreeMap::new();

        for action in actions {
            action_map.insert(action, 0); // inserts with uniform distribution
        }

        self.strategy_map.insert((node_id, bucket_id), action_map);
    }

    pub fn insert_infoset(&mut self, node_id: NodeId, bucket_id: BucketId, strategy: BTreeMap<Action, i32>) -> Option<BTreeMap<Action, i32>> {
        self.strategy_map.insert((node_id, bucket_id), strategy)
    }

    pub fn get_infoset(&self, node_id: NodeId, bucket_id: BucketId) -> Option<&BTreeMap<Action, i32>> {
        self.strategy_map.get(&(node_id, bucket_id))
    }

    pub fn get_mut_infoset(&mut self, node_id: NodeId, bucket_id: BucketId) -> Option<&mut BTreeMap<Action, i32>> {
        self.strategy_map.get_mut(&(node_id, bucket_id))
    }

    pub fn entry_infoset(&mut self, node_id: NodeId, bucket_id: BucketId) -> Entry<(NodeId, BucketId), BTreeMap<Action, i32>> {
        self.strategy_map.entry((node_id, bucket_id))
    }
}

pub struct Regrets {
    /// By pluribus, regret floor should be -310,000,000
    regrets: BTreeMap<(NodeId, BucketId), BTreeMap<Action, i32>>,
}


impl Regrets {
    pub fn new() -> Regrets {
        Regrets { regrets: BTreeMap::new() }
    }

    //CHECK: maybe add is too ambigious name
    pub fn add_infoset(&mut self, node_id: NodeId, bucket_id: BucketId, actions: Vec<Action>) {
        let mut action_map: BTreeMap<Action, i32> = BTreeMap::new();

        for action in actions {
            action_map.insert(action, 0);
        }

        self.regrets.insert((node_id, bucket_id), action_map);
    }

    pub fn get_infoset(&self, node_id: NodeId, bucket_id: BucketId) -> Option<&BTreeMap<Action, i32>> {
        self.regrets.get(&(node_id, bucket_id))
    }
}
