use std::{
    collections::{HashMap, HashSet}, env, net::TcpStream, sync::{Arc, Mutex}, thread
};

use shared::{
    enums::RegisterTeamResult,
    functions::{connect, play, register_player, register_team},
    game_engine::{Direction, GameState, GlobalMap, Player},
};

const SERVER_ADDRESS: &str = "localhost:8778";
const TEST_SERVER_ADDRESS: &str= "localhost:8888";

pub fn get_mode() -> String {
    let args: Vec<String> = env::args().collect();
    let mode = args[1].clone();
    mode
}

fn main() {
    let team_token: String;
    let max_players: u8;
    let mut server_address = TEST_SERVER_ADDRESS;
    let mode = get_mode();
    
    println!("Mode is {mode}");

    if mode == "live" {
        server_address  = SERVER_ADDRESS;
    }

    // Enregistrer une équipe
    let register_message: RegisterTeamResult = register_team("deadRuster0X256", server_address);

    // Déconstruire le message du server
    match register_message {
        RegisterTeamResult::Ok {
            expected_players,
            registration_token,
        } => {
            println!("Expected player: {:?}", &expected_players);
            team_token = registration_token;
            max_players = expected_players;
        }

        RegisterTeamResult::Err(err) => {
            panic!("{:?}\n", err);
        }
    }

    println!("Token: {:?}\n", &team_token);

    // Initialiser l'état du jeu
    let game_state = Arc::new(Mutex::new(GameState {
        team_secrets: HashMap::new(),
        modulo: 0,
    }));

    let mut threads: Vec<_> = Vec::new();
    let map = Arc::new(Mutex::new(GlobalMap::new()));


    // Enregister des joueurs et lancer la partie
    for i in 0..max_players {
        let team_token_clone = team_token.clone();
        let mut game_state_clone = Arc::clone(&game_state);
        let mut map_clone = Arc::clone(&map);

        threads.push(thread::spawn(move || {
            let mut stream: TcpStream = connect(server_address);

            let can_play = register_player(
                format!("Player-{}", &i).as_str(),
                &team_token_clone,
                &mut stream,
            );

            if can_play {

                let mut player = Player {
                    name: format!("Player-{}", &i),
                    position: (0, 0),
                    secret: Some(0),
                    direction: Direction::Front
                };

                println!("\n ==== La partie a commencé ===\n");

                loop {
                    play(&mut player, &mut stream, &mut game_state_clone, &mut map_clone);
                }
            }
        }));
    }

    // Attendre que tous les threads se terminent
    for thread in threads {
        thread.join().unwrap();
    }
}

