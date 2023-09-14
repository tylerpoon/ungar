/*
* Somewhat port of https://github.com/ethansbrown/acpc
*/

use log::warn;

use super::action_abstraction::{
    AbstractRaise, AbstractRaiseType, RaiseRoundConfig
};

use poker::Card;

use serde::{Deserialize, Serialize};

use std::fs;
use std::option::Option;

pub const MAX_PLAYERS: usize = 22;
pub const MAX_ROUNDS: usize = 4;
pub const MAX_NUM_ACTIONS: usize = 32;
pub const MAX_BOARD_CARDS: usize = 7;
pub const MAX_HOLE_CARDS: usize = 5;

/// Betting types of a poker game
#[derive(Debug, Deserialize, Serialize)]
pub enum BettingType {
    Limit,
    NoLimit,
}

/// Represents possible actions
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Action {
    Fold,
    Call,
    Raise(u32),
}

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
    /// Board cards added each round
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

    pub fn num_suits(&self) -> u8 {
        self.num_suits
    }

    pub fn num_ranks(&self) -> u8 {
        self.num_ranks
    }

    pub fn num_hole_cards(&self) -> u8 {
        self.num_hole_cards
    }

    pub fn num_players(&self) -> PlayerId {
        self.num_players
    }

    pub fn num_board_cards(&self, round: u8) -> u8 {
        self.num_board_cards[round as usize]
    }

    pub fn total_board_cards(&self, round: u8) -> u8 {
        let mut total = 0;
        for i in 0..round {
            total += self.num_board_cards[i as usize];
        }
        total
    }
}

