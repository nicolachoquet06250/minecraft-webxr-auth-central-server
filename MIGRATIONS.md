# Système de Migrations de Base de Données

Ce projet utilise `sea-orm-migration` pour gérer les migrations de base de données de manière cumulative avec support complet du rollback.

## Structure

```
backend/migration/
├── Cargo.toml
├── .env                    # Configuration DATABASE_URL
└── src/
    ├── main.rs             # Point d'entrée CLI
    ├── lib.rs              # Registre des migrations
    └── m{timestamp}_{name}.rs  # Fichiers de migration
```

## Commandes

### Windows
```bash
.\migrate.bat [command]
```

### Linux/Mac
```bash
chmod +x migrate.sh
./migrate.sh [command]
```

### Commandes disponibles

**`up`** - Applique toutes les migrations en attente
```bash
.\migrate.bat up
```

**`down`** - Annule la dernière migration
```bash
.\migrate.bat down
```

**`refresh`** - Annule toutes les migrations puis les réapplique
```bash
.\migrate.bat refresh
```

**`reset`** - Annule toutes les migrations
```bash
.\migrate.bat reset
```

**`fresh`** - Supprime toutes les tables et réexécute les migrations (⚠️ DESTRUCTIF)
```bash
.\migrate.bat fresh
```

**`status`** - Affiche l'état des migrations
```bash
.\migrate.bat status
```

**`generate`** - Génère un nouveau fichier de migration
```bash
.\migrate.bat generate add_user_avatar
```

## Migrations existantes

1. **m20240101_000001_create_users_table**
   - Crée la table `users`
   - Indexes sur email et discord_id
   - Rollback: Supprime la table users

2. **m20240101_000002_create_servers_table**
   - Crée la table `servers` avec clé étrangère vers users
   - Indexes sur owner_id et domain
   - Rollback: Supprime la table servers

3. **m20240610_000001_add_dual_domains**
   - Ajoute les colonnes `relay_domain` et `game_domain`
   - Migre les données depuis l'ancien champ `domain`
   - Supprime l'ancien champ `domain`
   - Rollback: Restaure l'ancien champ `domain`

## Créer une nouvelle migration

### 1. Générer le fichier
```bash
.\migrate.bat generate add_new_feature
```

### 2. Éditer le fichier généré
```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Changements à appliquer
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(MyTable::Table)
                    .add_column(ColumnDef::new(MyTable::NewColumn).string())
                    .to_owned(),
            )
            .await
    }

    // Comment annuler les changements
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(MyTable::Table)
                    .drop_column(MyTable::NewColumn)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum MyTable {
    Table,
    NewColumn,
}
```

### 3. Enregistrer dans lib.rs
```rust
mod m20240610_000002_add_new_feature;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // ... migrations existantes
            Box::new(m20240610_000002_add_new_feature::Migration),
        ]
    }
}
```

### 4. Appliquer
```bash
.\migrate.bat up
```

## Table de tracking

Sea-ORM crée automatiquement une table `seaql_migrations` qui track :
- Le nom de chaque migration
- La version
- La date d'application

Cela permet de savoir quelles migrations sont appliquées et dans quel ordre les rollback.

## Bonnes pratiques

✅ **Toujours tester le rollback** : Après avoir créé une migration, testez immédiatement le down
```bash
.\migrate.bat up
.\migrate.bat down
.\migrate.bat up
```

✅ **Une migration = un changement logique** : Ne mélangez pas plusieurs features dans une migration

✅ **Préserver les données** : Dans les migrations destructives (DROP column), copiez d'abord les données

✅ **Nommer clairement** : Utilisez des noms descriptifs `add_user_roles` plutôt que `update_users`

❌ **Ne jamais modifier une migration appliquée** : Créez une nouvelle migration pour corriger

## Workflow de développement

### Démarrer un nouveau projet
```bash
.\migrate.bat fresh
```

### Ajouter une feature
```bash
# 1. Créer la migration
.\migrate.bat generate add_feature_x

# 2. Éditer le fichier généré

# 3. Appliquer
.\migrate.bat up

# 4. Tester le rollback
.\migrate.bat down
.\migrate.bat up
```

### Revenir en arrière d'une version
```bash
.\migrate.bat down  # Annule la dernière migration
```

### Repartir de zéro
```bash
.\migrate.bat refresh  # Annule tout et réapplique
# ou
.\migrate.bat fresh    # Supprime tout et réapplique (⚠️ perte de données)
```

## En production

⚠️ **Important** : En production, utilisez toujours `up` pour appliquer les migrations, jamais `fresh` ou `refresh` qui sont destructifs.

```bash
# Production workflow
.\migrate.bat status  # Vérifier l'état
.\migrate.bat up      # Appliquer les nouvelles migrations
```

Si un rollback est nécessaire en production :
```bash
.\migrate.bat down  # Annule la dernière migration
```

## Dépannage

**Erreur de connexion** : Vérifiez `backend/migration/.env` contient la bonne `DATABASE_URL`

**Migration bloquée** : Vérifiez la table `seaql_migrations` pour voir l'état

**Conflit** : En dev, utilisez `.\migrate.bat fresh` pour repartir de zéro
