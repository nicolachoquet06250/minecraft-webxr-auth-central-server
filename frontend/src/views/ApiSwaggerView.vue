<template>
  <main class="api-page voxicraft-bg">
    <section class="voxicraft-panel api-panel">
      <div class="page-header">
        <h1 class="voxicraft-title">Swagger API</h1>
        <p class="voxicraft-text">Swagger UI officiel, protégé par authentification, avec JWT préchargé automatiquement.</p>
      </div>

      <p v-if="errorMessage" class="swagger-error">{{ errorMessage }}</p>
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
const errorMessage = ref('')
let swaggerUi: any = null

onMounted(async () => {
  if (!swaggerContainer.value) return

  try {
    const response = await fetch(`${API_BASE_URL}/openapi.json`)
    if (!response.ok) throw new Error(`OpenAPI HTTP ${response.status}`)

    const spec = await response.json()
    spec.servers = [{ url: API_BASE_URL }]

    swaggerUi = SwaggerUI({
      domNode: swaggerContainer.value,
      spec,
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
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Impossible de charger /api/openapi.json'
  }
})

onBeforeUnmount(() => {
  swaggerContainer.value?.replaceChildren()
  swaggerUi = null
})
</script>

<style scoped>
.api-page { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.api-panel { max-width: 1500px; margin: 0 auto; padding: 2rem; }
.page-header { text-align: center; margin-bottom: 2rem; }
.swagger-container { background: #ffffff; border-radius: 12px; overflow: hidden; padding: 1rem; }
.swagger-error { background: rgba(239, 71, 111, .18); border: 1px solid rgba(239, 71, 111, .55); color: #fff; border-radius: 10px; padding: 1rem; margin-bottom: 1rem; }
:deep(.swagger-ui) { font-family: Arial, sans-serif; }
:deep(.swagger-ui .topbar) { display: none; }
:deep(.swagger-ui .scheme-container) { border-radius: 8px; }
:deep(.swagger-ui .info) { margin: 1rem 0; }
</style>
