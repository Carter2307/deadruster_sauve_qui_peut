use std::thread;
use std::net::TcpStream;
use shared::enums::{Message, RegisterTeamResult};
use shared::structs::{MazeData, Position};
use algorithms::maze::Maze;
use shared::functions::{send_message, get_message};
use shared::functions::{register_team, register_player};


fn solve_maze(maze_data: MazeData) -> Vec<Position> {
    let mut maze = Maze::new(
        maze_data.width, 
        maze_data.height,
        maze_data.start,
        maze_data.end
    );
    
    for wall in maze_data.walls {
        maze.add_wall(wall);
    }

    maze.solve().unwrap_or_default()
}

const SERVER_ADDRESS: &str = "localhost:8778";

fn main() {
    let team_token: String;

    // Enregistrer une équipe
    let register_message = register_team("deadRuster0X256", SERVER_ADDRESS);

    // Déconstruire le message du server
    match register_message {
        RegisterTeamResult::Ok {
            expected_players,
            registration_token,
        } => {
            println!("Expected player: {:?}", expected_players);
            team_token = registration_token;
        }

        RegisterTeamResult::Err(err) => {
            panic!("{:?}\n", err);
        }
    }

    println!("Token: {:?}\n", &team_token);

    let mut threads: Vec<_> = Vec::new();

    // Enregister des joueurs et lancer la partie
    for i in 0..=2 {
        let team_token_clone = team_token.clone();

        threads.push(thread::spawn(move || {
            register_player(
                format!("Player-{}", &i).as_str(),
                &team_token_clone,
                SERVER_ADDRESS,
            )
        }));

        println!("Registering player number {} \n", i);
    }

    // Attendre que tous les threads se terminent
    for thread in threads {
        thread.join().unwrap();
    }

     // Après l'enregistrement réussi du joueur
     let mut stream = TcpStream::connect(SERVER_ADDRESS).unwrap();
    
     // Demander le labyrinthe
     let request = Message::RequestMaze;
     let request_str = serde_json::to_string(&request).unwrap();
     send_message(&mut stream, &request_str);
 
     // Recevoir le labyrinthe
     let response = get_message(&mut stream);
     let maze_message: Message = serde_json::from_str(&response).unwrap();
 
     if let Message::MazeResponse(maze_data) = maze_message {
         // Résoudre le labyrinthe
         let solution = solve_maze(maze_data.clone());
         
         // Envoyer la solution
         let solution_message = Message::MazeSolution(solution);
         let solution_str = serde_json::to_string(&solution_message).unwrap();
         send_message(&mut stream, &solution_str);
     }
}
