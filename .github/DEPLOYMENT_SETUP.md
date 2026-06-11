# 🚀 Configuration GitHub Actions - Déploiement AlwaysData

## 📝 Vue d'ensemble

Ce guide explique comment configurer les secrets GitHub nécessaires pour le déploiement automatique sur AlwaysData.

## 🔐 Secrets GitHub requis

Allez dans **Settings** → **Secrets and variables** → **Actions** → **New repository secret**

### 1. SSH Configuration

#### `ALWAYSDATA_SSH_KEY`
**Clé SSH privée** pour se connecter au serveur AlwaysData.

**Comment obtenir :**
```bash
# Sur votre machine locale
ssh-keygen -t rsa -b 4096 -C "github-actions@minecraft-auth" -f ~/.ssh/alwaysdata_deploy

# Copier la clé publique sur AlwaysData
cat ~/.ssh/alwaysdata_deploy.pub
# → Copier ce contenu dans AlwaysData : Account → SSH Keys

# Copier la clé PRIVÉE dans GitHub Secrets
cat ~/.ssh/alwaysdata_deploy
# → Copier TOUT le contenu (y compris BEGIN/END) dans le secret GitHub
```

**Format attendu :**
```
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAA...
...
-----END OPENSSH PRIVATE KEY-----
```

---

#### `ALWAYSDATA_HOST`
**Hostname SSH** de votre compte AlwaysData.

**Valeur :**
```
ssh-<votre-compte>.alwaysdata.net
```

**Exemple :**
```
ssh-nicolachoquet.alwaysdata.net
```

---

#### `ALWAYSDATA_USER`
**Nom d'utilisateur SSH** AlwaysData.

**Valeur :**
```
<votre-compte>
```

**Exemple :**
```
nicolachoquet
```

---

#### `ALWAYSDATA_DEPLOY_PATH`
**Chemin absolu** où déployer l'application sur le serveur.

**Valeur recommandée :**
```
/home/<votre-compte>/minecraft-auth-backend
```

**Exemple :**
```
/home/nicolachoquet/minecraft-auth-backend
```

**Note :** Créez ce dossier sur AlwaysData avant le premier déploiement :
```bash
ssh <votre-compte>@ssh-<votre-compte>.alwaysdata.net
mkdir -p ~/minecraft-auth-backend
```

---

### 2. AlwaysData API Configuration

#### `ALWAYSDATA_API_KEY`
**Token API** pour contrôler votre compte via l'API REST.

