# Guide de Démarrage Rapide

## Configuration Rapide (Développement Local)

### 1. Base de données MySQL

```sql
CREATE DATABASE minecraft_auth CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE minecraft_auth;
SOURCE migration/init.sql;
```

### 2. Configuration Discord OAuth (Optionnel)

Si vous souhaitez tester l'authentification Discord:

1. Visitez https://discord.com/developers/applications
2. Cliquez sur "New Application"
3. Donnez-lui un nom (ex: "Minecraft Auth Local")
4. Dans OAuth2 > General:
   - Ajoutez un Redirect URL: `http://localhost:5173/auth/discord/callback`
   - Sauvegardez
5. Copiez le Client ID et Client Secret

### 3. Configuration Backend

```bash
cd backend

# Créer le fichier .env
echo "DATABASE_URL=mysql://root:password@localhost:3306/minecraft_auth
JWT_SECRET=$(openssl rand -base64 32)
DISCORD_CLIENT_ID=votre_client_id_ou_test
DISCORD_CLIENT_SECRET=votre_client_secret_ou_test
DISCORD_REDIRECT_URI=http://localhost:5173/auth/discord/callback
API_PORT=8080
CORS_ORIGIN=http://localhost:5173" > .env

# Lancer le backend
cargo run
```

### 4. Configuration Frontend

Dans un nouveau terminal:

```bash
cd frontend
npm install
npm run dev
```

## Test Rapide

L'application sera accessible sur: http://localhost:5173

### Créer un compte test

1. Cliquez sur "S'inscrire"
2. Remplissez le formulaire:
   - Pseudo: test_user
   - Email: test@example.com
   - Mot de passe: password123
   - Avatar: Steve ou Alex
   - Date de naissance: Choisissez une date (18+)
3. Soumettez le formulaire

### Tester les fonctionnalités

**Profil:**
- Après inscription, vous serez automatiquement connecté
- Visitez "Mon Profil" pour voir vos informations
- Modifiez votre bio ou avatar

**Serveurs:**
- Visitez "Mes Serveurs"
- Cliquez sur "Créer un serveur"
- Remplissez:
  - Nom: Mon Serveur Test
  - Domaine: https://test.example.com
  - Description: Serveur de test
- Gérez vos serveurs (activer/désactiver, supprimer)

## API Testing avec curl

### Inscription
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123",
    "avatar": "steve",
    "birthdate": "2000-01-01"
  }'
```

### Connexion
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

Sauvegardez le token retourné.

### Profil (Auth requis)
```bash
curl -X GET http://localhost:8080/api/users/me \
  -H "Authorization: Bearer VOTRE_TOKEN"
```

### Créer un serveur (Auth requis)
```bash
curl -X POST http://localhost:8080/api/servers \
  -H "Authorization: Bearer VOTRE_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Mon Serveur",
    "domain": "https://example.com",
    "description": "Description optionnelle"
  }'
```

## Dépannage

### Erreur de connexion MySQL
- Vérifiez que MySQL est lancé: `mysql -u root -p`
- Vérifiez les identifiants dans `.env`
- Vérifiez que la base existe: `SHOW DATABASES;`

### Port déjà utilisé
- Backend (8080): `lsof -ti:8080 | xargs kill -9` (macOS/Linux)
- Frontend (5173): Vite utilisera automatiquement un autre port

### Erreur CORS
- Vérifiez que `CORS_ORIGIN` dans `.env` du backend correspond à l'URL frontend
- Par défaut: `http://localhost:5173`

### Discord OAuth ne fonctionne pas
- Vérifiez les identifiants Discord dans `.env`
- Vérifiez que le Redirect URL correspond exactement
- L'authentification email/password fonctionne indépendamment

## Structure de développement

```
Terminal 1: Backend Rust
cd backend && cargo run

Terminal 2: Frontend Vue
cd frontend && npm run dev

Terminal 3: MySQL (si nécessaire)
mysql -u root -p

Terminal 4: Tests / Commandes
curl, git, etc.
```

## Prochaines étapes

1. ✅ Application fonctionnelle
2. 🔨 Ajoutez des tests unitaires
3. 🔨 Implémentez la vérification d'âge réelle
4. 🔨 Ajoutez plus de personnalisation des profils
5. 🔨 Créez une interface admin
6. 🔨 Déployez en production

## Ressources

- **Axum**: https://docs.rs/axum
- **SeaORM**: https://www.sea-ql.org/SeaORM
- **Vue 3**: https://vuejs.org
- **Pinia**: https://pinia.vuejs.org
- **Discord OAuth**: https://discord.com/developers/docs
