use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    enums::{Message, RegisterTeamResult, RegistrationError, SubscribePlayerResult},
    structs::{RegisterTeam, SubscribePlayer},
};


/// Connection au serveur
pub fn connect(addr: &str) -> TcpStream {
    TcpStream::connect(addr).unwrap()
}

/// Envoi d'un message au serveur.
pub fn send_message(stream: &mut TcpStream, message: &String) {
    // Envois d'abord la taille du message, ensuite le message
    println!("[client] => {}", message);
    let size = message.len() as u32;
    stream.write_all(&size.to_le_bytes()).unwrap();
    stream.write_all(message.as_bytes()).unwrap();
}

/// Récupération d'un message du serveur.
/// Le message est préfixé par sa taille (u32 au format little-endian).
pub fn get_message(stream: &mut TcpStream) -> String {
    // Lis la taille du message (4 octets)
    let mut message_len_buffer = [0_u8; 4];
    stream.read_exact(&mut message_len_buffer).unwrap();
    let message_len = u32::from_le_bytes(message_len_buffer);

    // Lis le message (message_len octets)
    // remplis le buffer en fonction de la taille du message
    let mut message_buffer = vec![0; message_len as usize];
    stream.read_exact(&mut message_buffer).unwrap();

    // Converti le message d'octet en chaine de charactère et le retourne
    match String::from_utf8(message_buffer) {
        Ok(message) => {
            println!("[server] => {}", message);
            message
        }
        Err(err) => {
            panic!("Failed to convert message to UTF-8: {:?}", err);
        }
    }
}

/// Handles the server's response to a message.
fn handle_server_response(stream: &mut TcpStream, message: &Message) -> Message {
    // Serialize the message to JSON
    let message_json = serde_json::to_string(message).unwrap();
    send_message(stream, &message_json);

    // Receive the server's response
    let response_json = get_message(stream);
    serde_json::from_str(&response_json).unwrap()
}




/// Enregistre une équipe et retourne le résultat du serveur.
pub fn register_team(name: &str, server_address: &str) -> Result<RegisterTeamResult, RegistrationError> {
    
    let mut stream = connect(server_address);
    let register_team_message = Message::RegisterTeam(RegisterTeam {
        name: String::from(name),
    });

    let response = handle_server_response(&mut stream, &register_team_message);
    match response {
        Message::RegisterTeamResult(result) => match result {
            RegisterTeamResult::Ok { expected_players, registration_token } => Ok(RegisterTeamResult::Ok { expected_players, registration_token }),
            RegisterTeamResult::Err(err) => Err(err),
        },
        _ => panic!("Unexpected response from server during team registration"),
    }
}


/// Enregistre un joueur avec le serveur et retourne le stream.
pub fn register_player(name: &str, token: &str, server_address: &str) -> Result<(SubscribePlayerResult, TcpStream), RegistrationError> {
    let mut stream = connect(server_address);
    let register_player_message = Message::SubscribePlayer(SubscribePlayer {
        name: String::from(name),
        registration_token: String::from(token),
    });

    let response = handle_server_response(&mut stream, &register_player_message);
    match response {
        Message::SubscribePlayerResult(result) => match result {
            SubscribePlayerResult::Ok => Ok((SubscribePlayerResult::Ok, stream)),
            SubscribePlayerResult::Err(err) => Err(err),
        },
        _ => panic!("Unexpected response from server during player registration"),
    }
}

/// Débute la partie
pub fn start_game(mut streams: Vec<TcpStream>) {
    println!("Game started!");
    loop {
        for stream in streams.iter_mut() {
            let _response_json = get_message(stream);
            // TODO: gérer la réponse du serveur
        }
    }
    
}
