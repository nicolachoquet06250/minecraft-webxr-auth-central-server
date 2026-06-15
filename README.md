# Voxicraft Authentication Platform

![Front coverage](https://img.shields.io/endpoint?url=https%3A%2F%2Fgithub.com%2Fnicolachoquet06250%2Fminecraft-webxr-auth-central-server%2Fraw%2Frefs%2Fheads%2Fmain%2F.github%2Fbadges%2Fcoverage.json&label=front%20coverage)
![Back coverage](https://img.shields.io/endpoint?url=https%3A%2F%2Fgithub.com%2Fnicolachoquet06250%2Fminecraft-webxr-auth-central-server%2Fraw%2Frefs%2Fheads%2Fmain%2F.github%2Fbadges%2Fcoverage.json&label=back%20coverage)

Une plateforme d'authentification centralisée pour les serveurs Voxicraft.

## Architecture

### Backend (Rust)
- **Framework**: Axum
- **ORM**: SeaORM
- **Base de données**: MySQL
- **Authentification**: JWT + OAuth Discord
- **Sécurité**: Bcrypt pour les mots de passe

### Frontend (Vue 3)
- **Framework**: Vue 3 + TypeScript
- **Build**: Vite
- **State Management**: Pinia
- **Routing**: Vue Router
- **Design**: Inspiré de Minecraft

## Fonctionnalités

### Gestion des utilisateurs
- ✅ Inscription avec email/mot de passe
- ✅ Connexion Discord OAuth
- ✅ Profils utilisateurs personnalisables
- ✅ Avatars (Steve/Alex)
- ✅ Vérification d'âge (placeholder)

### Gestion des serveurs
- ✅ Enregistrement de serveurs
- ✅ Validation de domaines
- ✅ Gestion multi-serveurs par créateur

## Structure du projet

```
voxicraft-auth-platform/
├── backend/           # API Rust
│   ├── src/
│   │   ├── main.rs         # Point d'entrée
│   │   ├── models/         # Modèles de données (User, Server)
│   │   ├── routes/         # Routes API (auth, user, server)
│   │   ├── services/       # Services (JWT, Discord OAuth)
│   │   ├── middleware/     # Middleware d'authentification
│   │   └── dto/            # Data Transfer Objects
│   ├── migration/          # Migrations SQL
│   └── Cargo.toml
│
└── frontend/          # Application Vue 3
    ├── src/
    │   ├── api/            # Client API Axios
    │   ├── components/     # Composants Vue
    │   ├── views/          # Pages (Home, Login, Register, Profile, Servers)
    │   ├── stores/         # Stores Pinia (auth, server)
    │   ├── router/         # Configuration des routes
    │   └── style.css       # Styles Minecraft
    └── package.json
```

## Installation

### Prérequis
- Rust (1.70+)
- Node.js (18+)
- MySQL (8.0+)

### Backend

1. **Créer la base de données MySQL:**
```sql
CREATE DATABASE voxicraft_auth;
```

2. **Configurer les variables d'environnement:**
```bash
cd backend
cp .env.example .env
```

Éditez le fichier `.env`:
```
DATABASE_URL=mysql://root:password@localhost:3306/voxicraft_auth
JWT_SECRET=your_secret_key_change_this_in_production
DISCORD_CLIENT_ID=your_discord_client_id
DISCORD_CLIENT_SECRET=your_discord_client_secret
DISCORD_REDIRECT_URI=http://localhost:3000/auth/discord/callback
API_PORT=8080
CORS_ORIGIN=http://localhost:5173
```

3. **Créer les tables de base de données:**
```bash
mysql -u root -p voxicraft_auth < migration/init.sql
```

4. **Lancer le serveur:**
```bash
cargo build
cargo run
```

Le serveur sera accessible sur `http://localhost:8080`

### Frontend

1. **Installer les dépendances:**
```bash
cd frontend
npm install
```

2. **Lancer le serveur de développement:**
```bash
npm run dev
```

L'application sera accessible sur `http://localhost:5173`

## API Endpoints