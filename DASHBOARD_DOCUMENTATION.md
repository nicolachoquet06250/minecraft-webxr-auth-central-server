# Dashboard du Serveur - Documentation

## Vue d'ensemble

Le dashboard du serveur offre une interface complète pour visualiser les statistiques et les métriques de performance de votre serveur de jeu Minecraft WebXR.

## Fonctionnalités

### 📊 Cartes de statistiques
Le dashboard affiche quatre cartes principales avec des statistiques clés :
- **Visites totales** : Le nombre total de visites depuis le lancement du serveur
- **Visites ce mois** : Le nombre de visites pour le mois en cours
- **Visites aujourd'hui** : Le nombre de visites pour la journée actuelle
- **Joueurs actifs** : Le nombre de joueurs actuellement connectés

### 📈 Graphiques interactifs

1. **Visites par mois**
   - Graphique en ligne montrant l'évolution des visites sur les 12 derniers mois
   - Permet de visualiser la tendance de croissance du serveur

2. **Visites des 30 derniers jours**
   - Graphique en barres montrant les visites quotidiennes du dernier mois
   - Utile pour identifier les jours avec le plus de trafic

3. **Trafic par heure (24h)**
   - Graphique en ligne montrant la distribution du trafic sur 24 heures
   - Aide à identifier les heures de pointe

4. **Types de connexion**
   - Graphique circulaire (doughnut) montrant la répartition des types de connexion
   - Affiche les différentes méthodes de connexion (WebSocket, WebRTC, HTTP, etc.)

### ℹ️ Informations du serveur

Section affichant les informations détaillées :
- URL du serveur relais
- Dernière mise à jour des statistiques
- Temps de fonctionnement (uptime)
- Version du serveur

## Navigation

### Accès au dashboard
1. **Depuis la liste des serveurs** : Cliquez directement sur une carte de serveur pour accéder à son dashboard
2. **Bouton Dashboard** : Utilisez le bouton "📊 Dashboard" sur chaque carte de serveur
3. **Après création** : Vous êtes automatiquement redirigé vers le dashboard après avoir créé un nouveau serveur

### Retour
Utilisez le bouton "← Retour aux serveurs" en haut à gauche pour revenir à la liste des serveurs

## API /stats du serveur relais

Le dashboard appelle automatiquement la route `/stats` de votre serveur relais. Le serveur relais doit retourner un JSON avec la structure suivante :

```json
{
  "total_visits": 12543,
  "visits_this_month": 1234,
  "visits_today": 89,
  "active_players": 12,
  "visits_by_month": [
    { "label": "janv. 2026", "count": 450 },
    { "label": "févr. 2026", "count": 520 },
    ...
  ],
  "visits_by_day": [
    { "label": "01/06", "count": 45 },
    { "label": "02/06", "count": 52 },
    ...
  ],
  "visits_by_hour": [
    { "label": "0h", "count": 12 },
    { "label": "1h", "count": 8 },
    ...
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

### Données de démonstration (Fallback)

Si le serveur relais ne retourne pas de données ou si la connexion échoue, le dashboard génère automatiquement des données de démonstration pour que vous puissiez voir à quoi ressemble l'interface.

## Technologies utilisées

- **Vue 3** : Framework JavaScript pour l'interface
- **Chart.js** : Bibliothèque de graphiques
- **vue-chartjs** : Intégration de Chart.js avec Vue 3
- **Vue Router** : Navigation entre les pages

## Personnalisation

Les couleurs et le style du dashboard suivent le thème Minecraft avec :
- Couleur primaire : `#64ffda` (cyan/turquoise)
- Fond sombre pour un meilleur contraste
- Animations et effets de survol

## Développement futur

Fonctionnalités potentielles à ajouter :
- Filtres de dates pour les graphiques
- Export des données en CSV/Excel
- Alertes et notifications en temps réel
- Comparaison entre périodes
- Statistiques géographiques des joueurs
- Métriques de performance du serveur (CPU, RAM, etc.)
