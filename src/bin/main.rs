use std::path::PathBuf;

use ungar::{*, cfr::{CFREngine, CFRConfig}, abstract_game::AbstractGame};

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    Train {
        #[arg(long)]
        cfr_config: PathBuf,
    },
    Play,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config file for the game
    #[arg(short, long)]
    game_config: PathBuf,

    #[arg(short, long)]
    action_abstraction_config: PathBuf,

    #[arg(short, long)]
    card_abstraction_config: PathBuf,

    /// Command
    #[command(subcommand)]
    command: Commands,
}

fn train(abstract_game: AbstractGame, cfr_config: CFRConfig) {
    let mut cfr_engine = CFREngine::new(abstract_game, cfr_config);

    cfr_engine.mccfr_p(1500, 20, 400, 400, 400);
    cfr_engine.print_average_strategy();
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let game_info = game::GameInfo::load_game_info(&args.game_config);
    let starting_state = game::GameState::new(&game_info, 0);
    let action_abstraction = action_abstraction::ActionAbstraction::from_config(&args.action_abstraction_config);
    let card_abstraction = card_abstraction::CardAbstraction::from_config(&args.card_abstraction_config);

    let abstract_game = AbstractGame::new(game_info, starting_state, action_abstraction, card_abstraction);

    match args.command {
        Commands::Train { cfr_config }=> {
            let cfr_config = CFRConfig::from_config(&cfr_config);
            train(abstract_game, cfr_config);
        },
        Commands::Play => {
        }
    }

}
