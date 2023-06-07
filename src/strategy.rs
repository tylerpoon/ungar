//TODO: very up for debate whether this hashmap focus is space efficient

use super::{
    card_abstraction::BucketId,
    game::Action,
    node::NodeId,
};

use std::collections::BTreeMap;

pub struct Strategy {
    strategy_map: BTreeMap<(NodeId, BucketId), BTreeMap<Action, u32>>,
}

impl Strategy {
    pub fn new() -> Strategy {
        Strategy { strategy_map: BTreeMap::new() }
    }

    pub fn add_infoset(&mut self, node_id: NodeId, bucket_id: BucketId, actions: Vec<Action>) {
        let mut action_map: BTreeMap<Action, u32> = BTreeMap::new();

        for action in actions {
            action_map.insert(action, 1); // inserts with uniform distribution
        }

        self.strategy_map.insert((node_id, bucket_id), action_map);
    }

    pub fn get_infoset(&self, node_id: NodeId, bucket_id: BucketId) -> Option<BTreeMap<Action, f32>> {
        let action_map = match self.strategy_map.get(&(node_id, bucket_id)) {
            Some(action_map) => { action_map },
            None =>  return None,
        };

        let mut action_to_float_map: BTreeMap<Action, f32> = BTreeMap::new();

        let mut total: u32 = 0;
        for (_, v) in action_map {
           total += v; 
        }

        for (k, v) in action_map {
            action_to_float_map.insert(*k, *v as f32 / total as f32); 
        }
        
        Some(action_to_float_map)
    }
}
