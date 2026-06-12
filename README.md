# Voxicraft Authentication Platform

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
CREATE DATABASE minecraft_auth;
```

2. **Configurer les variables d'environnement:**
```bash
cd backend
cp .env.example .env
```

Éditez le fichier `.env`:
```
DATABASE_URL=mysql://root:password@localhost:3306/minecraft_auth
JWT_SECRET=your_secret_key_change_this_in_production
DISCORD_CLIENT_ID=your_discord_client_id
DISCORD_CLIENT_SECRET=your_discord_client_secret
DISCORD_REDIRECT_URI=http://localhost:3000/auth/discord/callback
API_PORT=8080
CORS_ORIGIN=http://localhost:5173
```

3. **Créer les tables de base de données:**
```bash
mysql -u root -p minecraft_auth < migration/init.sql
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

### Authentification
- `POST /api/auth/register` - Inscription
- `POST /api/auth/login` - Connexion
- `GET /api/auth/discord/url` - Obtenir l'URL OAuth Discord
- `GET /api/auth/discord/callback` - Callback Discord OAuth

### Utilisateurs
- `GET /api/users/me` - Profil de l'utilisateur connecté (Auth requis)
- `PUT /api/users/me` - Modifier le profil (Auth requis)
- `DELETE /api/users/me` - Supprimer le compte (Auth requis)
- `GET /api/users/:id` - Profil public d'un utilisateur

### Serveurs
- `POST /api/servers` - Créer un serveur (Auth requis)
- `GET /api/servers` - Liste des serveurs de l'utilisateur (Auth requis)
- `GET /api/servers/:id` - Détails d'un serveur
- `PUT /api/servers/:id` - Modifier un serveur (Auth requis, owner only)
- `DELETE /api/servers/:id` - Supprimer un serveur (Auth requis, owner only)

## Authentification

L'API utilise des tokens JWT pour l'authentification. Pour les requêtes protégées, incluez le token dans le header:

```
Authorization: Bearer <votre_token_jwt>
```

## Configuration Discord OAuth

1. Créer une application Discord sur https://discord.com/developers/applications
2. Ajouter un OAuth2 redirect: `http://localhost:3000/auth/discord/callback`
3. Activer les scopes: `identify`, `email`
4. Copier le Client ID et Client Secret dans le fichier `.env`

## Design Minecraft

Le frontend utilise:
- Police "Press Start 2P" pour l'effet pixel art
- Palette de couleurs inspirée de Minecraft
- Effets d'ombre 3D pour les boutons
- Panels avec textures de bois
- Animation de gradient pour le fond

## Développement

### Tests Backend
```bash
cd backend
cargo test
```

### Build de production

**Backend:**
```bash
cd backend
cargo build --release
```

**Frontend:**
```bash
cd frontend
npm run build
```

## Sécurité

- Les mots de passe sont hashés avec bcrypt
- Les tokens JWT expirent après 24h
- CORS configuré pour le frontend
- Validation des données côté serveur avec validator
- Protection CSRF via les tokens

## Roadmap

- [ ] Système de vérification d'âge via service tiers
- [ ] Gestion des rôles et permissions
- [ ] API pour les serveurs de jeu
- [ ] Système de bannissement
- [ ] Logs d'activité
- [ ] Interface d'administration

## License

MIT

