# Configuration AlwaysData pour Minecraft Auth Platform

## 📁 Structure des fichiers

```
/home/<votre-compte>/
├── minecraft-auth-backend/          # Dossier de l'application
│   ├── minecraft-auth-backend       # Binaire exécutable
│   ├── .env                        # Configuration
│   └── logs/                       # Logs (optionnel)
│       └── app.log
├── .ssh/
│   └── authorized_keys             # Clés SSH publiques autorisées
└── www/                            # Autres sites web (optionnel)
```

## 🔧 Configuration du site

### Via l'interface web

**Web** → **Sites** → **Installer une application**

```yaml
Type d'application: Custom application (Application personnalisée)

Configuration:
  Commande: /home/<votre-compte>/minecraft-auth-backend/minecraft-auth-backend
  Répertoire de travail: /home/<votre-compte>/minecraft-auth-backend
  
Addresses (Adresses):
  - minecraft-auth.alwaysdata.net
  # Ou votre domaine personnalisé:
  - auth.example.com

Variables d'environnement:
  DATABASE_URL: mysql://<user>:<password>@mysql-<compte>.alwaysdata.net:3306/<compte>_minecraft
  JWT_SECRET: <votre_secret_jwt>
  DOMAIN: https://minecraft-auth.alwaysdata.net
  API_PORT: 8080
  CORS_ORIGIN: https://minecraft-auth.alwaysdata.net
  ENVIRONMENT: production

Ports:
  # AlwaysData attribuera automatiquement un port
  # Utilisez ce port dans API_PORT ci-dessus
```

## 🗄️ Configuration MySQL

### Créer la base de données

**Databases** → **MySQL** → **Ajouter une base de données**

```
Nom: <compte>_minecraft
```

Notez les informations de connexion :
- **Host**: `mysql-<compte>.alwaysdata.net`
- **Port**: `3306`
- **User**: `<compte>` ou `<compte>_user`
- **Password**: (généré automatiquement)
- **Database**: `<compte>_minecraft`

### Connection string

```
mysql://<user>:<password>@mysql-<compte>.alwaysdata.net:3306/<compte>_minecraft
```

## 📝 Fichier .env sur le serveur

Connectez-vous via SSH et créez le fichier :

```bash
ssh <compte>@ssh-<compte>.alwaysdata.net
cd ~/minecraft-auth-backend
nano .env
```

Contenu du `.env` :

```env
# Database MySQL AlwaysData
DATABASE_URL=mysql://nicolachoquet:password@mysql-nicolachoquet.alwaysdata.net:3306/nicolachoquet_minecraft

# JWT Secret (générez une clé sécurisée !)
JWT_SECRET=votre_super_secret_jwt_key_changez_moi_en_production_123456789

# Discord OAuth (optionnel)
# DISCORD_CLIENT_ID=your_discord_client_id
# DISCORD_CLIENT_SECRET=your_discord_client_secret
# DISCORD_REDIRECT_URI=https://minecraft-auth.alwaysdata.net/auth/discord/callback

# Server
API_PORT=8080

# CORS - Autoriser votre domaine
CORS_ORIGIN=https://minecraft-auth.alwaysdata.net

# Domain - URL publique de votre API
DOMAIN=https://minecraft-auth.alwaysdata.net

# Environment
ENVIRONMENT=production
```

**Important** : Adaptez les valeurs avec vos propres informations !

## 🔐 Configuration SSH

### Ajouter votre clé SSH publique

1. Sur votre machine locale, générez une paire de clés :
```bash
ssh-keygen -t rsa -b 4096 -C "github-deploy" -f ~/.ssh/alwaysdata_deploy
```

2. Copiez la clé publique :
```bash
cat ~/.ssh/alwaysdata_deploy.pub
```

3. Dans AlwaysData :
   - **Account** → **SSH Keys** → **Add an SSH key**
   - Collez le contenu de la clé publique

4. Testez la connexion :
```bash
ssh -i ~/.ssh/alwaysdata_deploy <compte>@ssh-<compte>.alwaysdata.net
```

## 🚀 Premier déploiement manuel

Avant de configurer GitHub Actions, testez le déploiement manuel :

