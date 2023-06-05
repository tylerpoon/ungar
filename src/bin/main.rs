use ungar::*;

fn main() {
    let path = "game_configs/kuhn.json";

    let game_info = game::GameInfo::load_game_info(path);
    let starting_state = game::GameState::new(&game_info, 0);
    let action_abstraction = action_abstraction::ActionAbstraction::from_config("game_configs/kuhn_action_abstraction.json");

    println!("{:?}", action_abstraction.get_actions(&game_info, &starting_state));

    // println!("{:?}", game_info);
    // println!("{:?}", starting_state);
    // println!("{:?}", action_abstraction);
}
