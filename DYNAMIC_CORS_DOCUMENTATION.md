# 🔐 CORS Dynamique - Documentation

## 📝 Vue d'ensemble

Le backend utilise maintenant un **middleware CORS dynamique** qui autorise automatiquement les requêtes cross-origin provenant de :

1. **Frontend principal** : Configuré via `CORS_ORIGIN` dans `.env` (par défaut `http://localhost:5176`)
2. **Serveurs enregistrés actifs** : Tous les domaines `relay_domain` et `game_domain` des serveurs marqués comme `is_active = true` dans la base de données

## 🎯 Avantages

- ✅ **Sécurité renforcée** : Seuls les domaines légitimes et enregistrés peuvent accéder à l'API
- ✅ **Configuration automatique** : Pas besoin de modifier le code pour ajouter de nouveaux domaines
- ✅ **Gestion centralisée** : Les domaines autorisés sont gérés via la base de données
- ✅ **Support multi-domaines** : Chaque serveur peut avoir un domaine de relais et un domaine de jeu différents

## 🔧 Configuration

### Variables d'environnement (.env)

```env
# Frontend principal (toujours autorisé)
CORS_ORIGIN=http://localhost:5176

# Autres variables...
DATABASE_URL=mysql://user:password@localhost:3306/minecraft_xr_central
JWT_SECRET=your_secret_key
API_PORT=8080
```

### Base de données

Les serveurs actifs sont automatiquement pris en compte :

```sql
-- Exemple de serveurs autorisés
SELECT id, name, relay_domain, game_domain, is_active 
FROM server 
WHERE is_active = true;

+------+---------------+----------------------------+---------------------------+-----------+
| id   | name          | relay_domain               | game_domain               | is_active |
+------+---------------+----------------------------+---------------------------+-----------+
| srv1 | Mon Serveur 1 | https://relay.example.com  | https://game.example.com  | 1         |
| srv2 | Mon Serveur 2 | https://relay2.example.com | https://game2.example.com | 1         |
+------+---------------+----------------------------+---------------------------+-----------+
```

**Domaines autorisés dans cet exemple :**
- `http://localhost:5176` (frontend)
- `https://relay.example.com`
- `https://game.example.com`
- `https://relay2.example.com`
- `https://game2.example.com`

## 🏗️ Architecture

### Middleware CORS (`backend/src/middleware/cors.rs`)

```rust
pub async fn dynamic_cors_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode>
```

**Fonctionnement :**

1. **Extraction de l'origine** : Récupère le header `Origin` de la requête
2. **Chargement des domaines** : Requête DB pour récupérer tous les serveurs actifs
3. **Parsing des URLs** : Extrait les origines (`protocol://host`) de chaque `relay_domain` et `game_domain`
4. **Vérification** : Compare l'origine de la requête avec la liste autorisée
5. **Headers CORS** : Ajoute les headers appropriés si l'origine est autorisée

### Headers CORS ajoutés

**Requêtes OPTIONS (preflight) :**
```http
Access-Control-Allow-Origin: <origin>
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS, PATCH
Access-Control-Allow-Headers: authorization, content-type, accept
Access-Control-Allow-Credentials: true
Access-Control-Max-Age: 3600
```

**Requêtes normales :**
```http
Access-Control-Allow-Origin: <origin>
Access-Control-Allow-Credentials: true
Access-Control-Expose-Headers: content-type, authorization
```

## 🚀 Utilisation

### 1. Enregistrer un nouveau serveur

```bash
curl -X POST http://localhost:8080/api/servers \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Mon Nouveau Serveur",
    "relay_domain": "https://relay.monserveur.com",
    "game_domain": "https://game.monserveur.com",
    "description": "Description",
    "is_active": true
  }'
```

**Résultat :** Les domaines `https://relay.monserveur.com` et `https://game.monserveur.com` sont **immédiatement autorisés** pour les requêtes CORS.

