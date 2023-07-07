use super::{
    abstract_game::AbstractGame,
    card_abstraction::BucketId,
    strategy::{ Strategy, Regrets },
    node::NodeId,
};

use poker::Card;

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

    pub fn solve(&mut self, node_id: NodeId, board_cards: Vec<Card>, hole_cards: Vec<Card>) {
        // TODO: don't unwrap, actually handle errors properly
        let current_node = self.abstract_game.get_node(node_id).unwrap();

        let round = current_node.state.current_round();
        let player = current_node.state.current_player();

        let bucket_id = self.abstract_game.card_abstraction.get_bucket(round, board_cards, hole_cards);

        let actions = self.abstract_game.get_actions(&current_node.state);

        let regret = match self.regrets.get_infoset(node_id, bucket_id) {
            Some(r) => r,
            None => { 
                self.regrets.add_infoset(node_id, bucket_id, actions);
                self.regrets.get_infoset(node_id, bucket_id).unwrap()
            },
        };

        //TODO from here 
    }
}
