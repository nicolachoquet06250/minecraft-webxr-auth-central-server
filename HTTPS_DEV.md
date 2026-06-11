# Configuration HTTPS en développement

## Configuration effectuée

### Backend (Rust/Axum)
- ✅ Ajout de `axum-server` avec support TLS/Rustls
- ✅ Configuration conditionnelle HTTPS via variable d'environnement `USE_HTTPS`
- ✅ Support des certificats auto-signés

### Frontend (Vue/Vite)
- ✅ Plugin `vite-plugin-mkcert` déjà configuré (génère automatiquement les certificats)
- ✅ HTTPS activé par défaut

## Mise en place

### 1. Générer les certificats pour le backend

**Windows avec PowerShell (recommandé) :**
```powershell
.\generate-certs.ps1
```

**Avec OpenSSL (Linux/Mac ou Windows avec OpenSSL installé) :**
```bash
# Windows
.\generate-certs.bat

# Linux/Mac
chmod +x generate-certs.sh
./generate-certs.sh
```

Le script PowerShell utilise `mkcert` qui est déjà installé avec le frontend (via vite-plugin-mkcert).

Cela créera :
- `backend/certs/cert.pem` : Certificat SSL
- `backend/certs/key.pem` : Clé privée

### 2. Configuration des variables d'environnement

Le fichier `backend/.env` est déjà configuré :
```env
USE_HTTPS=true
SSL_CERT_PATH=./certs/cert.pem
SSL_KEY_PATH=./certs/key.pem
CORS_ORIGIN=https://localhost:5173
```

Le fichier `frontend/.env` est configuré :
```env
VITE_API_URL=https://localhost:8080/api
```

### 3. Lancer les serveurs

**Terminal 1 - Backend :**
```bash
cd backend
cargo run
```
➡️ Serveur backend : https://localhost:8080

**Terminal 2 - Frontend :**
```bash
cd frontend
npm run dev
```
➡️ Serveur frontend : https://localhost:5173

## Accepter les certificats auto-signés

Au premier lancement, votre navigateur affichera un avertissement de sécurité car les certificats sont auto-signés. 

**Pour Chrome/Edge :**
1. Cliquez sur "Avancé"
2. Cliquez sur "Continuer vers localhost (non sécurisé)"

**Pour Firefox :**
1. Cliquez sur "Avancé"
2. Cliquez sur "Accepter le risque et continuer"

**Note :** Le plugin `vite-plugin-mkcert` installe automatiquement le certificat racine dans votre système pour éviter ces avertissements.

## Désactiver HTTPS (retour à HTTP)

Si vous voulez revenir à HTTP pour le développement :

**backend/.env :**
```env
USE_HTTPS=false
CORS_ORIGIN=http://localhost:5173
```

**frontend/.env :**
```env
VITE_API_URL=http://localhost:8080/api
```

## Production

En production, utilisez de vrais certificats (Let's Encrypt, etc.) et configurez les chemins :

```env
USE_HTTPS=true
SSL_CERT_PATH=/etc/letsencrypt/live/yourdomain.com/fullchain.pem
SSL_KEY_PATH=/etc/letsencrypt/live/yourdomain.com/privkey.pem
```

## Troubleshooting

### Erreur "Certificate verify failed"
- Assurez-vous que les certificats sont générés correctement
- Vérifiez les chemins dans `.env`

### Erreur CORS
- Vérifiez que `CORS_ORIGIN` correspond à l'URL du frontend (https://localhost:5173)
- Assurez-vous que le frontend et le backend utilisent tous les deux HTTPS

### Certificats Vite non reconnus
```bash
cd frontend
npx vite-plugin-mkcert install
```