**Comment obtenir :**
1. Connectez-vous sur [https://admin.alwaysdata.com](https://admin.alwaysdata.com)
2. Allez dans **Profile** → **API**
3. Cliquez sur **Generate a new token**
4. Copiez le token généré

**Format :**
```
abc123def456ghi789jkl012mno345pqr678stu901vwx234yz
```

---

#### `ALWAYSDATA_ACCOUNT`
**Nom du compte** AlwaysData (même que `ALWAYSDATA_USER` généralement).

**Valeur :**
```
<votre-compte>
```

---

#### `ALWAYSDATA_SITE_ID`
**ID du site** dans AlwaysData qui exécute l'application.

**Comment obtenir :**
1. Via l'API AlwaysData :
```bash
curl -u "<compte>:<api-key>" https://api.alwaysdata.com/v1/site/
```

2. Ou via l'interface web :
   - Allez dans **Web** → **Sites**
   - L'ID est visible dans l'URL : `/site/<ID>/`

**Format :**
```
123456
```

---

### 3. Application Configuration

#### `APP_URL`
**URL publique** de votre application pour le health check.

**Valeur :**
```
https://minecraft-auth.alwaysdata.net
```

Ou votre domaine personnalisé :
```
https://auth.example.com
```

---

#### `DATABASE_URL` (optionnel)
**Connection string MySQL** si vous exécutez des migrations depuis GitHub Actions.

**Format :**
```
mysql://user:password@host:port/database
```

**Exemple :**
```
mysql://nicolachoquet:MyP@ssw0rd@mysql-nicolachoquet.alwaysdata.net:3306/nicolachoquet_minecraft
```

**⚠️ Note :** Ce secret est optionnel. Les migrations peuvent aussi être exécutées directement sur le serveur.

---

## 📋 Récapitulatif des secrets

| Secret | Description | Exemple |
|--------|-------------|---------|
| `ALWAYSDATA_SSH_KEY` | Clé privée SSH | `-----BEGIN OPENSSH...` |
| `ALWAYSDATA_HOST` | Hostname SSH | `ssh-nicolachoquet.alwaysdata.net` |
| `ALWAYSDATA_USER` | Username SSH | `nicolachoquet` |
| `ALWAYSDATA_DEPLOY_PATH` | Chemin déploiement | `/home/nicolachoquet/minecraft-auth-backend` |
| `ALWAYSDATA_API_KEY` | Token API | `abc123...` |
| `ALWAYSDATA_ACCOUNT` | Nom du compte | `nicolachoquet` |
| `ALWAYSDATA_SITE_ID` | ID du site | `123456` |
| `APP_URL` | URL de l'app | `https://minecraft-auth.alwaysdata.net` |
| `DATABASE_URL` | MySQL URL (optionnel) | `mysql://user:pass@host/db` |

---

## 🔧 Configuration sur AlwaysData

### 1. Créer le site/application

1. **Web** → **Sites** → **Add a site**
2. Configuration :
   - **Type:** Custom application
   - **Command:** `/home/<compte>/minecraft-auth-backend/minecraft-auth-backend`
   - **Working directory:** `/home/<compte>/minecraft-auth-backend`
   - **Port:** Attribué automatiquement (notez-le)
   - **Environment variables:**
     ```
     DATABASE_URL=mysql://...
     JWT_SECRET=your_secret
     DOMAIN=https://minecraft-auth.alwaysdata.net
     API_PORT=<port-attribué>
     ```

### 2. Créer la base de données

1. **Databases** → **MySQL**
2. Créer une nouvelle base : `<compte>_minecraft`
3. Notez les credentials

### 3. Configurer le fichier .env sur le serveur

```bash
ssh <compte>@ssh-<compte>.alwaysdata.net

cd ~/minecraft-auth-backend
nano .env
```

Contenu :
```env
DATABASE_URL=mysql://user:pass@mysql-nicolachoquet.alwaysdata.net:3306/nicolachoquet_minecraft
JWT_SECRET=your_super_secret_key_here
DOMAIN=https://minecraft-auth.alwaysdata.net
API_PORT=8080
CORS_ORIGIN=https://minecraft-auth.alwaysdata.net
```

---

## 🚀 Déploiement

### Déploiement automatique

Le workflow se déclenche automatiquement sur :
- Push sur `main` ou `production`
- Déclenchement manuel via l'interface GitHub

### Déploiement manuel

1. Allez dans **Actions** → **Build and Deploy to AlwaysData**
2. Cliquez sur **Run workflow**
3. Sélectionnez la branche
4. Cliquez **Run workflow**

---

## 🔄 Workflow étapes

1. ✅ Checkout du code
2. 📦 Build du frontend (npm)
3. 🦀 Build du backend (cargo release)
4. 📦 Création du package de déploiement
5. 🚀 Upload via SSH sur AlwaysData
6. 🔄 Redémarrage via API AlwaysData
7. 🏥 Health check de l'application
8. 📢 Notification du résultat

---

## 🐛 Dépannage

### ❌ Erreur SSH "Permission denied"

**Cause :** Clé SSH non configurée correctement.

**Solution :**
1. Vérifier que la clé publique est bien ajoutée dans AlwaysData
2. Vérifier que le secret `ALWAYSDATA_SSH_KEY` contient la clé PRIVÉE complète
3. Tester manuellement :
   ```bash
   ssh -i ~/.ssh/alwaysdata_deploy <user>@<host>
   ```

---

### ❌ Erreur API "401 Unauthorized"

**Cause :** Token API invalide.

**Solution :**
1. Régénérer un nouveau token dans AlwaysData
2. Mettre à jour le secret `ALWAYSDATA_API_KEY`

---

### ❌ Health check failed

**Cause :** L'application ne démarre pas ou met trop de temps.

**Solution :**
1. Vérifier les logs sur AlwaysData :
   ```bash
   ssh <user>@<host>
   cd ~/minecraft-auth-backend
   tail -f logs/app.log
   ```
2. Vérifier que le `.env` est correct
3. Vérifier que la base de données est accessible

---

### ❌ Site not found (404)

**Cause :** Le site n'est pas configuré ou arrêté.

**Solution :**
1. Vérifier dans **Web** → **Sites** que le site existe
2. Vérifier que le site est **activé**
3. Vérifier le `ALWAYSDATA_SITE_ID` dans les secrets

---

## 📚 Ressources

- [Documentation AlwaysData SSH](https://help.alwaysdata.com/en/remote-access/ssh/)
- [Documentation AlwaysData API](https://help.alwaysdata.com/en/api/)
- [Documentation GitHub Actions](https://docs.github.com/en/actions)

---

## 🎉 Checklist de configuration

- [ ] Clé SSH générée et ajoutée dans AlwaysData
- [ ] Tous les secrets GitHub configurés
- [ ] Dossier de déploiement créé sur AlwaysData
- [ ] Site/application configuré sur AlwaysData
- [ ] Base de données MySQL créée
- [ ] Fichier `.env` configuré sur le serveur
- [ ] Premier déploiement testé
- [ ] Health check réussi

**Une fois tous ces points cochés, le déploiement automatique est prêt !** 🚀