```bash
# 1. Sur votre machine locale, buildez l'application
cd frontend
npm run build

cd ../backend
cargo build --release

# 2. Uploadez le binaire
scp ../target/release/minecraft-auth-backend <compte>@ssh-<compte>.alwaysdata.net:~/minecraft-auth-backend/

# 3. Connectez-vous et configurez
ssh <compte>@ssh-<compte>.alwaysdata.net

cd ~/minecraft-auth-backend
chmod +x minecraft-auth-backend

# 4. Créez le .env (voir section précédente)
nano .env

# 5. Testez l'exécution
./minecraft-auth-backend
```

## 🔄 Redémarrage du site

### Via l'API REST

```bash
# Avec curl
curl -X POST \
  -u "<compte>:<api-token>" \
  "https://api.alwaysdata.com/v1/site/<site-id>/restart/"

# Exemple avec des vraies valeurs
curl -X POST \
  -u "nicolachoquet:abc123token456" \
  "https://api.alwaysdata.com/v1/site/123456/restart/"
```

### Via l'interface web

1. **Web** → **Sites**
2. Cliquez sur votre site
3. Cliquez sur le bouton **⟳ Restart**

### Via SSH

```bash
ssh <compte>@ssh-<compte>.alwaysdata.net
killall minecraft-auth-backend

# Le site redémarrera automatiquement via le superviseur AlwaysData
```

## 📊 Logs et monitoring

### Voir les logs

```bash
ssh <compte>@ssh-<compte>.alwaysdata.net

# Logs de l'application (si vous loggez dans un fichier)
tail -f ~/minecraft-auth-backend/logs/app.log

# Logs du système AlwaysData
# Web → Sites → Votre site → Logs
```

### Variables d'environnement pour les logs

Ajoutez dans votre site AlwaysData :

```env
RUST_LOG=info
RUST_BACKTRACE=1
```

## 🌐 Domaine personnalisé

### Ajouter votre domaine

1. **Domains** → **Add a domain**
2. Ajoutez votre domaine (ex: `auth.example.com`)
3. Configurez les DNS chez votre registrar :

```
Type: CNAME
Name: auth (ou @)
Value: <compte>.alwaysdata.net.
```

4. Associez le domaine à votre site :
   - **Web** → **Sites** → Votre site → **Addresses**
   - Ajoutez `auth.example.com`

5. Activez HTTPS :
   - **SSL/TLS** → **Add a certificate**
   - Choisissez **Let's Encrypt**

6. Mettez à jour votre `.env` :
```env
DOMAIN=https://auth.example.com
CORS_ORIGIN=https://auth.example.com
```

## 🔒 Sécurité

### Bonnes pratiques

1. **JWT_SECRET** : Utilisez une clé très longue et aléatoire
```bash
openssl rand -hex 64
```

2. **Permissions fichiers** :
```bash
chmod 600 ~/.minecraft-auth-backend/.env
chmod 700 ~/.minecraft-auth-backend/minecraft-auth-backend
```

3. **Base de données** : Utilisez un utilisateur avec permissions limitées

4. **HTTPS** : Toujours utiliser HTTPS en production avec Let's Encrypt

5. **CORS** : Limitez aux domaines nécessaires

## 📋 Checklist de configuration

- [ ] Base de données MySQL créée
- [ ] Site/application configuré dans AlwaysData
- [ ] Dossier `~/minecraft-auth-backend` créé
- [ ] Clé SSH publique ajoutée
- [ ] Token API généré
- [ ] Fichier `.env` configuré sur le serveur
- [ ] Premier déploiement manuel testé
- [ ] Application accessible via navigateur
- [ ] Domaine personnalisé configuré (optionnel)
- [ ] HTTPS activé avec Let's Encrypt
- [ ] Secrets GitHub configurés
- [ ] Premier déploiement automatique testé

## 🆘 Support

- Documentation AlwaysData: https://help.alwaysdata.com
- Support AlwaysData: https://admin.alwaysdata.com/support/
- API Documentation: https://help.alwaysdata.com/en/api/

---

**Configuration terminée !** Votre application est prête pour le déploiement automatique via GitHub Actions. 🎉