/// Represents the state of a poker game
#[derive(Clone, Debug)]
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
    acting_player: [[PlayerId; MAX_NUM_ACTIONS]; MAX_ROUNDS],
    /// Player who is currently active
    active_player: PlayerId,
    /// num_actions[r] gives number of actions made in round r
    num_actions: [u8; MAX_ROUNDS],
    round: u8,
    finished: bool,
    /// Which players have folded
    players_folded: [bool; MAX_PLAYERS],
    board_cards: Vec<Card>,
    hole_cards: [Vec<Card>; MAX_PLAYERS],
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
            hand_id,
            max_spent,
            min_no_limit_raise_to,
            spent,
            stack_player,
            sum_round_spent,
            action: [[None; MAX_NUM_ACTIONS]; MAX_ROUNDS],
            acting_player: [[0; MAX_NUM_ACTIONS]; MAX_ROUNDS],
            active_player: game_info.first_player[0],
            num_actions: [0; MAX_ROUNDS],
            round: 0,
            finished: false,
            players_folded,
            board_cards: Vec::new(),
            hole_cards: [(); MAX_PLAYERS].map(|_| Vec::new()),
        }
    }

    pub fn current_round(&self) -> u8 {
        self.round
    }
    
    /// Returns current player
    pub fn current_player(&self) -> Result<PlayerId, &'static str> {
        if self.finished {
            return Err("state is finished so there is no active player");
        }

        Ok(self.active_player)
    }

    /// Returns players who can still take actions
    pub fn num_active_players(&self, game_info: &GameInfo) -> u8 {
        let mut count = 0;
        for i in 0..game_info.num_players {
            if !self.players_folded[i as usize] && self.spent[i as usize] < self.stack_player[i as usize] {
                count += 1;
            }
        }

        count
    }

    /// Returns players who have called
    pub fn num_called(&self, game_info: &GameInfo) -> u8 {
        let mut count = 0;

        for i in (0..self.num_actions[self.round as usize]).rev() {
            let player = self.acting_player[self.round as usize][i as usize];

            if matches!(self.action[self.round as usize][i as usize].unwrap(), Action::Raise(_)) {
                if self.spent[player as usize] < self.stack_player[player as usize] {
                    count += 1;
                }

                return count;
            } else if self.action[self.round as usize][i as usize].unwrap() == Action::Call {
                if self.spent[player as usize] < self.stack_player[player as usize] {
                    count += 1;
                }
            }
        }

        count
    }

    /// Returns players who have folded
    pub fn num_folded(&self, game_info: &GameInfo) -> u8 {
        let mut count = 0;
        for i in 0..game_info.num_players() {
            if self.has_folded(i) {
                count += 1;
            }
        }

        count
    }

    /// Returns next player after active_player
    fn next_player(&self, game_info: &GameInfo) -> Result<PlayerId, &'static str> {
        if self.finished {
            return Err("state is finished so there is no active player");
        }

        let mut p = self.active_player;

        loop {
            p = (p + 1) % game_info.num_players;

            if !self.players_folded[p as usize] && self.spent[p as usize] < self.stack_player[p as usize] {
                break;
            }
        }

        Ok(p)
    }

    /// Returns if state is finished(ie terminal state)
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Returns if player has folded
    pub fn has_folded(&self, player: PlayerId) -> bool {
        self.players_folded[player as usize]
    }

    /// Returns number of raises made in this round
    pub fn num_raises(&self) -> u8 {
        let mut count: u8 = 0;
        for i in 0..self.num_actions[self.round as usize] {
            if let Some(Action::Raise(_)) = self.action[self.round as usize][i as usize] {
                count += 1;
            }
        }
        count
    }

    fn raise_range(&self, game_info: &GameInfo) -> (u32, u32) {
        if self.finished {
            return (0, 0);
        }

        if self.num_raises() >= game_info.max_raises[self.round as usize] {
            return (0, 0);
        }

        // TODO: might be worth figuring out a way to allow infinite actions(need to do it
        // without sacrificing efficiency too much)
        if (self.num_actions[self.round as usize] + game_info.num_players) as usize > MAX_NUM_ACTIONS {
            warn!("Making raise invalid since possible actions {} > MAX_NUM_ACTIONS", self.num_actions[self.round as usize] + game_info.num_players);
            return (0, 0);
        }

        if self.num_active_players(game_info) <= 1 {
            return (0, 0);
        }


        match game_info.betting_type {
            BettingType::Limit => {
                warn!("raise_range called with limit betting type!");
                (0, 0)
            }, // TODO: maybe change this here
            BettingType::NoLimit => {
                let mut min_raise = self.min_no_limit_raise_to;
                let max_raise = self.stack_player[self.active_player as usize];
                if self.stack_player[self.active_player as usize] < self.min_no_limit_raise_to {
                    if self.max_spent >= self.stack_player[self.active_player as usize] {
                        return (0, 0);
                    } else {
                        min_raise = max_raise;
                    }
                }

                (min_raise, max_raise)
            }
        }

    }

    pub fn is_valid_action(&self, game_info: &GameInfo, action: Action) -> bool{
        if self.finished {
            return false;
        }

        match action {
            Action::Fold => {
                // TODO: determine whether to consider premature folding(ie folding when all bets
                // are called) a "valid" action, right now only prevents folding when all-in
                if self.spent[self.active_player as usize] == self.stack_player[self.active_player as usize] {
                    return false;
                }

                true
            },
            Action::Call => true,
            Action::Raise(r) => {
                match game_info.betting_type {
                    BettingType::Limit => r == game_info.raise_sizes[self.round as usize],
                    BettingType::NoLimit => {
                        let (min_raise, max_raise) = self.raise_range(game_info);
                        r >= min_raise && r <= max_raise
                    }
                }
            },
        }
    }
    
    /// Converts abstract raise to a real raise if it is valid
    pub fn abstract_raise_to_real(&self, game_info: &GameInfo, abstract_raise: &AbstractRaise) -> Option<Action> {
        match abstract_raise.round_config[self.round as usize] {
            RaiseRoundConfig::Always => {},
            RaiseRoundConfig::Before(i) if i > self.num_raises() as u32 => {},
            _ => return None,
        }

        let raise = match abstract_raise.raise_type {
            AbstractRaiseType::AllIn => Action::Raise(self.stack_player[self.active_player as usize]),
            AbstractRaiseType::Fixed(i) => {
                match game_info.betting_type {
                    BettingType::NoLimit => Action::Raise(self.max_spent + i),
                    BettingType::Limit => Action::Raise(i)
                }
            },
            //TODO: Check below is correct
            AbstractRaiseType::PotRatio(r) => Action::Raise((self.max_spent as f32 * r) as u32),
        };

        if self.is_valid_action(game_info, raise) {
            return Some(raise);
        }
        
        None
    }
    
    /// Returns a new state with that action applied, DOES NOT update cards(this may be something
    /// that gets refactored later).
    pub fn apply_action_no_cards(&self, game_info: &GameInfo, action: Action) -> Result<GameState, &'static str> {
        let mut new_state = self.clone();

        if self.is_finished() {
            return Err("cannot apply action to finished state");
        }

        if self.num_actions[self.round as usize] >= MAX_NUM_ACTIONS as u8 {
            return Err("cannot apply action to state: already at max actions for this round");
        }

        if self.is_valid_action(game_info, action) == false {
            return Err("cannot apply an invalid action");
        }

        let player = self.current_player().unwrap();

        new_state.action[self.round as usize][self.num_actions[self.round as usize] as usize] = Some(action);
        new_state.acting_player[self.round as usize][self.num_actions[self.round as usize] as usize] = player;
        new_state.num_actions[self.round as usize] += 1;

        match action {
            Action::Fold => {
                new_state.players_folded[player as usize] = true;
            },
            Action::Call => {
                if new_state.max_spent > new_state.stack_player[player as usize] {
                    new_state.spent[player as usize] = new_state.stack_player[player as usize];
                    new_state.sum_round_spent[self.round as usize][player as usize] = new_state.stack_player[player as usize];
                } else {
                    new_state.spent[player as usize] = new_state.max_spent;
                    new_state.sum_round_spent[self.round as usize][player as usize] = new_state.max_spent;
                }
            },
            Action::Raise(r) => {
                match game_info.betting_type {
                    BettingType::NoLimit => {
                        if r * 2 - new_state.max_spent > new_state.min_no_limit_raise_to {
                            new_state.min_no_limit_raise_to = r * 2 - new_state.max_spent;
                        }
                        new_state.max_spent = r;
                    },
                    BettingType::Limit => {
                        if new_state.max_spent + game_info.raise_sizes[new_state.round as usize] > new_state.stack_player[player as usize] {
                            new_state.max_spent = new_state.stack_player[player as usize];
                        } else {
                            new_state.max_spent += game_info.raise_sizes[new_state.round as usize];
                        }
                    },
                };

                new_state.spent[player as usize] = new_state.max_spent;
                new_state.sum_round_spent[new_state.round as usize][player as usize] = new_state.max_spent;
            }
        };

        //TODO: change rounds etc

        if new_state.num_folded(game_info) + 1>= game_info.num_players() {
            new_state.finished = true;
        } else if new_state.num_called(game_info) >= new_state.num_active_players(game_info) {
            if new_state.num_active_players(game_info) > 1 {
                if new_state.round + 1 > game_info.num_rounds {
                    new_state.round += 1;
                    new_state.min_no_limit_raise_to = 1;
                    for i in 0..game_info.num_players() {
                        if game_info.blinds[i as usize] > new_state.min_no_limit_raise_to {
                            new_state.min_no_limit_raise_to = game_info.blinds[i as usize];
                        }
                    }
                    new_state.min_no_limit_raise_to += new_state.max_spent;
                } else {
                    new_state.finished = true;
                }
            } else {
                // skip to showdown
                new_state.finished = true;
                new_state.round = game_info.num_rounds - 1;
            }
        }

        Ok(new_state)
    }
}

