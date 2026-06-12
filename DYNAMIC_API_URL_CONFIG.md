# 🔧 Configuration dynamique de l'URL de l'API

## 📝 Vue d'ensemble

Le backend remplace automatiquement l'URL de l'API par défaut dans les fichiers JavaScript du frontend au moment de les servir. Cela permet de déployer l'application sur n'importe quel domaine sans avoir à rebuilder le frontend.

## ⚙️ Fonctionnement

### 1. Backend (.env)

```env
# URL complète du domaine (avec protocole et port si nécessaire)
DOMAIN=http://localhost:8080

# Exemples pour différents environnements:
# DOMAIN=https://api.example.com
# DOMAIN=https://voxicraft-auth.example.com:8443
```

### 2. Frontend (valeur par défaut dans le code)

```typescript
// frontend/src/api/index.ts
const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
```

Cette URL **sera remplacée automatiquement** par `{DOMAIN}/api` au runtime.

### 3. Remplacement automatique (backend/src/static_files.rs)

Lorsqu'un fichier JavaScript est servi :

```rust
fn replace_api_url(content: &[u8]) -> Vec<u8> {
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_url = format!("{}/api", domain);
    
    // Remplace "http://localhost:8080/api" par "{DOMAIN}/api"
    text = text.replace("http://localhost:8080/api", &api_url);
}
```

## 🚀 Utilisation

### Développement local

```env
# backend/.env
DOMAIN=http://localhost:8080
```

**Résultat** : Le frontend utilisera `http://localhost:8080/api`

### Production (domaine personnalisé)

```env
# backend/.env (sur le serveur de production)
DOMAIN=https://auth.example.com
```

**Résultat** : Le frontend utilisera automatiquement `https://auth.example.com/api`

### Production (sous-domaine)

```env
# backend/.env
DOMAIN=https://api.voxicraft.com
```

**Résultat** : Le frontend utilisera `https://api.voxicraft.com/api`

### Production (avec port personnalisé)

```env
# backend/.env
DOMAIN=https://voxicraft-auth.example.com:8443
```

**Résultat** : Le frontend utilisera `https://voxicraft-auth.example.com:8443/api`

## 🔍 Vérification

### 1. Vérifier l'URL dans le frontend buildé

```bash
cd frontend/dist/assets
grep -o "http://localhost:8080/api" *.js
```

**Attendu** : La chaîne doit être trouvée (c'est la valeur par défaut)

### 2. Lancer le backend et tester

```bash
# Terminal 1 : Backend
cd backend
DOMAIN=https://production.example.com cargo run

# Terminal 2 : Tester
curl http://localhost:8080/assets/index-*.js | grep -o "production.example.com/api"
```

**Attendu** : Vous devriez voir `production.example.com/api` au lieu de `localhost:8080/api`

### 3. Vérifier dans le navigateur

1. Ouvrir les DevTools (F12)
2. Onglet **Sources** → `assets/` → `index-[hash].js`
3. Rechercher (Ctrl+F) : `api_url` ou `/api`
4. Vérifier que l'URL correspond à la variable `DOMAIN`

## 📊 Avantages

✅ **Un seul build** : Le frontend est buildé une seule fois, sans configuration spécifique à l'environnement

✅ **Déploiement flexible** : Changez simplement la variable `DOMAIN` dans le `.env` du backend

✅ **Pas de rebuild** : Inutile de rebuilder le frontend pour chaque environnement (dev, staging, prod)

✅ **Configuration centralisée** : Toute la configuration est dans le `.env` du backend

✅ **Zero downtime** : Changez la variable `DOMAIN` et redémarrez le backend, c'est tout

## ⚠️ Limitations et considérations

### 1. Performance

Le remplacement se fait **à chaque requête** pour les fichiers JavaScript. Pour optimiser :

- ✅ Les navigateurs **mettront en cache** les fichiers JS
- ✅ Le remplacement est très rapide (simple `String::replace`)
- ⚠️ Pour de meilleures performances en production, envisagez de :
  - Mettre en cache le résultat du remplacement
  - Utiliser un CDN
  - Activer la compression gzip/brotli

### 2. Restriction du remplacement

Le remplacement ne fonctionne que pour :
- ✅ Fichiers avec MIME type `application/javascript` ou `text/javascript`
- ✅ La chaîne exacte `http://localhost:8080/api`

### 3. Valeur par défaut obligatoire

La valeur par défaut dans le code frontend **doit être** :
```
'http://localhost:8080/api'
```

Si vous la changez, pensez à modifier aussi `static_files.rs` :
```rust
text.replace("NOUVELLE_VALEUR_PAR_DEFAUT", &api_url);
```

## 🔧 Dépannage

### Le frontend utilise toujours localhost:8080

**Causes possibles** :

1. **Cache navigateur** : Effacez le cache (Ctrl+Shift+Delete) ou mode incognito
2. **Variable DOMAIN non définie** : Vérifiez que `DOMAIN` existe dans `.env`
3. **Backend non redémarré** : Redémarrez le backend après avoir modifié `.env`
4. **Fichier JS non minifié** : Le remplacement fonctionne sur les fichiers buildés

**Solution** :

```bash
# 1. Vérifier la variable
cd backend
grep DOMAIN .env

# 2. Rebuilder le frontend
cd ../frontend
npm run build

# 3. Rebuilder le backend (important !)
cd ../backend
cargo build --release

# 4. Lancer le backend
cargo run --release
```

### Le remplacement ne fonctionne pas en production

**Cause** : RustEmbed embarque les fichiers au moment de la compilation

**Solution** : **Rebuilder le backend après avoir buildé le frontend**

```bash
# Ordre correct :
cd frontend
npm run build

cd ../backend
cargo build --release  # Ceci embarque les fichiers de frontend/dist
```

## 📚 Exemples de configuration

### Configuration Docker

```dockerfile
# Dockerfile
ENV DOMAIN=https://auth.example.com
```

### Configuration avec Nginx reverse proxy

```nginx
# nginx.conf
location / {
    proxy_pass http://backend:8080;
}
```

```env
# backend/.env
DOMAIN=https://voxicraft-auth.example.com
```

### Configuration multi-environnement

```bash
# .env.development
DOMAIN=http://localhost:8080

# .env.staging
DOMAIN=https://staging-auth.example.com

# .env.production
DOMAIN=https://auth.example.com
```

Puis au lancement :
```bash
# Staging
cargo run --release -- --env-file .env.staging

# Production
cargo run --release -- --env-file .env.production
```

## 🎯 Résumé

| Étape | Action | Fichier concerné |
|-------|--------|------------------|
| 1 | Build frontend | `npm run build` |
| 2 | Configurer DOMAIN | `backend/.env` |
| 3 | Build backend | `cargo build --release` |
| 4 | Lancer backend | `cargo run --release` |
| 5 | ✅ Frontend utilise automatiquement DOMAIN | - |

**Important** : Le backend doit être **rebuild après le frontend** pour embarquer les fichiers buildés.
