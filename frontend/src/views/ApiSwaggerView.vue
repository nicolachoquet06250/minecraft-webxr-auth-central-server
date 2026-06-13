<template>
  <main class="api-page voxicraft-bg">
    <section class="voxicraft-panel api-panel">
      <div class="page-header">
        <h1 class="voxicraft-title">Swagger API</h1>
        <p class="voxicraft-text">Vue OpenAPI protégée par authentification pour l'API Voxicraft Auth.</p>
      </div>

      <div class="swagger-layout">
        <aside class="endpoint-list">
          <h2>Endpoints</h2>
          <button
            v-for="endpoint in endpoints"
            :key="`${endpoint.method}-${endpoint.path}`"
            class="endpoint-button"
            :class="[`method-${endpoint.method.toLowerCase()}`, { active: selectedEndpoint === endpoint }]"
            type="button"
            @click="selectedEndpoint = endpoint"
          >
            <span class="method">{{ endpoint.method }}</span>
            <span class="path">{{ endpoint.path }}</span>
          </button>
        </aside>

        <section class="endpoint-detail">
          <div class="endpoint-heading">
            <span class="method-badge" :class="`method-${selectedEndpoint.method.toLowerCase()}`">{{ selectedEndpoint.method }}</span>
            <code>{{ selectedEndpoint.path }}</code>
          </div>
          <p>{{ selectedEndpoint.summary }}</p>

          <template v-if="selectedEndpoint.authenticated">
            <h3>Authentification</h3>
            <p>Requiert un token Bearer dans l'en-tête <code>Authorization</code>.</p>
          </template>

          <template v-if="selectedEndpoint.body">
            <h3>Body JSON</h3>
            <pre>{{ selectedEndpoint.body }}</pre>
          </template>

          <template v-if="selectedEndpoint.response">
            <h3>Réponse</h3>
            <pre>{{ selectedEndpoint.response }}</pre>
          </template>
        </section>
      </div>

      <section class="openapi-block">
        <h2>Spécification OpenAPI</h2>
        <p>Spécification synthétique embarquée côté front pour consultation.</p>
        <pre>{{ openApiSpec }}</pre>
      </section>
    </section>
  </main>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'

type Endpoint = {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE'
  path: string
  summary: string
  authenticated: boolean
  body?: string
  response?: string
}

const endpoints: Endpoint[] = [
  { method: 'POST', path: '/api/auth/register', summary: 'Créer un compte utilisateur.', authenticated: false, body: '{ username, email, password, avatar, birthdate, bio? }', response: '{ token, user }' },
  { method: 'POST', path: '/api/auth/login', summary: 'Connecter un utilisateur.', authenticated: false, body: '{ email, password }', response: '{ token, user }' },
  { method: 'GET', path: '/api/auth/discord/url', summary: 'Récupérer l’URL OAuth Discord.', authenticated: false, response: '{ url }' },
  { method: 'GET', path: '/api/users/me', summary: 'Récupérer le profil connecté.', authenticated: true, response: 'User' },
  { method: 'PUT', path: '/api/users/me', summary: 'Mettre à jour le profil connecté.', authenticated: true, body: 'Partial<User>', response: 'User' },
  { method: 'DELETE', path: '/api/users/me', summary: 'Supprimer le compte connecté.', authenticated: true, response: '204 No Content' },
  { method: 'GET', path: '/api/users/:id', summary: 'Récupérer un profil public.', authenticated: false, response: 'User' },
  { method: 'GET', path: '/api/users/me/avatar', summary: 'Récupérer l’avatar actif.', authenticated: true, response: 'ActiveAvatarResponse' },
  { method: 'DELETE', path: '/api/users/me/avatar', summary: 'Désactiver l’avatar personnalisé actif.', authenticated: true, response: '204 No Content' },
  { method: 'GET', path: '/api/users/me/avatars', summary: 'Lister les avatars personnalisés.', authenticated: true, response: 'UserAvatar[]' },
  { method: 'POST', path: '/api/users/me/avatars', summary: 'Créer une copie d’avatar.', authenticated: true, body: '{ name, base_kind, texture_data }', response: 'UserAvatar' },
  { method: 'PUT', path: '/api/users/me/avatars/:id', summary: 'Modifier un avatar personnalisé.', authenticated: true, body: '{ name?, texture_data }', response: 'UserAvatar' },
  { method: 'DELETE', path: '/api/users/me/avatars/:id', summary: 'Supprimer un avatar personnalisé.', authenticated: true, response: '204 No Content' },
  { method: 'PUT', path: '/api/users/me/avatars/:id/select', summary: 'Sélectionner un avatar personnalisé.', authenticated: true, response: '204 No Content' },
  { method: 'GET', path: '/api/users/me/profile-pic.svg', summary: 'Récupérer la tête SVG de l’avatar connecté.', authenticated: true, response: 'image/svg+xml' },
  { method: 'GET', path: '/api/users/:id/profile-pic.svg', summary: 'Récupérer la tête SVG d’un utilisateur.', authenticated: true, response: 'image/svg+xml' },
  { method: 'GET', path: '/api/users/:id/matrix-color', summary: 'Récupérer la couleur Matrix d’un utilisateur.', authenticated: true, response: '{ color }' },
  { method: 'POST', path: '/api/servers', summary: 'Créer un serveur de jeu.', authenticated: true, body: '{ name, game_domain, description? }', response: 'Server' },
  { method: 'GET', path: '/api/servers', summary: 'Lister les serveurs du compte connecté.', authenticated: true, response: 'Server[]' },
  { method: 'GET', path: '/api/servers/:id', summary: 'Récupérer un serveur public.', authenticated: false, response: 'Server' },
  { method: 'PUT', path: '/api/servers/:id', summary: 'Modifier un serveur.', authenticated: true, body: 'Partial<CreateServerData>', response: 'Server' },
  { method: 'DELETE', path: '/api/servers/:id', summary: 'Supprimer un serveur.', authenticated: true, response: '204 No Content' },
]

