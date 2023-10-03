use ungar::{*, cfr::{CFREngine, CFRConfig}, abstract_game::AbstractGame};

fn main() {
    env_logger::init();

    let path = "game_configs/leduc.json";

    let game_info = game::GameInfo::load_game_info(path);
    let starting_state = game::GameState::new(&game_info, 0);
    let action_abstraction = action_abstraction::ActionAbstraction::from_config("game_configs/leduc_action_abstraction.json");
    let buckets0 = card_abstraction::NoBuckets::new(&game_info, 0);
    let buckets1 = card_abstraction::NoBuckets::new(&game_info, 1);
    let card_abstraction = card_abstraction::CardAbstraction::new(vec![Box::new(buckets0), Box::new(buckets1)]);

    let abstract_game = AbstractGame::new(game_info, starting_state, action_abstraction, card_abstraction);
    let config = CFRConfig::new(1);
    let mut cfr_engine = CFREngine::new(abstract_game, config);

    cfr_engine.mccfr_p(1500, 20, 400, 400, 400);
    cfr_engine.print_average_strategy();
}
