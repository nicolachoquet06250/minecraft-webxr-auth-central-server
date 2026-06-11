# Dashboard de Serveur - Guide Rapide

## ✨ Nouveautés

Un dashboard complet a été ajouté à l'application pour afficher les statistiques et métriques de vos serveurs de jeu Minecraft WebXR.

## 🚀 Utilisation

### Accès au Dashboard

Vous pouvez accéder au dashboard de trois façons :

1. **Après création d'un serveur** 
   - Créez un nouveau serveur via le formulaire
   - Vous serez automatiquement redirigé vers le dashboard

2. **Depuis la liste des serveurs**
   - Cliquez directement sur une carte de serveur
   - Ou utilisez le bouton "📊 Dashboard" sur chaque carte

3. **URL directe**
   - `/servers/:id/dashboard` où `:id` est l'ID du serveur

### Navigation

- **Retour** : Cliquez sur "← Retour aux serveurs" en haut à gauche

## 📊 Contenu du Dashboard

### Cartes de Statistiques (4 KPIs)
- **👥 Visites totales** - Nombre total de visites depuis le lancement
- **📈 Visites ce mois** - Visites du mois en cours
- **📅 Visites aujourd'hui** - Visites du jour
- **🎮 Joueurs actifs** - Nombre de joueurs connectés actuellement

### Graphiques Interactifs

1. **📊 Visites par mois** (Graphique en ligne)
   - Évolution des visites sur les 12 derniers mois
   - Identifie les tendances de croissance

2. **📅 Visites des 30 derniers jours** (Graphique en barres)
   - Vue quotidienne détaillée du dernier mois
   - Repère les jours avec le plus de trafic

3. **🕐 Trafic par heure** (Graphique en ligne)
   - Distribution du trafic sur 24 heures
   - Identifie les heures de pointe

4. **🔌 Types de connexion** (Graphique doughnut)
   - Répartition par type de connexion
   - WebSocket, WebRTC, HTTP, Direct, etc.

### Informations du Serveur
- URL du serveur relais
- Dernière mise à jour des stats
- Temps de fonctionnement (uptime)
- Version du serveur

## 🔧 Configuration Backend

### Endpoint `/stats` requis

Le serveur relais doit exposer un endpoint `/stats` retournant un JSON :

```json
{
  "total_visits": 12543,
  "visits_this_month": 1234,
  "visits_today": 89,
  "active_players": 12,
  "visits_by_month": [
    { "label": "janv. 2026", "count": 450 },
    { "label": "févr. 2026", "count": 520 }
  ],
  "visits_by_day": [
    { "label": "01/06", "count": 45 },
    { "label": "02/06", "count": 52 }
  ],
  "visits_by_hour": [
    { "label": "0h", "count": 12 },
    { "label": "1h", "count": 8 }
  ],
  "connection_types": {
    "WebSocket": 450,
    "WebRTC": 320,
    "HTTP": 180,
    "Direct": 90
  },
  "uptime": 2592000,
  "server_version": "1.0.0"
}
```

### Configuration CORS

Assurez-vous que votre serveur relais accepte les requêtes CORS depuis votre frontend :

```rust
// Exemple Rust (actix-web)
use actix_cors::Cors;

let cors = Cors::default()
    .allowed_origin("http://localhost:5173")
    .allowed_origin("https://votre-domaine.com")
    .allowed_methods(vec!["GET"])
    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    .max_age(3600);
```

### Données de Démonstration

Si l'endpoint `/stats` n'est pas disponible ou retourne une erreur, le dashboard affiche automatiquement des données de démonstration pour visualiser l'interface.

## 🎨 Fonctionnalités Visuelles

- **Thème Minecraft** cohérent avec l'application
- **Responsive Design** pour mobile et desktop
- **Animations** fluides sur les cartes et graphiques
- **Effets de survol** interactifs
- **Loading States** pendant le chargement
- **Gestion d'erreurs** avec possibilité de réessayer

## 📦 Technologies Utilisées

- **Vue 3** - Framework JavaScript
- **Chart.js** - Bibliothèque de graphiques
- **vue-chartjs** - Intégration Vue 3 / Chart.js
- **Vue Router** - Navigation entre pages
- **TypeScript** - Typage fort

## 🚦 Démarrage

```bash
# Installation des dépendances
npm install

# Mode développement
npm run dev

# Compilation production
npm run build
```

## 📝 Fichiers Modifiés

- ✅ `frontend/src/views/ServerDashboardView.vue` (nouveau)
- ✅ `frontend/src/views/ServersView.vue` (modifié)
- ✅ `frontend/src/router/index.ts` (modifié)
- ✅ `frontend/src/api/index.ts` (modifié)
- ✅ `frontend/tsconfig.app.json` (modifié)

## 🔮 Évolutions Futures

- Filtres de dates personnalisés
- Export des données (CSV/Excel)
- Alertes et notifications en temps réel
- Comparaison entre périodes
- Statistiques géographiques
- Métriques serveur (CPU, RAM, Bande passante)
- Graphiques de performance
- Logs en temps réel

## 📚 Documentation Complète

Pour plus d'informations, consultez :
- `DASHBOARD_DOCUMENTATION.md` - Documentation détaillée
- `DASHBOARD_CHANGES.md` - Résumé des modifications

## 🆘 Support

En cas de problème :
1. Vérifiez que le serveur relais est accessible
2. Vérifiez que l'endpoint `/stats` fonctionne (testez avec curl/Postman)
3. Vérifiez la configuration CORS
4. Consultez la console du navigateur pour les erreurs
5. En cas d'erreur, le dashboard affichera un message et des données de démo

## ✅ Checklist de Déploiement

- [ ] Implémenter l'endpoint `/stats` dans le serveur relais
- [ ] Configurer CORS pour accepter les requêtes du frontend
- [ ] Collecter et stocker les statistiques de visites
- [ ] Tester l'endpoint avec curl ou Postman
- [ ] Déployer le frontend avec `npm run build`
- [ ] Vérifier que le dashboard fonctionne en production

---

**Bon dashboard !** 🎮📊
