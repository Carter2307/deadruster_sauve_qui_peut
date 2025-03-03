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

## 🧪 Tests Unitaires

Plusieurs fonctions critiques sont couvertes par des tests unitaires pour garantir leur fiabilité :

### Encodage/décodage Base64 (`shared/src/base64.rs`)
- Vérifie l'encodage/décodage de chaînes simples (ex: `"Hello"` → `"sgvSBg8"`)
- Gère les cas limites (valeurs 0-255, padding)
- Rejette les caractères invalides

**Lancer les tests** :
```bash
cd ./shared
cargo test -- --test-threads=1
```

### Décodage des RadarView (`shared/src/radar_view.rs`)
- Valide le décodage d'une vue radar encodée (ex: `"ieysGjGO8papd/a"`)
- Vérifie la cohérence mur/cellule après un round-trip (encodage → décodage)

**Lancer les tests** :
```bash
cd ./shared
cargo test --test radar_tests
```

### Exemple de sortie réussie
```
running 4 tests
test base64::tests::test_decode_invalid ... ok
test base64::tests::test_encode ... ok
test base64::tests::test_all_case ... ok
test base64::tests::test_decode ... ok

running 1 test
test radar_view::tests::test_decode_encode ... ok
```

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
