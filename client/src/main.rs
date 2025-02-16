use std::thread;

use shared::{
    enums::RegisterTeamResult,
    functions::{register_player, register_team}, radar_view::{decode_radarview, encode_radarview},
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
    // Exemple de chaîne encodée (doit être valide selon ton cas d'utilisation)
    let encoded = "ieysGjGO8papd/a"; 

    match decode_radarview(&encoded) {
        Ok((h_passages, v_passages, cells)) => {
            println!("Décodage réussi !");
            println!("Passages horizontaux : {:?}", h_passages);
            println!("Passages verticaux   : {:?}", v_passages);
            println!("Cellules             : {:?}", cells);

            // Test de ré-encodage
            match encode_radarview(&h_passages, &v_passages, &cells) {
                Ok(re_encoded) => {
                    println!("Ré-encodage réussi : {}", re_encoded);
                    if re_encoded == encoded {
                        println!("✅ Test validé : l'encodage est réversible !");
                    } else {
                        println!("❌ Problème : l'encodage donne un résultat différent.");
                    }
                }
                Err(e) => println!("Erreur d'encodage : {:?}", e),
            }
        }
        Err(e) => println!("Erreur de décodage : {:?}", e),
    }
}
