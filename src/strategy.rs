//TODO: very up for debate whether this hashmap focus is space efficient

use super::{
    card_abstraction::BucketId,
    game::Action,
    node::NodeId,
};

use std::collections::BTreeMap;

pub struct Strategy {
    strategy_map: BTreeMap<(NodeId, BucketId), BTreeMap<Action, f32>>,
}

impl Strategy {
    pub fn new() -> Strategy {
        Strategy { strategy_map: BTreeMap::new() }
    }

    //TODO: maybe add is too ambigious name
    pub fn add_infoset(&mut self, node_id: NodeId, bucket_id: BucketId, actions: Vec<Action>) {
        let mut action_map: BTreeMap<Action, f32> = BTreeMap::new();
        let length: f32 = actions.len() as f32;

        for action in actions {
            action_map.insert(action, 1. / length); // inserts with uniform distribution
        }

        self.strategy_map.insert((node_id, bucket_id), action_map);
    }

    pub fn insert_infoset(&mut self, node_id: NodeId, bucket_id: BucketId, strategy: BTreeMap<Action, f32>) -> Option<BTreeMap<Action, f32>> {
        self.strategy_map.insert((node_id, bucket_id), strategy)
    }

    pub fn get_infoset(&self, node_id: NodeId, bucket_id: BucketId) -> Option<BTreeMap<Action, f32>> {
        self.strategy_map.get(&(node_id, bucket_id)).cloned()
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

    //TODO: maybe add is too ambigious name
    pub fn add_infoset(&mut self, node_id: NodeId, bucket_id: BucketId, actions: Vec<Action>) {
        let mut action_map: BTreeMap<Action, i32> = BTreeMap::new();

        for action in actions {
            action_map.insert(action, 0);
        }

        self.regrets.insert((node_id, bucket_id), action_map);
    }

    pub fn get_infoset(&self, node_id: NodeId, bucket_id: BucketId) -> Option<BTreeMap<Action, i32>> {
        self.regrets.get(&(node_id, bucket_id)).cloned()
    }
}
