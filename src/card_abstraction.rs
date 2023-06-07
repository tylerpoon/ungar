use super::game::GameInfo;

use poker::Card;

type BucketId = u32;

pub struct CardAbstraction {
    round_infosets: Vec<Box<dyn RoundBuckets>>,
}

impl CardAbstraction {
    pub fn new(round_infosets: Vec<Box<dyn RoundBuckets>>) -> CardAbstraction {
        CardAbstraction { round_infosets }
    }

    pub fn get_bucket(&self, round: u8, board_cards: Vec<Card>, hole_cards: Vec<Card>) -> BucketId {
        self.round_infosets[round as usize].get_bucket(board_cards, hole_cards)
    }
}

pub trait RoundBuckets {
    fn get_bucket(&self, board_cards: Vec<Card>, hole_cards: Vec<Card>) -> BucketId;
}

pub struct LosslessBuckets {
    num_suits: u8,
    num_ranks: u8,
    num_board_cards: u8,
    num_hole_cards: u8,
}

impl LosslessBuckets {
    pub fn new(game_info: &GameInfo, round: u8) -> LosslessBuckets {
        LosslessBuckets {
            num_suits: game_info.num_suits(),
            num_ranks: game_info.num_ranks(),
            num_board_cards: game_info.num_board_cards(round),
            num_hole_cards: game_info.num_hole_cards(), 
        }
    }
}

impl RoundBuckets for LosslessBuckets {
    fn get_bucket(&self, board_cards: Vec<Card>, hole_cards: Vec<Card>) -> BucketId {
        //TODO
        0
    }
}
