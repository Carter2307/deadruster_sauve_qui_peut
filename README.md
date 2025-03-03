# Projet Rust - *Sauve qui peut*
**Architecture des Logiciels - ESGI - 4ème année**

Un client Rust pour explorer un labyrinthe généré par un serveur, résoudre des défis collaboratifs et s'échapper avant ses adversaires.

---

## 🚀 Fonctionnalités implémentées
- **Enregistrement des équipes/joueurs** sur le serveur.
- **Réception des vues radar** (`RadarView`) encodées et décodage partiel.
- **Déplacements de base** (`MoveTo`) avec gestion des murs.
- **Solveur de labyrinthe** (BFS) pour guider les joueurs.
- **Communication TCP** avec préfixe de taille (format `u32` little-endian + JSON).
- **Serveur de test minimal** pour valider l'enregistrement et les déplacements.

---

## 📦 Installation
1. **Installer Rust** :


2. **Cloner le dépôt** :
   ```bash  
   git clone https://github.com/Carter2307/deadruster_sauve_qui_peut.git 
   ```
---

## 🕹️ Utilisation

### Mode standard (avec le serveur de référence)
1. **Lancer le serveur** :
   ```bash  
   cd <chemin_vers_serveur_test>
   ./server run
   ```  
2. **Lancer 3 clients/joueurs** (dans un terminal séparé) :
   ```bash  
   cd ./client  
   cargo run -- live  
   ```  

### Mode test (avec le serveur minimal)
1. **Lancer le serveur de test** :
   ```bash  
   cd ./server  
   cargo run
   ```  
2. **Lancer les clients en mode test** :
   ```bash  
   cd ./client  
   cargo run -- test  
   ```  

---

## 🧩 Structure du projet
```  
.  
├── client/           # Crate du client (joueurs)  
├── server/           # Crate du serveur minimal  
├── shared/           # Structures et fonctions communes  
│   ├── enums.rs      # Messages JSON (RegisterTeam, Action...)  
│   ├── radar_view.rs # Décodage des RadarView  
│   └── base64.rs     # Encodage/décodage Base64  
├── algorithms/       # Solveur de labyrinthe (BFS)  
└── Cargo.toml        # Configuration du workspace  
```  

---

## 👥 Contributeurs
- Roger BENTCHA
- Clément KINSIONA MFINDA
- Gilles KOUNDOUD
- Raphael GOISQUE
---
