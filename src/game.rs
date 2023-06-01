/*
* Somewhat port of https://github.com/ethansbrown/acpc
*/

use poker::{Card};

const MAX_PLAYERS: usize = 22;
const MAX_ROUNDS: usize = 4;
const MAX_NUM_ACTIONS: usize = 32;
const MAX_BOARD_CARDS: usize = 7;
const MAX_HOLE_CARDS: usize = 5;

/// Betting types of a poker game
pub enum BettingType {
    Limit,
    NoLimit,
}

/// Represents possible actions
#[derive(Copy, Clone, Debug)]
pub enum Action {
    FOLD,
    CALL,
    RAISE(i32),
}

pub type ActionId = usize;

/// Represents the rules and parameters of a poker game
pub struct GameInfo {
    /// Starting stack for each player
    starting_stacks: Vec<u32>,
    /// Blinds per player
    blind: Vec<u32>,
    /// Size of fixed raises per round for limit games
    raiseSize: Vec<u32>,
    bettingType: BettingType,
    numPlayers: u8,
    numRounds: u8,
    /// First player to act in a round
    firstPlayer: Vec<u8>,
    numSuits: u8,
    numRanks: u8,
    numHoleCards: u8,
    numBoardCards: Vec<u8>,
}

/// Represents the state of a poker game
pub struct GameState {
    handId: u32,
    /// Largest bet over all rounds so far
    maxSpent: u32,
    /// Minimum number of chips a player has to bet to raise in no limit games
    minNoLimitRaiseTo: u32,
    /// Total amount put into pot by each player
    spent: [u32; MAX_PLAYERS],
    /// Stack of each player
    stackPlayer: [u32; MAX_PLAYERS],
    /// sumRoundSpent[r][p] gives amount in pot for round r of player p
    sumRoundSpent: [[u32; MAX_PLAYERS]; MAX_ROUNDS], 
    /// action[r][i] gives the ith action in round r
    action: [[Action; MAX_NUM_ACTIONS]; MAX_ROUNDS],
    /// actingPlayer[r][i] gives the player who made ith action in round r
    actingPlayer: [[u8; MAX_NUM_ACTIONS]; MAX_ROUNDS],
    /// numActions[r] gives number of actions made in round r
    numActions: [u8; MAX_ROUNDS],
    round: u8,
    finished: bool,
    /// Which players have folded
    playerFolded: [bool; MAX_PLAYERS],
    boardCards: [Card; MAX_BOARD_CARDS],
    holeCards: [[Card; MAX_HOLE_CARDS]; MAX_PLAYERS],
}