### 2. Désactiver un serveur

```bash
curl -X PUT http://localhost:8080/api/servers/<id> \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "is_active": false
  }'
```

**Résultat :** Les domaines du serveur sont **immédiatement révoqués** du CORS.

### 3. Tester depuis un serveur enregistré

```javascript
// Depuis https://game.monserveur.com
fetch('http://localhost:8080/api/servers', {
  method: 'GET',
  headers: {
    'Authorization': 'Bearer <token>',
    'Content-Type': 'application/json',
  },
  credentials: 'include', // Important pour CORS avec credentials
})
.then(response => response.json())
.then(data => console.log(data))
.catch(error => console.error('Erreur CORS:', error));
```

## 🧪 Tests

### Test manuel avec curl

```bash
# Requête OPTIONS (preflight)
curl -X OPTIONS http://localhost:8080/api/servers \
  -H "Origin: https://game.example.com" \
  -H "Access-Control-Request-Method: GET" \
  -H "Access-Control-Request-Headers: authorization" \
  -v

# Requête GET normale
curl -X GET http://localhost:8080/api/servers \
  -H "Origin: https://game.example.com" \
  -H "Authorization: Bearer <token>" \
  -v
```

### Test depuis le navigateur

```javascript
// Console du navigateur depuis n'importe quelle page
const testCORS = async () => {
  try {
    const response = await fetch('http://localhost:8080/api/servers/public-id', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    const data = await response.json();
    console.log('✅ CORS OK:', data);
  } catch (error) {
    console.error('❌ CORS Bloqué:', error);
  }
};

testCORS();
```

## 🔍 Debugging

### Logs du serveur

Le middleware log automatiquement les erreurs :

```rust
tracing::error!("Failed to fetch servers for CORS: {}", e);
```

### Vérifier les domaines autorisés

```bash
# Depuis MySQL
mysql -u root -p minecraft_xr_central -e "
  SELECT name, relay_domain, game_domain, is_active 
  FROM server 
  WHERE is_active = true;
"
```

### Headers de réponse

Utilisez les DevTools du navigateur (F12) → onglet **Network** → sélectionner une requête → **Headers** pour voir :

```
Response Headers:
  access-control-allow-origin: https://game.example.com
  access-control-allow-credentials: true
  access-control-expose-headers: content-type, authorization
```

Si ces headers sont absents, l'origine n'est pas autorisée.

## ⚠️ Limitations et considérations

### Performance

- **Cache recommandé** : Pour de meilleures performances en production, envisagez de mettre en cache la liste des domaines autorisés avec une invalidation lors des modifications
- **Requête DB par requête** : Actuellement, chaque requête CORS interroge la base de données

### Sécurité

- **HTTPS en production** : Les domaines `game_domain` et `relay_domain` devraient utiliser HTTPS en production
- **Validation des URLs** : Le middleware parse les URLs mais ne valide pas leur format strictement
- **Credentials** : `Access-Control-Allow-Credentials: true` permet l'envoi de cookies/tokens

### Améliorations futures

1. **Cache Redis** : Mettre en cache la liste des domaines autorisés
2. **Webhook** : Invalider le cache lors de la création/modification d'un serveur
3. **Wildcard domains** : Support pour `*.example.com`
4. **Rate limiting** : Par domaine pour éviter les abus
5. **Whitelist statique** : Option pour désactiver le CORS dynamique et utiliser une liste statique

## 📚 Références

- [MDN - CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
- [Axum Middleware](https://docs.rs/axum/latest/axum/middleware/index.html)
- [Tower HTTP CORS](https://docs.rs/tower-http/latest/tower_http/cors/index.html)

## 🎉 Conclusion

Le CORS dynamique permet une gestion flexible et sécurisée des accès cross-origin. Les serveurs enregistrés peuvent immédiatement communiquer avec l'API sans configuration manuelle supplémentaire.
