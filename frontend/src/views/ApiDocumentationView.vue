<template>
  <main class="api-docs voxicraft-bg">
    <section class="voxicraft-panel docs-panel">
      <header class="docs-header">
        <h1 class="voxicraft-title">Documentation API</h1>
        <p class="voxicraft-text">Documentation détaillée de l’API Voxicraft Auth et de son modèle de données.</p>
      </header>

      <section class="docs-section">
        <h2>Authentification</h2>
        <p>Les routes protégées utilisent un JWT envoyé dans l’en-tête HTTP suivant :</p>
        <pre>Authorization: Bearer &lt;token&gt;</pre>
        <p>Les pages Swagger et documentation sont elles aussi protégées côté routeur front : sans session active, l’utilisateur est redirigé vers la page de login.</p>
      </section>

      <section class="docs-section">
        <h2>Ressources principales</h2>
        <div class="resource-grid">
          <article v-for="resource in resources" :key="resource.name" class="resource-card">
            <h3>{{ resource.name }}</h3>
            <p>{{ resource.description }}</p>
            <ul>
              <li v-for="field in resource.fields" :key="field"><code>{{ field }}</code></li>
            </ul>
          </article>
        </div>
      </section>

      <section class="docs-section">
        <h2>Modèle de données</h2>
        <div class="schema-grid">
          <article v-for="schema in schemas" :key="schema.name" class="schema-card">
            <h3>{{ schema.name }}</h3>
            <pre>{{ schema.shape }}</pre>
          </article>
        </div>
      </section>

      <section class="docs-section">
        <h2>Flux fonctionnels</h2>
        <div class="flow-list">
          <article>
            <h3>Créer et sélectionner un avatar</h3>
            <ol>
              <li>Créer une copie via <code>POST /api/users/me/avatars</code>.</li>
              <li>Modifier la texture via <code>PUT /api/users/me/avatars/:id</code>.</li>
              <li>Sélectionner l’avatar via <code>PUT /api/users/me/avatars/:id/select</code>.</li>
            </ol>
          </article>
          <article>
            <h3>Créer un serveur</h3>
            <ol>
              <li>Créer le serveur via <code>POST /api/servers</code>.</li>
              <li>Le client dashboard lit les statistiques via l’endpoint <code>/stats</code> du serveur de jeu.</li>
              <li>Le temps réel utilise le WebSocket du serveur de jeu, pas un WebSocket central.</li>
            </ol>
          </article>
        </div>
      </section>
    </section>
  </main>
</template>

<script setup lang="ts">
const resources = [
  {
    name: 'User',
    description: 'Compte utilisateur central utilisé pour l’authentification, le profil et les avatars.',
    fields: ['id', 'username', 'email', 'avatar', 'bio?', 'birthdate', 'age_verified', 'discord_username?', 'created_at'],
  },
  {
    name: 'Server',
    description: 'Serveur de jeu enregistré par un utilisateur.',
    fields: ['id', 'owner_id', 'name', 'game_domain', 'description?', 'is_active', 'created_at', 'updated_at'],
  },
  {
    name: 'UserAvatar',
    description: 'Avatar personnalisé stocké sous forme de matrices de couleurs.',
    fields: ['id', 'name', 'base_kind', 'is_active', 'texture_data', 'created_at', 'updated_at'],
  },
  {
    name: 'AvatarTextureData',
    description: 'Structure sérialisée des textures pixel art de chaque partie du corps.',
    fields: ['version', 'palette', 'parts.head', 'parts.torso', 'parts.rightArm', 'parts.leftArm', 'parts.rightLeg', 'parts.leftLeg'],
  },
]

const schemas = [
  {
    name: 'RegisterData',
    shape: `{
  username: string
  email: string
  password: string
  avatar: 'steve' | 'alex'
  birthdate: string
  bio?: string
}`,
  },
  {
    name: 'SaveAvatarData',
    shape: `{
  name: string
  base_kind: 'steve' | 'alex' | 'custom'
  texture_data: AvatarTextureData
}`,
  },
  {
    name: 'AvatarTextureData',
    shape: `{
  version: 1
  palette: Record<string, [number, number, number, number]>
  parts: {
    [partName]: {
      [faceName]: {
        width: number
        height: number
        matrix: string[]
      }
    }
  }
}`,
  },
  {
    name: 'CreateServerData',
    shape: `{
  name: string
  game_domain: string
  description?: string
}`,
  },
]
</script>

<style scoped>
.api-docs { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.docs-panel { max-width: 1400px; margin: 0 auto; padding: 2rem; }
.docs-header { text-align: center; margin-bottom: 2rem; }
.docs-section { margin-top: 2rem; }
.docs-section h2 { color: #64ffda; margin-bottom: 1rem; }
.resource-grid, .schema-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1rem; }
.resource-card, .schema-card, .flow-list article { background: rgba(0, 0, 0, .28); border: 1px solid rgba(100, 255, 218, .18); border-radius: 12px; padding: 1rem; }
h3 { color: #ffd700; margin-bottom: .75rem; }
p, li { color: rgba(255, 255, 255, .82); line-height: 1.6; }
ul, ol { padding-left: 1.25rem; }
code { color: #64ffda; }
pre { overflow: auto; background: rgba(0,0,0,.55); color: #d8fff6; border-radius: 10px; padding: 1rem; font-size: .85rem; }
.flow-list { display: grid; gap: 1rem; }
</style>
