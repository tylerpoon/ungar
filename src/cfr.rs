use super::{
    abstract_game::AbstractGame,
    card_abstraction::BucketId,
    strategy::{ Strategy, Regrets },
    node::NodeId,
};

pub struct CFREngine {
    abstract_game: AbstractGame,
    strategy: Strategy,
    regrets: Regrets,
}

impl CFREngine {
    pub fn new(abstract_game: AbstractGame) -> CFREngine {
        CFREngine {
            abstract_game,
            strategy: Strategy::new(),
            regrets: Regrets::new(), 
        }
    }

    pub fn solve(&mut self, node_id: NodeId, bucket_id: BucketId) {
        //TODO: implement MCCFR here
    }
}
