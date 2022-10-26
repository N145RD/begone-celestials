use std::io;
use ggst_api::*;

pub async fn verifier(player_name: String) -> i64 {
    let (replays, _errors) = get_replays(
        &Context::default(),
        100,
        127,
        QueryParameters::default()
            .min_floor(Floor::Celestial)
        ).await.unwrap();

    println!("Errors:");
    for error in _errors {
        println!("{}", error);
    }
    for replay in replays {
        if (replay.winner == Winner::Player1 && replay.players.0.name == player_name) || (replay.winner == Winner::Player2 && replay.players.1.name == player_name) {
            println!("Player {player_name} won against a celestial player at {replay}");
            return 1;
        }
    }
    println!("Player {player_name} has never won against a celestial player");
    return 0;
}
#[tokio::main]
async fn main() {
    let mut player_name = String::new();

    println!("Give the name of the player to check:");
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line !");
    
    verifier(player_name);
}
