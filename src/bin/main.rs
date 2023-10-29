use std::path::PathBuf;

use ungar::{*, cfr::{CFREngine, CFRConfig}, abstract_game::AbstractGame, play::play, strategy::Strategy};

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    Train {
        #[arg(long)]
        cfr_config: PathBuf,
        #[arg(long)]
        output_strategy_path: Option<PathBuf>,
        #[arg(long)]
        output_nodes_path: Option<PathBuf>,
    },
    Play {
        #[arg(short, long)]
        strategy_path: PathBuf,
        #[arg(short, long)]
        nodes_path: PathBuf,
    },
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

fn train(abstract_game: AbstractGame, cfr_config: CFRConfig, output_strategy_path: Option<PathBuf>, output_nodes_path: Option<PathBuf>) {
    let mut cfr_engine = CFREngine::new(abstract_game, cfr_config);

    cfr_engine.mccfr_p(150000, 20, 400, 100000, 2500);
    match output_strategy_path {
        Some(p) => cfr_engine.save_average_strategy(&p),
        None => cfr_engine.print_average_strategy(),
    };
    match output_nodes_path {
        Some(p) => cfr_engine.save_nodes(&p),
        None => (),
    };
    cfr_engine.print_regrets();
    cfr_engine.print_average_strategy()
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let game_info = game::GameInfo::load_game_info(&args.game_config);
    let starting_state = game::GameState::new(&game_info, 0);
    let action_abstraction = action_abstraction::ActionAbstraction::from_config(&args.action_abstraction_config);
    let card_abstraction = card_abstraction::CardAbstraction::from_config(&args.card_abstraction_config);

    match args.command {
        Commands::Train { cfr_config, output_strategy_path, output_nodes_path }=> {
            let abstract_game = AbstractGame::new(game_info, starting_state, action_abstraction, card_abstraction);
            let cfr_config = CFRConfig::from_config(&cfr_config);
            train(abstract_game, cfr_config, output_strategy_path, output_nodes_path);
        },
        Commands::Play { strategy_path, nodes_path } => {
            let mut abstract_game = AbstractGame::load_nodes(game_info, &nodes_path, action_abstraction, card_abstraction);
            let strategy = Strategy::from_file(&strategy_path);
            play(&mut abstract_game, strategy);
        }
    }

}
