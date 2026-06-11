# Vérification de santé du serveur relais

## 🎯 Vue d'ensemble

Avant d'enregistrer un serveur dans la base de données, le backend vérifie automatiquement que le serveur relais est accessible et fonctionne correctement.

## 🔍 Fonctionnement

### 1. Route de santé requise

Chaque serveur relais **DOIT** exposer une route `/health` qui répond avec un statut HTTP de succès (2xx).

**Exemple d'implémentation côté serveur relais :**

```javascript
// Node.js/Express
app.get('/health', (req, res) => {
  res.status(200).json({ status: 'ok' });
});
```

```rust
// Rust/Axum
async fn health_check() -> StatusCode {
    StatusCode::OK
}

// Dans le router
.route("/health", get(health_check))
```

```python
# Python/Flask
@app.route('/health')
def health():
    return {'status': 'ok'}, 200
```

### 2. Processus de vérification

Lors de l'enregistrement d'un serveur, le backend :

1. **Construit l'URL de santé** : `{relay_domain}/health`
   - Supprime automatiquement les `/` en fin d'URL
   - Exemple : `https://relay.example.com/health`

2. **Envoie une requête GET** avec un timeout de 10 secondes

3. **Vérifie la réponse** :
   - ✅ Statut 2xx (200-299) : Serveur accessible → Enregistrement autorisé
   - ❌ Statut autre ou erreur de connexion → Enregistrement bloqué

## 📋 Codes d'erreur

| Code HTTP | Signification | Message utilisateur |
|-----------|---------------|---------------------|
| 503 Service Unavailable | Le serveur relais est inaccessible | "Le serveur relais est injoignable. Vérifiez que le serveur est en ligne et que l'URL est correcte." |
| 409 Conflict | Le domaine est déjà enregistré | "Ce domaine est déjà enregistré par un autre serveur." |
| 400 Bad Request | Données de formulaire invalides | "Données invalides. Vérifiez les champs du formulaire." |

## 🛠️ Implémentation technique

### Backend (Rust)

**Fichier** : `backend/src/routes/server.rs`

```rust
// Check relay server health
let health_url = format!("{}/health", payload.relay_domain.trim_end_matches('/'));

tracing::info!("Checking relay server health at: {}", health_url);

let health_check = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()
    .map_err(|e| {
        tracing::error!("Failed to build HTTP client: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .get(&health_url)
    .send()
    .await;

match health_check {
    Ok(response) => {
        if !response.status().is_success() {
            tracing::error!("Relay server health check failed with status: {}", response.status());
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
        tracing::info!("Relay server health check passed");
    }
    Err(e) => {
        tracing::error!("Failed to connect to relay server: {}", e);
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
}
```

### Frontend (TypeScript)

**Fichier** : `frontend/src/stores/server.ts`

```typescript
const createServer = async (data: CreateServerData) => {
  loading.value = true
  error.value = null
  try {
    const response = await serverApi.createServer(data)
    servers.value.push(response.data)
    return true
  } catch (err: any) {
    if (err.response?.status === 503) {
      error.value = 'Le serveur relais est injoignable. Vérifiez que le serveur est en ligne et que l\'URL est correcte.'
    } else if (err.response?.status === 409) {
      error.value = 'Ce domaine est déjà enregistré par un autre serveur.'
    } else if (err.response?.status === 400) {
      error.value = 'Données invalides. Vérifiez les champs du formulaire.'
    } else {
      error.value = err.response?.data?.message || 'Échec de la création du serveur'
    }
    return false
  } finally {
    loading.value = false
  }
}
```

## 📝 Logs

Les vérifications de santé sont entièrement loggées :

```
INFO  Checking relay server health at: https://relay.example.com/health
INFO  Relay server health check passed
```

En cas d'erreur :

```
ERROR Failed to connect to relay server: error sending request for url (https://relay.example.com/health): error trying to connect: tcp connect error: Connection refused (os error 111)
```

## ⚙️ Configuration

### Timeout

Le timeout par défaut est de **10 secondes**. Pour le modifier :

```rust
.timeout(std::time::Duration::from_secs(10)) // Changer la valeur ici
```

### Format de réponse

La route `/health` peut retourner n'importe quoi tant que le statut HTTP est dans la plage 2xx (200-299). Exemples acceptables :

- `200 OK` avec un corps vide
- `200 OK` avec `{ "status": "ok" }`
- `200 OK` avec `{ "healthy": true, "version": "1.0.0" }`
- `204 No Content` (sans corps)

## 🔒 Sécurité

### Protection contre les faux serveurs

La vérification de santé empêche :
- L'enregistrement de serveurs inexistants
- L'enregistrement de serveurs temporairement hors ligne
- Les erreurs de configuration (URL incorrectes)

### Limites

- **Timeout** : Empêche les blocages sur des serveurs lents
- **Vérification ponctuelle** : Vérifie uniquement au moment de l'enregistrement
  - Le serveur peut tomber après l'enregistrement
  - Possibilité d'ajouter des vérifications périodiques futures

## 🧪 Test manuel

### 1. Serveur relais fonctionnel

```bash
# Démarrer un serveur de test avec route /health
python3 -c "
from flask import Flask
app = Flask(__name__)

@app.route('/health')
def health():
    return {'status': 'ok'}, 200

app.run(port=8081)
"
```

Ensuite, enregistrer le serveur avec `relay_domain = http://localhost:8081`
→ ✅ Doit réussir

### 2. Serveur relais inaccessible

Enregistrer avec `relay_domain = http://localhost:9999` (port fermé)
→ ❌ Doit échouer avec "Le serveur relais est injoignable..."

### 3. Serveur sans route /health

```bash
python3 -m http.server 8082
```

Enregistrer avec `relay_domain = http://localhost:8082`
→ ❌ Doit échouer (réponse 404 sur /health)

## 🚀 Améliorations futures possibles

- [ ] Vérifications périodiques de santé (cron job)
- [ ] Dashboard de statut des serveurs
- [ ] Notifications quand un serveur devient injoignable
- [ ] Métriques de disponibilité
- [ ] Retry automatique lors de l'enregistrement
- [ ] Support de différents endpoints de santé (/healthz, /status, etc.)
- [ ] Vérification SSL/TLS pour les URLs HTTPS
- [ ] Test de latence et affichage dans l'interface

## 📚 Références

- **reqwest** : https://docs.rs/reqwest/
- **HTTP Status Codes** : https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
- **Health Check Patterns** : https://microservices.io/patterns/observability/health-check-api.html
