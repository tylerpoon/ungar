//CHECK: very up for debate whether this hashmap focus is space efficient

use super::{
    card_abstraction::BucketId,
    game::Action,
    node::NodeId,
};

use std::collections::BTreeMap;

pub type Strategy = BTreeMap<(NodeId, BucketId), BTreeMap<Action, i32>>;

pub type Regrets = BTreeMap<(NodeId, BucketId), BTreeMap<Action, i32>>;
