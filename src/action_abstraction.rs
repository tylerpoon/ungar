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
    Fixed(u32),
}

/// Represents possible configurations for a raise on a particular round
#[derive(Debug, Deserialize, Serialize)]
pub enum RaiseRoundConfig {
    NotAllowed,
    Always,
    /// Only allowed before X many raises have been made
    Before(u32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AbstractRaise {
    pub raise_type: AbstractRaiseType,
    pub round_config: Vec<RaiseRoundConfig>,
}

/// Used to generate possible abstract actions for a given state
#[derive(Debug, Deserialize, Serialize)]
pub struct ActionAbstraction {
    possible_raises: Vec<AbstractRaise>,
}

impl ActionAbstraction {
    pub fn new(possible_raises: Vec<AbstractRaise>) -> ActionAbstraction {
        ActionAbstraction { possible_raises }
    }

    pub fn from_config(path: &str) -> ActionAbstraction {
        let action_abstraction: ActionAbstraction = serde_json::from_str(&fs::read_to_string(path).expect("failed to read action abstraction config")).expect("failed to deserialize action abstraction");
        action_abstraction
    }

    pub fn get_actions(&self, game_info: &GameInfo, game_state: &GameState) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        if game_state.is_valid_action(game_info, Action::Fold) {
            actions.push(Action::Fold);
        }

        if game_state.is_valid_action(game_info, Action::Call) {
            actions.push(Action::Call);
        }

        for abstract_raise in &self.possible_raises {
            match game_state.abstract_raise_to_real(game_info, abstract_raise) {
                Some(raise) => {
                    if !actions.contains(&raise) {
                        actions.push(raise);
                    }
                },
                None => {},
            }
        }

        actions
    }
}
