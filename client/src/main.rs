use std::thread;

use shared::{
    base64::{decode, encode},
    enums::RegisterTeamResult,
    functions::{register_player, register_team},
};

const SERVER_ADDRESS: &str = "localhost:8778";

// fn main() {
//     let team_token: String;

//     // Enregistrer une équipe
//     let register_message = register_team("deadRuster0X256", SERVER_ADDRESS);

//     // Déconstruire le message du server
//     match register_message {
//         RegisterTeamResult::Ok {
//             expected_players,
//             registration_token,
//         } => {
//             println!("Expected player: {:?}", expected_players);
//             team_token = registration_token;
//         }

//         RegisterTeamResult::Err(err) => {
//             panic!("{:?}\n", err);
//         }
//     }

//     println!("Token: {:?}\n", &team_token);

//     let mut threads: Vec<_> = Vec::new();

//     // Enregister des joueurs et lancer la partie
//     for i in 0..=2 {
//         let team_token_clone = team_token.clone();

//         threads.push(thread::spawn(move || {
//             register_player(
//                 format!("Player-{}", &i).as_str(),
//                 &team_token_clone,
//                 SERVER_ADDRESS,
//             )
//         }));

//         println!("Registering player number {} \n", i);
//     }

//     // Attendre que tous les threads se terminent
//     for thread in threads {
//         thread.join().unwrap();
//     }
// }

fn main() {
    let encoded = encode(b"Hello, World!");
    println!("Encoded: {}", encoded);

    let decoded = decode(&encoded).unwrap();
    println!("Decoded: {:?}", String::from_utf8(decoded).unwrap());
}
