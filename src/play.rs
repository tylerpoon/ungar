use super::{
    abstract_game::AbstractGame,
    strategy::Strategy,
};

pub fn play(abstract_game: AbstractGame, strategy: Strategy) {
    println!("{:?}", strategy);
}
