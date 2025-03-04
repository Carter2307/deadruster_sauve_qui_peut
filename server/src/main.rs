use rand::{distr::Alphanumeric, rng, Rng};
use serde::{Deserialize, Serialize}; // Dé/Sérialisation JSON
use shared::{
    enums::{Action, Message, RegisterTeamResult, RegistrationError, SubscribePlayerResult}, functions::{get_message, send_message}, game_engine::{Direction, Player}, structs::SubscribePlayer
};
use std::{
    collections::HashMap, // Pour stocker les équipes
    net::{TcpListener, TcpStream}, // outils pour la communication réseau
    sync::{Arc, Mutex}, // Gestion des accès concurrent aux données
    thread, // Gérer plusieurs connexions en parallèle
};

// Fournit des méthodes pour convertir des objets en JSON et inversement
pub struct Request;

impl Request {
    // Convertit chaîne JSON => objet Rust
    pub fn from_string<'a, T: Serialize + Deserialize<'a>>(text: &'a str) -> Option<T> {
        match serde_json::from_str(text) {
            Ok(value) => Some(value),
            Err(e) => {
                print!("Une erreur c'est produite lors de la déserialisation: {e:?}");
                None
            }
        }
    }

    // Convertit un objet Rust en chaîne JSON
    fn to_serde_string<T: Serialize>(element: T) -> Result<String, serde_json::Error> {
        match serde_json::to_string(&element) {
            Ok(value) => Ok(value),
            Err(e) => {
                print!("Une erreur c'est produite lors de la Sérialisation: {e:?}");
                Err(e)
            }
        }
    }
}

// Le serveur écoute sur localhost port 8888
const SERVER_PORT: &str = "localhost:8888";

// Controller: Gestion des équipes et joueurs
pub struct Controller {
    // Stock les équipes
    pub teams: HashMap<String, Team>,
    // nb joueur attendu par équipe
    pub expected_players: u64,
}

// Fournit des services réseau pour envoyer des messages JSON aux clients
pub struct Services;

impl Services {

    // Enregistrement d'une équipe
    pub fn register_team_service(
        &self,
        register_result: RegisterTeamResult,
        stream: &mut TcpStream,
    ) {
        // Transformer le message JSON => string
        if let Ok(register_team_string) =
            serde_json::to_string(&Message::RegisterTeamResult(register_result))
        {
            // Envoyer le message
            send_message(stream, &register_team_string);
        }
    }

    // Enregistre un joueur auprès du serveur
    pub fn subscribe_player_service(
        &self,
        subscribe_player_result: SubscribePlayerResult,
        stream: &mut TcpStream,
    ) {
        // Transformer le message en string
        if let Ok(subscribe_player_string) =
            serde_json::to_string(&Message::SubscribePlayerResult(subscribe_player_result))
        {
            // Envoyer le message au client
            send_message(stream, &subscribe_player_string);
        }
    }
}

// Controller: Gestion des équipes et joueurs
impl Controller {

    // Sauvegarde des équipes et joueurs
    pub fn save_team(&mut self, team: &Team) {
        self.teams.insert(team.clone().name, team.clone());
    }

    // Ajoute une équipe dans le Hashmap
    pub fn save_player(&mut self, player: &SubscribePlayer) {
        for (_, team) in self.teams.iter_mut() {
            if team.token == player.registration_token {
                team.players.push(Player {
                    position: (0, 0),
                    name: player.name.clone(),
                    secret: None,
                    direction: Direction::Front
                });
            }
        }
    }

    // Enregistrer une équipe
    pub fn register_team(&mut self, team: Team) -> RegisterTeamResult {
        // Vérifier si le nom de l'équipe n'est pas vide => InvalidName
        if team.name.len() < 3 {
            return RegisterTeamResult::Err(RegistrationError::InvalidName);
        }

        // Vérifier si l'équipe existe déjà => AlreadyRegistered
        if self.teams.contains_key(&team.name) {
            return RegisterTeamResult::Err(RegistrationError::AlreadyRegistered);
        }

        // Sauvegarder l'équipe
        self.save_team(&team);

        // Retourner le resultat de l'enregistrement
        RegisterTeamResult::Ok {
            expected_players: 3,
            registration_token: team.token,
        }
    }

    // Enregistrement d'un joueur
    pub fn register_player(
        &mut self,
        player_to_subscribe: &SubscribePlayer,
    ) -> SubscribePlayerResult {
        let mut player_team: Option<&Team> = None;

        // Trouver l'équipe avec le token correspondant
        for (_, team) in self.teams.iter_mut() {
            if team.token == player_to_subscribe.registration_token {
                player_team = Some(team);
                break; // On a trouvé l'équipe, on peut sortir de la boucle
            }
        }

        // Si aucune équipe ne correspond, retourner une erreur
        let player_team = match player_team {
            Some(team) => team,
            None => return SubscribePlayerResult::Err(RegistrationError::InvalidRegistrationToken),
        };

        // Vérifier que le nombre de joueurs ne dépasse pas la limite
        if player_team.players.len() >= self.expected_players as usize {
            return SubscribePlayerResult::Err(RegistrationError::TooManyPlayers);
        }

        // Vérifier que le joueur n'est pas déjà enregistré
        if player_team
            .players
            .iter()
            .any(|p| p.name == player_to_subscribe.name)
        {
            return SubscribePlayerResult::Err(RegistrationError::AlreadyRegistered);
        }

        // Enregistrer le joueur
        self.save_player(player_to_subscribe);

        // Retourner un succès
        SubscribePlayerResult::Ok
    }

