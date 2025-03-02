use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

use crate::{
    enums::{Message, RegisterTeamResult, RegistrationError, SubscribePlayerResult},
    structs::{RegisterTeam, SubscribePlayer},
    radar_view::decode_radarview
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

pub fn register_player(name: &str, token: &String, server_adress: &str) {
    let mut stream: TcpStream = connect(&server_adress);
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
                        // Ready to play
                        loop {
                            let message = get_message(&mut stream);
                            println!("Server is sending data {}", message);
                        }
                    }

                    SubscribePlayerResult::Err(err) => {
                        print!("Error occur: {:?}", err);
                    }
                }
            } else {
                panic!("Unexpected message type");
            }
        }
        Err(error) => {
            panic!("{error}");
        }
    };
}

pub fn connect(addr: &str) -> TcpStream {
    let connection = TcpStream::connect(addr).unwrap();
    connection
}
