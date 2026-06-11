# 🚀 CI/CD GitHub Actions - Résumé

## ✅ Workflow créé

**Fichier :** `.github/workflows/deploy.yml`

### 📋 Étapes du workflow

1. **📥 Checkout** - Récupération du code
2. **🟢 Setup Node.js** - Configuration Node 20
3. **📦 Build Frontend** - `npm ci && npm run build`
4. **🦀 Setup Rust** - Configuration Rust 1.75
5. **🏗️ Build Backend** - `cargo build --release` (depuis racine)
6. **📦 Package** - Création de `minecraft-auth-backend.tar.gz`
7. **🚀 Deploy SSH** - Upload sur AlwaysData via SSH
8. **🔄 Restart** - Redémarrage via API AlwaysData
9. **🏥 Health Check** - Vérification que l'app fonctionne
10. **📢 Notification** - Notification du résultat

### 🔄 Déclenchement

- ✅ Push sur `main` ou `production`
- ✅ Déclenchement manuel (workflow_dispatch)

---

## 🔐 Secrets GitHub requis

Configurez ces secrets dans **Settings** → **Secrets and variables** → **Actions** :

| Secret | Description | Exemple |
|--------|-------------|---------|
| `ALWAYSDATA_SSH_KEY` | Clé SSH privée | `-----BEGIN OPENSSH...` |
| `ALWAYSDATA_HOST` | Hostname SSH | `ssh-nicolachoquet.alwaysdata.net` |
| `ALWAYSDATA_USER` | Username SSH | `nicolachoquet` |
| `ALWAYSDATA_DEPLOY_PATH` | Chemin déploiement | `/home/nicolachoquet/minecraft-auth-backend` |
| `ALWAYSDATA_API_KEY` | Token API | `abc123...` |
| `ALWAYSDATA_ACCOUNT` | Nom du compte | `nicolachoquet` |
| `ALWAYSDATA_SITE_ID` | ID du site | `123456` |
| `APP_URL` | URL de l'app | `https://minecraft-auth.alwaysdata.net` |

**Documentation complète :** Voir `.github/DEPLOYMENT_SETUP.md`

---

## 🎯 Fonctionnalités

### ✅ Build optimisé
- Frontend buildé en mode production
- Backend compilé en `--release`
- Caching des dépendances Cargo

### ✅ Déploiement sécurisé
- Upload via SSH avec clé privée
- Backup automatique de la version précédente
- Package compressé pour transfert rapide

### ✅ Rollback automatique
- Job séparé pour rollback manuel
- Restauration de la dernière version fonctionnelle
- Protection avec environment `rollback`

### ✅ Health check
- 5 tentatives avec délai de 5s
- Vérification HTTP 200/301/302
- Échec si l'app ne répond pas

---

## 📦 Structure du déploiement

### Sur AlwaysData
```
/home/<compte>/minecraft-auth-backend/
├── minecraft-auth-backend           # Binaire actif
├── minecraft-auth-backend.backup.*  # Backups horodatés
├── .env                            # Variables d'environnement
└── logs/                           # Logs application (optionnel)
```

### Package déployé
```
minecraft-auth-backend.tar.gz
├── minecraft-auth-backend  # Binaire
└── .env                   # Config (si .env.production existe)
```

---

## 🚀 Utilisation

### Déploiement automatique

```bash
# Pousser sur main
git add .
git commit -m "feat: nouvelle fonctionnalité"
git push origin main

# → Le workflow se déclenche automatiquement
```

### Déploiement manuel

1. GitHub → **Actions**
2. Sélectionner **Build and Deploy to AlwaysData**
3. Cliquer **Run workflow**
4. Sélectionner la branche
5. Cliquer **Run workflow**

### Rollback

1. GitHub → **Actions**
2. Sélectionner **Build and Deploy to AlwaysData**
3. Cliquer **Run workflow**
4. Aller dans l'onglet **rollback**
5. Confirmer le rollback

---

## 🔧 Configuration AlwaysData

### 1. Créer le site

**Web** → **Sites** → **Add a site**

```
Type: Custom application
Command: /home/<compte>/minecraft-auth-backend/minecraft-auth-backend
Working directory: /home/<compte>/minecraft-auth-backend
Addresses: minecraft-auth.alwaysdata.net
Environment variables:
  DATABASE_URL=mysql://...
  JWT_SECRET=...
  DOMAIN=https://minecraft-auth.alwaysdata.net
  API_PORT=8080
  CORS_ORIGIN=https://minecraft-auth.alwaysdata.net
```

### 2. Créer la base de données

**Databases** → **MySQL** → **Add a database**

```
Name: <compte>_minecraft
```

### 3. Générer le token API

**Profile** → **API** → **Generate a new token**

---

## 📊 Statistiques

**Temps de build estimé :**
- Frontend: ~2 min
- Backend: ~5-8 min (première fois avec cache)
- Backend: ~2-3 min (builds suivants)
- Déploiement: ~1 min
- **Total: ~6-12 min**

**Taille du package :**
- Frontend dist: ~400 KB
- Backend binary: ~50-80 MB (release)
- Package compressé: ~15-20 MB

---

## 🐛 Dépannage

### Erreur: "Permission denied (publickey)"
**Solution :** Vérifier que `ALWAYSDATA_SSH_KEY` contient la clé PRIVÉE complète

### Erreur: "401 Unauthorized" (API)
**Solution :** Régénérer le token API dans AlwaysData

### Erreur: "Health check failed"
**Solution :** Vérifier les logs sur le serveur :
```bash
ssh <user>@<host>
cd ~/minecraft-auth-backend
tail -f logs/app.log
```

### Build backend échoue
**Solution :** Vérifier que `frontend/dist` existe avant le build backend

---

## 📚 Fichiers créés

- `.github/workflows/deploy.yml` - Workflow principal (340 lignes)
- `.github/DEPLOYMENT_SETUP.md` - Guide de configuration détaillé
- `backend/.env.example` - Template de configuration

---

## 🎉 Avantages

✅ **Déploiement automatique** - Push et c'est déployé  
✅ **Rollback facile** - Un clic pour revenir en arrière  
✅ **Health check** - Assurance que l'app fonctionne  
✅ **Backup automatique** - Versions précédentes sauvegardées  
✅ **Cache optimisé** - Builds plus rapides  
✅ **Déploiement sécurisé** - SSH + API tokens  

**Votre CI/CD est prête pour la production !** 🚀