    pub fn start_game(&mut self, mut stream: &mut TcpStream) {
        print!("Start Game");

        // Liste définié des vues radars à envoyé
        let radarview_sample = "ieysGjGO8papd/a";

        // Envoyer un radarview
        let radar_string =
            Request::to_serde_string(Message::RadarView(String::from(radarview_sample))).unwrap();

        print!("{}", &radar_string);

        send_message(stream, &radar_string);

        // Recevoir une action
        let response: String = get_message(&mut stream);
        let message: Message = Request::from_string(&response).unwrap();

        if let Message::Action(action) = message {
            match action {
                Action::MoveTo(direction) => print!("MoveTo action: {:?}", direction),
                Action::SolveChallenge { answer } => print!("Solve Challenge answer: {:?}", answer),
            }
        }
    }
}

// Une équipe de joueurs
#[derive(Debug, Serialize, Clone)]
pub struct Team {
    players: Vec<Player>,
    name: String,
    token: String,
}

pub fn gen_team_tokem() -> String {
    let rand_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    print!("{rand_string}");
    rand_string
}

// Connexion bien établie avec le client
pub fn handle_connection(
    stream: &mut TcpStream,
    controller: &mut Controller,
    services: &mut Services,
) {
    let request = get_message(stream);
    let response: Message = Request::from_string(&request).unwrap();

    // Gérer les demandes d'enregistrement d'une équipe
    if let Message::RegisterTeam(register_team) = &response {
        print!("Demande d'enregistrement d'une équipe...");

        // Créer la team
        let team: Team = Team {
            name: register_team.name.clone(),
            players: Vec::new(),
            token: gen_team_tokem(),
        };

        // Enregister la team
        let register_team_result = controller.register_team(team);
        services.register_team_service(register_team_result, stream);
    }

    // Gérer les demandes d'enregistrement des joueurs
    if let Message::SubscribePlayer(player_info) = &response {
        print!("Demande d'enregistrement d'un joueur...");

        // Enregistrer le joueur
        let register_player_result = controller.register_player(&player_info);
        services.subscribe_player_service(register_player_result, stream);
    }

    // Démarrer le jeu si toute les équipes ont trois joueurs
    let mut is_full = true;

    for (_, team) in &controller.teams {
        if (team.players.len() as u64) != controller.expected_players {
            is_full = false;
            break;
        }
    }


    if is_full {
        print!("\n === La partie démarre ===\n");
        loop {
            controller.start_game(stream);
        }
    }
}

fn main() {
    // Initialiser le server
    let listener = TcpListener::bind(SERVER_PORT);
    // Arc & Mutex pour permettre un accès concurrent à la structure Controller
    // Mutex: protège les données quand plusieurs users accèdent au serveur en même temps.
    // Arc: permet de partager ces objets entre plusieurs threads (= tâches parallèles)
    let controller = Arc::new(Mutex::new(Controller {
        teams: HashMap::new(),
        expected_players: 3,
    }));


    // Accepte les connexions entrantes et démarre un thread pour chaque client
    match listener {
        Ok(tcp_listener) => { // Ok = le server s'est bien lancé
            // boucle infinie qui attend qu'un nouvel user se connecte
            for stream in tcp_listener.incoming() {

                // Créer une référence à l'objet controller pour pouvoir le partager avec plusieurs clients
                let controller_clone = Arc::clone(&controller);

                match stream {
                    // A - Si tout va bien, stream contient la connexion de l'utilisateur
                    Ok(mut stream) => { // mut car stream peut être modifié (= connexion active)

                        // Gérer les connections/requêtes au server
                        // Créer un nouveau thread pour gérer ce client sans bloquer le serveur
                        thread::spawn(move || {
                            // Récupère les données de controller et empêche les autres thread de les modifier tant qu'on travaille dessus
                            let mut controller = controller_clone.lock().unwrap();
                            // struct service: // Fournit des services réseau pour envoyer des messages JSON aux clients
                            let mut services = Services;
                            handle_connection(&mut stream, &mut controller, &mut services);
                        });
                    }
                    Err(e) => print!("La connection au client à échoué: {:}", e),
                }
            }
        }
        Err(err) => {
            print!("Une erreur c'est produite lors du lancement du server: {err}")
        }
    }
}