const selectedEndpoint = ref(endpoints[0])
const openApiSpec = computed(() => JSON.stringify({
  openapi: '3.0.3',
  info: { title: 'Voxicraft Auth API', version: '1.0.0' },
  security: [{ bearerAuth: [] }],
  paths: Object.fromEntries(endpoints.map((endpoint) => [endpoint.path, { [endpoint.method.toLowerCase()]: { summary: endpoint.summary, security: endpoint.authenticated ? [{ bearerAuth: [] }] : [] } }])),
  components: { securitySchemes: { bearerAuth: { type: 'http', scheme: 'bearer', bearerFormat: 'JWT' } } },
}, null, 2))
</script>

<style scoped>
.api-page { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.api-panel { max-width: 1400px; margin: 0 auto; padding: 2rem; }
.page-header { text-align: center; margin-bottom: 2rem; }
.swagger-layout { display: grid; grid-template-columns: 380px minmax(0, 1fr); gap: 1.5rem; align-items: start; }
.endpoint-list, .endpoint-detail, .openapi-block { background: rgba(0, 0, 0, .28); border: 1px solid rgba(100, 255, 218, .18); border-radius: 12px; padding: 1rem; }
.endpoint-list { display: flex; flex-direction: column; gap: .5rem; max-height: 720px; overflow: auto; }
.endpoint-list h2, .endpoint-detail h3, .openapi-block h2 { color: #64ffda; }
.endpoint-button { display: grid; grid-template-columns: 72px 1fr; gap: .75rem; align-items: center; text-align: left; border: 1px solid rgba(255,255,255,.12); background: rgba(255,255,255,.05); color: #fff; border-radius: 8px; padding: .55rem; cursor: pointer; }
.endpoint-button.active { border-color: #64ffda; background: rgba(100,255,218,.12); }
.method, .method-badge { font-weight: 800; text-align: center; border-radius: 6px; padding: .25rem .45rem; color: #101010; }
.method-get { background: rgba(100,255,218,.75); }
.method-post { background: rgba(76,175,80,.8); }
.method-put { background: rgba(255,193,7,.85); }
.method-delete { background: rgba(255,107,107,.85); }
.path { font-family: monospace; overflow-wrap: anywhere; }
.endpoint-heading { display: flex; align-items: center; gap: .75rem; margin-bottom: 1rem; color: #fff; }
pre { overflow: auto; background: rgba(0,0,0,.55); color: #d8fff6; border-radius: 10px; padding: 1rem; font-size: .82rem; }
.openapi-block { margin-top: 1.5rem; }
@media (max-width: 980px) { .swagger-layout { grid-template-columns: 1fr; } }
</style>
