use ungar::*;

fn main() {
    let path = "game_configs/kuhn.json";

    let game_info = game::GameInfo::load_game_info(path);

    println!("{:?}", game_info);
}
