use ungar::{*, cfr::CFREngine, abstract_game::AbstractGame};

fn main() {
    env_logger::init();

    let path = "game_configs/kuhn.json";

    let game_info = game::GameInfo::load_game_info(path);
    let starting_state = game::GameState::new(&game_info, 0);
    let action_abstraction = action_abstraction::ActionAbstraction::from_config("game_configs/kuhn_action_abstraction.json");
    let buckets = card_abstraction::NoBuckets::new(&game_info, 0);
    let card_abstraction = card_abstraction::CardAbstraction::new(vec!(Box::new(buckets)));

    let abstract_game = AbstractGame::new(game_info, starting_state, action_abstraction, card_abstraction);
    let mut cfr_engine = CFREngine::new(abstract_game);

    cfr_engine.mccfr_p(1500, 20, 400, 400, 400);
    cfr_engine.print_strategy();
}
