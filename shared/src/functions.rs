use std::{
    io::{Read, Write},
    net::TcpStream, sync::{Arc, Mutex, MutexGuard},
};

use crate::{
    enums::{
        Action, ActionError, Challenge, Hint, Message, RegisterTeamResult, SubscribePlayerResult
    },
    game_engine::{Direction, GameState, GlobalMap, Player},
    radar_view::{decode_radarview, RadarView},
    structs::{RegisterTeam, SubscribePlayer},
};

pub fn send_message(stream: &mut TcpStream, message: &String) {
    // Envois d'abord la taille du message, ensuite le message
    let size = message.len() as u32;
    stream.write(&size.to_le_bytes()).unwrap();
    stream.write(message.as_bytes()).unwrap();
}

pub fn get_message(stream: &mut TcpStream) -> String {
    //Lis la réponse du server
    let mut recieved_message_len_buffer = [0_u8; 4];

    println!("{:?}",stream);

    //Lis la taille du message envoyer par le server
    stream.read_exact(&mut recieved_message_len_buffer).unwrap();
    // Converti la taille recue (en octet) en entier
    let n = u32::from_le_bytes(recieved_message_len_buffer);

    // remplis le buffer en fonction de la taille du message
    let mut message_recieved_buffer = vec![0; n as usize];
    stream.read_exact(&mut message_recieved_buffer).unwrap();

    // Converti le message d'octet en chaine de charactère et le retourne
    match String::from_utf8(message_recieved_buffer) {
        Ok(message) => message,
        Err(err) => {
            panic!("Err: {:?}", err)
        }
    }
}

pub fn register_team(name: &str, server_adress: &str) -> RegisterTeamResult {
    let mut stream = connect(server_adress);

    print!(
        "Register team stream addr: {:?}",
        &stream.local_addr().unwrap().port()
    );

    let register_team_message = Message::RegisterTeam(RegisterTeam {
        name: String::from(name),
    });

    //Transform le message en json
    match serde_json::to_string(&register_team_message) {
        Ok(message) => {
            println!("{message}");

            send_message(&mut stream, &message);
            let message_text: String = get_message(&mut stream);

            // Passe d'une chaine à une structure ou un enum
            println!("{:?}", &message_text);
            let message: Message = serde_json::from_str(&message_text).unwrap();

            if let Message::RegisterTeamResult(register_team_result) = message {
                register_team_result
            } else {
                panic!("Unexpected message type")
            }
        }

        Err(error) => {
            panic!("{error}");
        }
    }
}

pub fn send_move(stream: &mut TcpStream, direction: &Direction) {
    let action_move_message = Message::Action(Action::MoveTo(*direction));
    print!("Move to: {:?}\n", *direction);
    let action_move_message_stringify = serde_json::to_string(&action_move_message).unwrap();
    send_message(stream, &action_move_message_stringify);
}

pub fn resolve_secret_sum_challenge(stream: &mut TcpStream, game_state:&mut MutexGuard<'_, GameState>) {
    let answer = game_state.calculate_secret_sum_modulo(game_state.modulo);
    let action = Message::Action(Action::SolveChallenge {
        answer: answer.to_string(),
    });
    let action_message_string = serde_json::to_string(&action).unwrap();
    print!("Action message string: {:?}\n", &action_message_string);
    send_message(stream, &action_message_string);
}

pub fn play(player: &mut Player, mut stream: &mut TcpStream, game_state_clone:&mut  Arc<Mutex<GameState>>, map_clone: &mut Arc<Mutex<GlobalMap>>) {
    
    let mut game_state = game_state_clone.lock().unwrap();


    let message = get_message(&mut stream);
    let response: Message = serde_json::from_str(&message).unwrap();

    // Si c'est une radaview
    if let Message::RadarView(encoded_string) = &response {
        println!("\n ==== Reception d'une radaview ===\n");
        let mut map = map_clone.lock().unwrap();
        let radar: RadarView = decode_radarview(&encoded_string).unwrap();

        // Mettre à jour la carte avec les nouvelles informations
        map.update_from_radar(&radar);

        // Choisir un déplacement
        let direction = map.next_move(map.player_direction);

        map.player_direction = direction; 

        // Mettre à jour la position du joueur
        map.move_player(direction);


        // Envoyer le mouvement au serveur
        send_move(&mut stream, &direction);

    }

    // Si c'est un challenge
    if let Message::Challenge(challenge) = &response {
        println!("\n === Reception d'un Challenge {:?} ===\n", challenge);
        match challenge {
            Challenge::SecretSumModulo(modulo) => {
                // Stocker le modulo
                game_state.modulo = *modulo;
                resolve_secret_sum_challenge(stream, &mut game_state);
            }
        }
    }

    // Si c'est un indice
    if let Message::Hint(hint) = &response {
        println!("\n ==== Reception d'un indice ===\n");
        match hint {
            Hint::Secret(secret) => {
                game_state.update_secret(&player.name, *secret);
                println!("Hint secret:{:?}\n", *secret);
            },
            Hint::RelativeCompass { angle } => {
                print!("Relative compass {}", &angle);
            },
            _ => (),
        }
    }

    // Si c'est une Action error
    if let Message::ActionError(error) = &response {
        println!("\n ==== Reception d'une action error {:?} ===\n", error);

        match error {
            ActionError::CannotPassThroughWall => print!("Cannot pass through wall!"),
            ActionError::CannotPassThroughOpponent => print!("Cannot pass through opponent!"),
            ActionError::NoRunningChallenge => print!("No Running challenge!"),
            ActionError::SolveChallengeFirst => print!("Solve challenge first!"),
            ActionError::InvalidChallengeSolution => {
                println!("Invalid challenge solution");
                resolve_secret_sum_challenge(stream, &mut game_state);
            },
        }
    }
}

pub fn register_player(name: &str, token: &String, mut stream: &mut TcpStream) -> bool {
    print!(
        "Register player stream addr: {:?}\n",
        &stream.local_addr().unwrap().port()
    );

    let player: Message = Message::SubscribePlayer(SubscribePlayer {
        name: String::from(name),
        registration_token: String::from(token),
    });

    match serde_json::to_string(&player) {
        Ok(player_string) => {
            send_message(&mut stream, &player_string);
            let response: String = get_message(&mut stream);

            let result: Message = serde_json::from_str(&response).unwrap();

            if let Message::SubscribePlayerResult(subscribe_player_result) = result {
                match subscribe_player_result {
                    SubscribePlayerResult::Ok => {
                        println!("Joueur bien enregistrée");
                        true
                    },

                    SubscribePlayerResult::Err(err) => {
                        print!("Error occur: {:?}", err);
                        false
                    }
                }
            } else {
                panic!("Unexpected message type");
            }
        }
        Err(error) => {
            panic!("{error}");
        }
    }
}

pub fn connect(addr: &str) -> TcpStream {
    let connection = TcpStream::connect(addr).unwrap();
    connection
}
