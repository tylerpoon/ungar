use super::{
    game::{Action, GameInfo, GameState},
};

use std::fs;

use serde::{Deserialize, Serialize};

/// Represents a possible abstract raise type
#[derive(Debug, Deserialize, Serialize)]
pub enum AbstractRaiseType {
    AllIn,
    PotRatio(f32),
    /// Usually just an option for limit games
    Fixed(i32),
}

/// Represents possible configurations for a raise on a particular round
#[derive(Debug, Deserialize, Serialize)]
pub enum RaiseRoundConfig {
    NotAllowed,
    Always,
    /// Only allowed before X many raises have been made
    Before(i32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AbstractRaise {
    raise_type: AbstractRaiseType,
    round_config: Vec<RaiseRoundConfig>,
}

/// Used to generate possible abstract actions for a given state
#[derive(Debug, Deserialize, Serialize)]
pub struct ActionAbstraction {
    possible_raises: Vec<AbstractRaise>,
}

impl ActionAbstraction {
    pub fn new(possible_raises: Vec<AbstractRaise>) -> ActionAbstraction {
        ActionAbstraction { possible_raises: possible_raises }
    }

    pub fn from_config(path: &str) -> ActionAbstraction {
        let action_abstraction: ActionAbstraction = serde_json::from_str(&fs::read_to_string(path).expect("failed to read action abstraction config")).expect("failed to deserialize action abstraction");
        action_abstraction
    }

    pub fn get_actions(&self, game_info: &GameInfo, game_state: &GameState) -> Vec<Action> {
        let actions: Vec<Actions> = Vec::new();
        //TODO

        actions
    }
}
