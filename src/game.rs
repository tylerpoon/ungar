/*
* Somewhat port of https://github.com/ethansbrown/acpc
*/

use poker::{Card};

use serde::{Deserialize, Serialize};

use std::fs;

const MAX_PLAYERS: usize = 22;
const MAX_ROUNDS: usize = 4;
const MAX_NUM_ACTIONS: usize = 32;
const MAX_BOARD_CARDS: usize = 7;
const MAX_HOLE_CARDS: usize = 5;

/// Betting types of a poker game
#[derive(Debug, Deserialize, Serialize)]
pub enum BettingType {
    Limit,
    NoLimit,
}

/// Represents possible actions
#[derive(Copy, Clone, Debug)]
pub enum Action {
    Fold,
    Call,
    Raise(i32),
}

pub type ActionId = usize;

/// Represents the rules and parameters of a poker game
#[derive(Debug, Deserialize, Serialize)]
pub struct GameInfo {
    /// Starting stack for each player
    starting_stacks: Vec<u32>,
    /// Blinds per player
    blinds: Vec<u32>,
    /// Size of fixed raises per round for limit games
    raise_sizes: Vec<u32>,
    betting_type: BettingType,
    num_players: u8,
    num_rounds: u8,
    /// First player to act in a round
    first_player: Vec<u8>,
    num_suits: u8,
    num_ranks: u8,
    num_hole_cards: u8,
    num_board_cards: Vec<u8>,
}

impl GameInfo {
    pub fn load_game_info(path: &str) -> GameInfo {
        let game_info: GameInfo = serde_json::from_str(&fs::read_to_string(path).expect("failed to read game info")).expect("failed to deserialize game info");
        assert!(game_info.starting_stacks.len() as u8 == game_info.num_players);
        assert!(game_info.blinds.len() as u8 == game_info.num_players);
        assert!(game_info.raise_sizes.len() as u8 == game_info.num_rounds);
        assert!(game_info.first_player.len() as u8 == game_info.num_rounds);
        assert!(game_info.num_board_cards.len() as u8 == game_info.num_rounds);
        game_info
    }
}

/// Represents the state of a poker game
pub struct GameState {
    hand_id: u32,
    /// Largest bet over all rounds so far
    max_spent: u32,
    /// Minimum number of chips a player has to bet to raise in no limit games
    min_no_limit_raise_to: u32,
    /// Total amount put into pot by each player
    spent: [u32; MAX_PLAYERS],
    /// Stack of each player
    stack_player: [u32; MAX_PLAYERS],
    /// sumRoundSpent[r][p] gives amount in pot for round r of player p
    sum_round_spent: [[u32; MAX_PLAYERS]; MAX_ROUNDS], 
    /// action[r][i] gives the ith action in round r
    action: [[Action; MAX_NUM_ACTIONS]; MAX_ROUNDS],
    /// actingPlayer[r][i] gives the player who made ith action in round r
    acting_player: [[u8; MAX_NUM_ACTIONS]; MAX_ROUNDS],
    /// numActions[r] gives number of actions made in round r
    num_actions: [u8; MAX_ROUNDS],
    round: u8,
    finished: bool,
    /// Which players have folded
    players_folded: [bool; MAX_PLAYERS],
    board_cards: [Card; MAX_BOARD_CARDS],
    hole_cards: [[Card; MAX_HOLE_CARDS]; MAX_PLAYERS],
}

impl GameState {
    pub fn new(game_info: GameInfo) -> GameState {

    }
}

