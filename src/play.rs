use super::{
    abstract_game::AbstractGame,
    strategy::Strategy,
    game::Action,
};

use poker::Evaluator;

use std::io;

pub fn play(abstract_game: &mut AbstractGame, strategy: Strategy) {
    let evaluator = Evaluator::new();
    let player: u8 = 0;
    let mut total_payouts = vec![0; abstract_game.game_info.num_players() as usize];
    let mut matches = 0;

    loop {
        println!("Do you want to play a game? [Y/n]");
        let mut action_line: String = String::new();
        match io::stdin().read_line(&mut action_line) {
            Ok(_) => (),
            Err(e) => continue
        };
        match action_line.trim() {
            "y" => (),
            "Y" => (),
            "yes" => (),
            _ => break
        };

        let (hole_cards, board_cards) = abstract_game.game_info.deal_hole_cards_and_board_cards();
        let mut board_cards_i = abstract_game.game_info.total_board_cards(0) as usize;

        let mut node_id = abstract_game.nodes.get_root_node_id();
        let mut current_node = abstract_game.nodes.get_node(node_id).unwrap();
        let mut state = current_node.state.clone();
        
        while !state.is_finished() {
            println!("Chips in pot: {}", state.pot_total(&abstract_game.game_info));
            for i in 0..abstract_game.game_info.num_players() {
                println!("Player {} chips: {}", i, state.player_stack(i) - state.player_spent(i));
            }
            print!("Player hole cards: ");
            for card in &hole_cards[player as usize] {
                print!("{} ", card);
            }
            print!("\n");
            print!("Board cards: ");
            for i in 0..board_cards_i {
                print!("{} ", board_cards[i]);
            }
            print!("\n");

            let mut action;
            if state.current_player().unwrap() == player {
                loop {
                    println!("Input action:");
                    action = match get_player_action() {
                        Ok(a) => a,
                        Err(_) => { println!("Failed to read action"); continue },
                    };

                    if state.is_valid_action(&abstract_game.game_info, action) {
                        break;
                    } else {
                        println!("Not a valid action");
                    }
                }
            } else {
                let bucket_id = abstract_game.get_bucket(state.current_round(), &board_cards, &hole_cards[state.current_player().unwrap() as usize]);
                action = strategy.sample(&abstract_game, node_id, bucket_id);
            }

            println!("Player {}: {}\n", state.current_player().unwrap(), action);
            state = state.apply_action_no_cards(&abstract_game.game_info, action).unwrap();
            node_id = abstract_game.apply_action_to_node(node_id, &mut board_cards_i, action);
            current_node = abstract_game.nodes.get_node(node_id).unwrap();
        }

        println!("Round finished");

        for i in 0..abstract_game.game_info.num_players() {
            let payout = state.get_payout(&abstract_game.game_info, &evaluator, &board_cards, &hole_cards, i);
            total_payouts[i as usize] += payout;
            print!("Player {} hole cards: ", i);
            for card in &hole_cards[i as usize] {
                print!("{} ", card);
            }
            print!("\n");
            println!("Player {} payout: {}", i, payout);
        }

        matches += 1;

        print!("\n");
        println!("Total payouts after {} matches", matches);
        for i in 0..abstract_game.game_info.num_players() {
            println!("Player {} total payout: {}", i, total_payouts[i as usize]);
        }
        print!("\n");
    }
}

fn get_player_action() -> Result<Action, io::Error> {
    let mut action_line: String = String::new();
    io::stdin().read_line(&mut action_line)?;

    let action_line: Vec<&str> = action_line.trim().split_whitespace().collect();

    if action_line.len() < 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid action input"));
    }

    match action_line[0] {
        "f" => Ok(Action::Fold),
        "c" => Ok(Action::Call),
        "r" =>  {
            if action_line.len() < 2 {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid action input"));
            }
            let raise_size: u32 = match action_line[1].parse() {
                Ok(i) => i,
                Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid action input")),
            };

            Ok(Action::Raise(raise_size))
        },
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid action input")),
    }
}
