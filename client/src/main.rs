use shared::{
    enums::{RegisterTeamResult, SubscribePlayerResult},
    functions::{register_player, register_team, start_game},
};

const SERVER_ADDRESS: &str = "localhost:8778";
const NB_PLAYERS: usize = 3;

fn main() {

    println!("\n------ Client started ------");

    let team_token: String;

    println!("\n--- 1: Enregistrer une équipe ('RegisterTeam')");
    match register_team("deadRuster0X256", SERVER_ADDRESS) {
        Ok(RegisterTeamResult::Ok {
            expected_players,
            registration_token,
        }) => {
            println!("Expected player: {:?}", expected_players);
            team_token = registration_token;
        }

        Ok(RegisterTeamResult::Err(err)) => {
            panic!("{:?}\n", err);
        }

        Err(err) => {
            panic!("Failed to register team: {:?}", err);
        }
    }

    println!("Token: {:?}", &team_token);

    println!("\n--- 2 : Enregistrer les joueurs ('SubscribePlayer')");
    let mut streams = Vec::new();
    let mut player_name: String;

    for i in 0..=NB_PLAYERS-1 {
        player_name = format!("Player-{}", i);
        match register_player(&player_name, &team_token, SERVER_ADDRESS) {
            Ok((SubscribePlayerResult::Ok, stream)) => streams.push(stream),
            Ok((SubscribePlayerResult::Err(err), _)) => eprintln!("Failed to register player: {:?}", err),
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }

    println!("\n--- 3 : Démarrer le jeu (start_game() à implémenter)");
    if streams.len() == NB_PLAYERS {
        start_game(streams);
    } else {
        eprintln!("Not all players were registered successfully.");
    }
    

}
