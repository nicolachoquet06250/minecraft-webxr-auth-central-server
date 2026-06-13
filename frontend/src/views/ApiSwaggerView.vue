<template>
  <main class="api-page voxicraft-bg">
    <section class="voxicraft-panel api-panel">
      <div class="page-header">
        <h1 class="voxicraft-title">Swagger API</h1>
        <p class="voxicraft-text">Swagger UI officiel, protégé par authentification, avec JWT préchargé automatiquement.</p>
      </div>

      <div ref="swaggerContainer" class="swagger-container"></div>
    </section>
  </main>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'
import SwaggerUI from 'swagger-ui-dist/swagger-ui-bundle'
import 'swagger-ui-dist/swagger-ui.css'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const swaggerContainer = ref<HTMLElement | null>(null)
let swaggerUi: any = null

const openApiSpec = {
  openapi: '3.0.3',
  info: {
    title: 'Voxicraft Auth API',
    version: '1.0.0',
    description: 'API centrale d’authentification, gestion des avatars personnalisés et registre de serveurs Voxicraft.',
  },
  servers: [{ url: API_BASE_URL }],
  components: {
    securitySchemes: {
      bearerAuth: { type: 'http', scheme: 'bearer', bearerFormat: 'JWT' },
    },
    schemas: {
      User: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          username: { type: 'string' },
          email: { type: 'string' },
          avatar: { type: 'string', enum: ['steve', 'alex'] },
          bio: { type: 'string', nullable: true },
          birthdate: { type: 'string' },
          age_verified: { type: 'boolean' },
          discord_username: { type: 'string', nullable: true },
          created_at: { type: 'string' },
        },
      },
      AuthResponse: {
        type: 'object',
        properties: {
          token: { type: 'string' },
          user: { $ref: '#/components/schemas/User' },
        },
      },
      RegisterData: {
        type: 'object',
        required: ['username', 'email', 'password', 'avatar', 'birthdate'],
        properties: {
          username: { type: 'string' },
          email: { type: 'string' },
          password: { type: 'string', format: 'password' },
          avatar: { type: 'string', enum: ['steve', 'alex'] },
          birthdate: { type: 'string' },
          bio: { type: 'string' },
        },
      },
      LoginData: {
        type: 'object',
        required: ['email', 'password'],
        properties: {
          email: { type: 'string' },
          password: { type: 'string', format: 'password' },
        },
      },
      Server: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          owner_id: { type: 'string' },
          name: { type: 'string' },
          game_domain: { type: 'string' },
          description: { type: 'string', nullable: true },
          is_active: { type: 'boolean' },
          created_at: { type: 'string' },
          updated_at: { type: 'string' },
        },
      },
      CreateServerData: {
        type: 'object',
        required: ['name', 'game_domain'],
        properties: {
          name: { type: 'string' },
          game_domain: { type: 'string' },
          description: { type: 'string' },
        },
      },
      AvatarTextureData: {
        type: 'object',
        properties: {
          version: { type: 'number', enum: [1] },
          palette: { type: 'object', additionalProperties: { type: 'array', items: { type: 'number' }, minItems: 4, maxItems: 4 } },
          parts: { type: 'object' },
        },
      },
      UserAvatar: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          name: { type: 'string' },
          base_kind: { type: 'string', enum: ['steve', 'alex', 'custom'] },
          is_active: { type: 'boolean' },
          texture_data: { $ref: '#/components/schemas/AvatarTextureData' },
          created_at: { type: 'string' },
          updated_at: { type: 'string' },
        },
      },
      SaveAvatarData: {
        type: 'object',
        required: ['name', 'base_kind', 'texture_data'],
        properties: {
          name: { type: 'string' },
          base_kind: { type: 'string', enum: ['steve', 'alex', 'custom'] },
          texture_data: { $ref: '#/components/schemas/AvatarTextureData' },
        },
      },
      UpdateAvatarData: {
        type: 'object',
        required: ['texture_data'],
        properties: {
          name: { type: 'string' },
          texture_data: { $ref: '#/components/schemas/AvatarTextureData' },
        },
      },
      ActiveAvatarResponse: {
        type: 'object',
        properties: {
          kind: { type: 'string', enum: ['default', 'custom'] },
          avatar: { oneOf: [{ $ref: '#/components/schemas/UserAvatar' }, { type: 'null' }] },
        },
      },
    },
  },
  paths: {
    '/auth/register': {
      post: {
        summary: 'Créer un compte utilisateur',
        requestBody: jsonBody('#/components/schemas/RegisterData'),
        responses: { 200: jsonResponse('#/components/schemas/AuthResponse'), 400: description('Requête invalide') },
      },
    },
    '/auth/login': {
      post: {
        summary: 'Connecter un utilisateur',
        requestBody: jsonBody('#/components/schemas/LoginData'),
        responses: { 200: jsonResponse('#/components/schemas/AuthResponse'), 401: description('Identifiants invalides') },
      },
    },
    '/auth/discord/url': {
      get: { summary: 'Récupérer l’URL OAuth Discord', responses: { 200: inlineJsonResponse({ url: { type: 'string' } }) } },
    },
    '/users/me': {
      get: protectedOperation('Récupérer le profil connecté', { 200: jsonResponse('#/components/schemas/User') }),
      put: protectedOperation('Mettre à jour le profil connecté', { 200: jsonResponse('#/components/schemas/User') }),
      delete: protectedOperation('Supprimer le compte connecté', { 204: description('Compte supprimé') }),
    },
    '/users/{id}': {
      get: {
        summary: 'Récupérer un profil public',
        parameters: pathParams('id'),
        responses: { 200: jsonResponse('#/components/schemas/User'), 404: description('Utilisateur introuvable') },
      },
    },
    '/users/me/avatar': {
      get: protectedOperation('Récupérer l’avatar actif', { 200: jsonResponse('#/components/schemas/ActiveAvatarResponse') }),
      delete: protectedOperation('Désactiver l’avatar personnalisé actif', { 204: description('Avatar actif supprimé') }),
    },
    '/users/me/avatars': {
      get: protectedOperation('Lister les avatars personnalisés', { 200: arrayResponse('#/components/schemas/UserAvatar') }),
      post: protectedOperation('Créer une copie d’avatar', { 200: jsonResponse('#/components/schemas/UserAvatar') }, jsonBody('#/components/schemas/SaveAvatarData')),
    },
    '/users/me/avatars/{id}': {
      put: protectedOperation('Modifier un avatar personnalisé', { 200: jsonResponse('#/components/schemas/UserAvatar') }, jsonBody('#/components/schemas/UpdateAvatarData'), pathParams('id')),
      delete: protectedOperation('Supprimer un avatar personnalisé', { 204: description('Avatar supprimé') }, undefined, pathParams('id')),
    },
    '/users/me/avatars/{id}/select': {
      put: protectedOperation('Sélectionner un avatar personnalisé', { 204: description('Avatar sélectionné') }, undefined, pathParams('id')),
    },
    '/users/me/profile-pic.svg': {
      get: protectedOperation('Récupérer la tête SVG de l’avatar connecté', { 200: { description: 'Image SVG', content: { 'image/svg+xml': { schema: { type: 'string' } } } } }),
    },
    '/users/{id}/profile-pic.svg': {
      get: protectedOperation('Récupérer la tête SVG d’un utilisateur', { 200: { description: 'Image SVG', content: { 'image/svg+xml': { schema: { type: 'string' } } } } }, undefined, pathParams('id')),
    },
    '/users/{id}/matrix-color': {
      get: protectedOperation('Récupérer la couleur Matrix d’un utilisateur', { 200: inlineJsonResponse({ color: { type: 'string' } }) }, undefined, pathParams('id')),
    },
    '/servers': {
      get: protectedOperation('Lister les serveurs du compte connecté', { 200: arrayResponse('#/components/schemas/Server') }),
      post: protectedOperation('Créer un serveur de jeu', { 200: jsonResponse('#/components/schemas/Server') }, jsonBody('#/components/schemas/CreateServerData')),
    },
    '/servers/{id}': {
      get: { summary: 'Récupérer un serveur public', parameters: pathParams('id'), responses: { 200: jsonResponse('#/components/schemas/Server') } },
      put: protectedOperation('Modifier un serveur', { 200: jsonResponse('#/components/schemas/Server') }, jsonBody('#/components/schemas/CreateServerData'), pathParams('id')),
      delete: protectedOperation('Supprimer un serveur', { 204: description('Serveur supprimé') }, undefined, pathParams('id')),
    },
  },
}

