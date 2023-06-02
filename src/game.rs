/*
* Somewhat port of https://github.com/ethansbrown/acpc
*/

use poker::{Card};

use serde::{Deserialize, Serialize};

use std::fs;
use std::option::Option;

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

pub type PlayerId = u8;

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
    num_players: PlayerId,
    num_rounds: u8,
    /// Max amount of raises per round
    max_raises: Vec<u8>,
    /// First player to act in a round
    first_player: Vec<PlayerId>,
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
        assert!(game_info.max_raises.len() as u8 == game_info.num_rounds);
        assert!(game_info.first_player.len() as u8 == game_info.num_rounds);
        assert!(game_info.num_board_cards.len() as u8 == game_info.num_rounds);
        game_info
    }
}

/// Represents the state of a poker game
#[derive(Debug)]
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
    /// sum_round_spent[r][p] gives amount in pot for round r of player p
    sum_round_spent: [[u32; MAX_PLAYERS]; MAX_ROUNDS], 
    /// action[r][i] gives the ith action in round r
    action: [[Option<Action>; MAX_NUM_ACTIONS]; MAX_ROUNDS],
    /// acting_player[r][i] gives the player who made ith action in round r
    acting_player: [[u8; MAX_NUM_ACTIONS]; MAX_ROUNDS],
    /// num_actions[r] gives number of actions made in round r
    num_actions: [u8; MAX_ROUNDS],
    round: u8,
    finished: bool,
    /// Which players have folded
    players_folded: [bool; MAX_PLAYERS],
    board_cards: [Option<Card>; MAX_BOARD_CARDS],
    hole_cards: [[Option<Card>; MAX_HOLE_CARDS]; MAX_PLAYERS],
}

impl GameState {
    pub fn new(game_info: &GameInfo, hand_id: u32) -> GameState {
        let mut sum_round_spent: [[u32; MAX_PLAYERS]; MAX_ROUNDS]  = [[0; MAX_PLAYERS]; MAX_ROUNDS];
        let mut spent = [0; MAX_PLAYERS];
        let mut max_spent: u32 = 0;
        let mut players_folded: [bool; MAX_PLAYERS] = [true; MAX_PLAYERS];

        for i in 0..game_info.num_players {
            spent[i as usize] = game_info.blinds[i as usize];
            sum_round_spent[0][i as usize] = game_info.blinds[i as usize];

            if game_info.blinds[i as usize] > max_spent {
                max_spent = game_info.blinds[i as usize];
            }
            players_folded[i as usize] = false;
        }

        let min_no_limit_raise_to = match &game_info.betting_type {
            BettingType::NoLimit if max_spent > 0 => max_spent * 2,
            BettingType::NoLimit => 1,
            BettingType::Limit => 0,
        };

        let mut stack_player: [u32; MAX_PLAYERS] = [0; MAX_PLAYERS];
        for (i, s) in game_info.starting_stacks.iter().enumerate() {
            stack_player[i] = *s;
        }

        GameState {
            hand_id: hand_id,
            max_spent: max_spent,
            min_no_limit_raise_to: min_no_limit_raise_to,
            spent: spent,
            stack_player: stack_player,
            sum_round_spent: sum_round_spent,
            action: [[None; MAX_NUM_ACTIONS]; MAX_ROUNDS],
            acting_player: [[0; MAX_NUM_ACTIONS]; MAX_ROUNDS],
            num_actions: [0; MAX_ROUNDS],
            round: 0,
            finished: false,
            players_folded: players_folded,
            board_cards: [None; MAX_BOARD_CARDS],
            hole_cards: [[None; MAX_HOLE_CARDS]; MAX_PLAYERS],
        }
    }

    pub fn current_player(&self) -> Result<PlayerId, &'static str> {
        if self.finished {
            return Err("state is finished so there is no active player");
        }

        //TODO(should we track or do similar to acpc?)
    }
}

