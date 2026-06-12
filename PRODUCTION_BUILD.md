# Build en Production

## Fonctionnement

Le binaire de production embarque le frontend compilé à l'intérieur grâce à `rust-embed`. Cela permet d'avoir un seul exécutable qui sert à la fois l'API et l'interface web.

## Structure

```
voxicraft-auth-platform/
├── frontend/
│   ├── dist/          # Build du frontend (généré par Vite)
│   └── ...
└── backend/
    ├── src/
    │   ├── static_files.rs  # Module pour servir le frontend embarqué
    │   └── main.rs
    └── Cargo.toml
```

## Build en production

### Méthode 1 : Script automatique

**Windows :**
```bash
.\build.bat
```

**Linux/Mac :**
```bash
chmod +x build.sh
./build.sh
```

### Méthode 2 : Manuelle

```bash
# 1. Builder le frontend
cd frontend
npm install
npm run build

# 2. Builder le backend (avec le frontend embarqué)
cd ../backend
cargo build --release
```

Le binaire sera disponible à : `backend/target/release/voxicraft-auth-backend[.exe]`

## Développement

En développement, vous pouvez :

1. **Frontend séparé** : Lancer le frontend avec `npm run dev` dans `frontend/`
2. **Backend seul** : Lancer le backend avec `cargo run` dans `backend/`

Le CORS est configuré pour permettre au frontend (port 5173) de communiquer avec le backend (port 8080).

## Routes

- `/api/*` : Routes de l'API
- `/*` : Toutes les autres routes servent le frontend
  - Fichiers statiques : CSS, JS, images (avec MIME type correct)
  - Routes SPA : Retourne `index.html` pour le routing Vue Router

## Variables d'environnement en production

Le fichier `.env` doit être présent dans le même répertoire que le binaire ou vous devez définir les variables d'environnement :

```env
DATABASE_URL=mysql://user:password@localhost:3306/voxicraft_xr_central
JWT_SECRET=votre_secret_jwt
API_PORT=8080
CORS_ORIGIN=*
```

## Déploiement

1. Builder le projet avec `build.bat` ou `build.sh`
2. Copier le binaire `backend/target/release/voxicraft-auth-backend[.exe]`
3. Créer un fichier `.env` avec les variables de production
4. Lancer le binaire : `./voxicraft-auth-backend`

Le serveur écoute sur `http://0.0.0.0:8080` (configurable via `API_PORT`)