onMounted(() => {
  if (!swaggerContainer.value) return

  swaggerUi = SwaggerUI({
    domNode: swaggerContainer.value,
    spec: openApiSpec,
    deepLinking: true,
    persistAuthorization: true,
    displayRequestDuration: true,
    tryItOutEnabled: true,
  })

  const jwt = localStorage.getItem('auth_token')
  if (jwt) {
    swaggerUi.authActions.authorize({
      bearerAuth: {
        name: 'bearerAuth',
        schema: { type: 'http', scheme: 'bearer', bearerFormat: 'JWT' },
        value: jwt,
      },
    })
  }
})

onBeforeUnmount(() => {
  swaggerContainer.value?.replaceChildren()
  swaggerUi = null
})

function description(description: string) {
  return { description }
}

function jsonResponse(ref: string) {
  return { description: 'OK', content: { 'application/json': { schema: { $ref: ref } } } }
}

function arrayResponse(ref: string) {
  return { description: 'OK', content: { 'application/json': { schema: { type: 'array', items: { $ref: ref } } } } }
}

function inlineJsonResponse(properties: Record<string, unknown>) {
  return { description: 'OK', content: { 'application/json': { schema: { type: 'object', properties } } } }
}

function jsonBody(ref: string) {
  return { required: true, content: { 'application/json': { schema: { $ref: ref } } } }
}

function pathParams(name: string) {
  return [{ name, in: 'path', required: true, schema: { type: 'string' } }]
}

function protectedOperation(summary: string, responses: Record<string | number, unknown>, requestBody?: unknown, parameters?: unknown[]) {
  return {
    summary,
    security: [{ bearerAuth: [] }],
    ...(parameters ? { parameters } : {}),
    ...(requestBody ? { requestBody } : {}),
    responses,
  }
}
</script>

<style scoped>
.api-page { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.api-panel { max-width: 1500px; margin: 0 auto; padding: 2rem; }
.page-header { text-align: center; margin-bottom: 2rem; }
.swagger-container { background: #ffffff; border-radius: 12px; overflow: hidden; padding: 1rem; }
:deep(.swagger-ui) { font-family: Arial, sans-serif; }
:deep(.swagger-ui .topbar) { display: none; }
:deep(.swagger-ui .scheme-container) { border-radius: 8px; }
:deep(.swagger-ui .info) { margin: 1rem 0; }
</style>
