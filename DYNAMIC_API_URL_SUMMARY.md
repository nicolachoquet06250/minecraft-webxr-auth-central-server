# 🎯 Configuration dynamique de l'URL de l'API - Résumé

## ✅ Fonctionnalité implémentée

Le backend remplace **automatiquement** l'URL de l'API par défaut (`http://localhost:8080/api`) par la valeur de la variable d'environnement `DOMAIN` + `/api` au moment de servir les fichiers JavaScript.

## 📝 Changements effectués

### Fichiers modifiés

1. **`backend/src/static_files.rs`** (+30 lignes)
   - Ajout de la fonction `replace_api_url()`
   - Détection automatique des fichiers JavaScript
   - Remplacement de l'URL au runtime

2. **`backend/.env`** (+1 ligne)
   - Ajout de la variable `DOMAIN=http://localhost:8080`

### Code clé

```rust
// backend/src/static_files.rs
fn replace_api_url(content: &[u8]) -> Vec<u8> {
    let domain = env::var("DOMAIN")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    let api_url = format!("{}/api", domain);
    
    if let Ok(mut text) = String::from_utf8(content.to_vec()) {
        text = text.replace("http://localhost:8080/api", &api_url);
        text.into_bytes()
    } else {
        content.to_vec()
    }
}
```

## 🚀 Utilisation

### Développement
```bash
# backend/.env
DOMAIN=http://localhost:8080

cargo run
# → Frontend utilisera http://localhost:8080/api
```

### Production
```bash
# backend/.env
DOMAIN=https://auth.example.com

cargo run --release
# → Frontend utilisera https://auth.example.com/api
```

### Docker
```dockerfile
ENV DOMAIN=https://minecraft-auth.example.com
```

## 📊 Avantages

| Avantage | Description |
|----------|-------------|
| 🏗️ **Build unique** | Un seul build frontend pour tous les environnements |
| ⚡ **Déploiement rapide** | Changez juste la variable DOMAIN et redémarrez |
| 🔧 **Config centralisée** | Toute la configuration dans backend/.env |
| 🌐 **Multi-environnement** | Dev, staging, prod avec la même build |
| 📦 **Zero rebuild** | Pas besoin de rebuilder le frontend |

## 🔄 Workflow de déploiement

```bash
# 1. Build frontend (une seule fois)
cd frontend
npm run build

# 2. Configure DOMAIN pour l'environnement cible
cd ../backend
echo "DOMAIN=https://auth.example.com" > .env

# 3. Build backend (embarque le frontend)
cargo build --release

# 4. Deploy et run
./target/release/minecraft-auth-backend
```

## 🧪 Test

### Vérifier le remplacement

```bash
# 1. Lancer le serveur avec DOMAIN custom
cd backend
DOMAIN=https://test.example.com cargo run

# 2. Dans un autre terminal
curl http://localhost:8080/assets/index-*.js | grep -o "test.example.com/api"

# Attendu: "test.example.com/api" trouvé
```

### Test navigateur

1. Ouvrir http://localhost:8080
2. F12 → Sources → assets/index-[hash].js
3. Rechercher `/api`
4. Vérifier que l'URL correspond à DOMAIN

## ⚠️ Points importants

1. **Ordre de build** : Frontend d'abord, backend ensuite
   ```bash
   npm run build && cd ../backend && cargo build --release
   ```

2. **Cache navigateur** : Effacer le cache après changement de DOMAIN

3. **Valeur par défaut** : Doit rester `http://localhost:8080/api` dans le code

4. **Performance** : Le remplacement est fait à chaque requête mais les navigateurs cachent les JS

## 📚 Documentation complète

Voir `DYNAMIC_API_URL_CONFIG.md` pour :
- Explications détaillées
- Exemples de configuration
- Dépannage
- Limitations et optimisations

## 🎉 Résultat

✅ Application déployable sur n'importe quel domaine  
✅ Configuration simple via variable d'environnement  
✅ Pas de build spécifique par environnement  
✅ Compatible Docker, Kubernetes, etc.

**L'URL de l'API s'adapte automatiquement à l'environnement !** 🚀
