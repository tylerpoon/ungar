use super::game::GameInfo;

use poker::Card;

pub type BucketId = u32;

pub struct CardAbstraction {
    round_infosets: Vec<Box<dyn RoundBuckets>>,
}

impl CardAbstraction {
    pub fn new(round_infosets: Vec<Box<dyn RoundBuckets>>) -> CardAbstraction {
        CardAbstraction { round_infosets }
    }

    pub fn get_bucket(&self, round: u8, board_cards: &Vec<Card>, hole_cards: &Vec<Card>) -> BucketId {
        self.round_infosets[round as usize].get_bucket(board_cards, hole_cards)
    }
}

pub trait RoundBuckets {
    fn get_bucket(&self, board_cards: &Vec<Card>, hole_cards: &Vec<Card>) -> BucketId;
}

pub struct NoBuckets {
    num_suits: u8,
    num_ranks: u8,
    num_board_cards: u8,
    num_hole_cards: u8,
}

impl NoBuckets {
    pub fn new(game_info: &GameInfo, round: u8) -> NoBuckets {
        NoBuckets {
            num_suits: game_info.num_suits(),
            num_ranks: game_info.num_ranks(),
            num_board_cards: game_info.total_board_cards(round),
            num_hole_cards: game_info.num_hole_cards(), 
        }
    }
}

impl RoundBuckets for NoBuckets {
    fn get_bucket(&self, board_cards: &Vec<Card>, hole_cards: &Vec<Card>) -> BucketId {
        let mut bucket: BucketId = 0;
        for i in 0..self.num_hole_cards {
            if i > 0 {
                bucket *= self.num_suits as u32 * self.num_ranks as u32;
            }
            bucket += hole_cards[i as usize].rank() as u32 * self.num_suits as u32 + hole_cards[i as usize].suit() as u32;
        }

        for i in 0..self.num_board_cards {
            bucket *= self.num_suits as u32 * self.num_ranks as u32;
            bucket += board_cards[i as usize].rank() as u32 * self.num_suits as u32 + board_cards[i as usize].suit() as u32;
        }

        bucket
    }
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
            num_board_cards: game_info.total_board_cards(round),
            num_hole_cards: game_info.num_hole_cards(), 
        }
    }
}

impl RoundBuckets for LosslessBuckets {
    fn get_bucket(&self, board_cards: &Vec<Card>, hole_cards: &Vec<Card>) -> BucketId {
        //TODO: implement lossless(suit isomprhims etc) abstraction, look at http://www.kevinwaugh.com/pdf/isomorphism13.pdf
        0
    }
}
