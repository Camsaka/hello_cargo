# passwordManager
Application de management de mot de passe en local

# How to launch
cargo build (debug by default)
to build release : cargo build --release
.\target\debug\hello_cargo.exe

or 

cargo run

# Check will not produce an executable file
cargo check


# Feuille de route

Phase 1 — Fondations Rust
Semaines 1–3 · Rustlings + concepts clés
· Ownership, borrowing, lifetimes
· Types de base, enums, pattern matching
· Gestion d'erreurs avec Result<T, E>


Phase 2 — CLI basique, pas de chiffrement
Semaines 4–6 · Premier vrai programme
· Entrées/sorties avec stdin, println!
· Stockage JSON local avec serde_json
· Recherche et affichage d'entrées
· Parsing d'arguments avec clap
→ Jalon : ajouter / lister / supprimer une entrée


Phase 3 — Chiffrement et sécurité
Semaines 7–10 · Le cœur du projet
· Dérivation de clé master avec Argon2
· Chiffrement des données avec AES-256-GCM
· Crate recommandées : ring, argon2, aes-gcm
· Zéroïsation mémoire (zeroize crate)
→ Jalon : vault chiffré persisté sur disque


Phase 4 — Fonctionnalités avancées
Semaines 11–16 · Peaufinage et exploration
· Génération de mots de passe sécurisés
· Interface TUI avec ratatui
· Export / import chiffré
· Tests unitaires et d'intégration
→ Jalon : projet utilisable au quotidien