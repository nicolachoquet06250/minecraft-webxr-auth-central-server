<template>
  <div class="servers-history voxicraft-bg">
    <div class="voxicraft-container">
      <div class="page-header">
        <div class="header-actions">
          <router-link to="/servers" class="voxicraft-button secondary">← Tous les serveurs</router-link>
          <router-link to="/servers/favorites" class="voxicraft-button secondary">⭐ Favoris</router-link>
        </div>
        <h1 class="voxicraft-title">🕘 Serveurs récemment visités</h1>
        <p class="voxicraft-text subtitle">Les 10 derniers serveurs que tu as ouverts.</p>
      </div>

      <div v-if="loading" class="voxicraft-text loading-message">⏳ Chargement...</div>

      <div v-else-if="serverStore.recentServers.length === 0" class="empty-state voxicraft-panel">
        <div class="empty-icon">🕘</div>
        <h3>Aucun serveur récemment visité</h3>
        <p class="voxicraft-text">Clique sur l’URL d’un serveur pour l’ajouter à cette liste.</p>
        <router-link to="/servers" class="voxicraft-button">Voir les serveurs</router-link>
      </div>

      <div v-else class="server-grid">
        <div v-for="entry in serverStore.recentServers" :key="entry.server.id" class="server-card voxicraft-panel" @click="goToDashboard(entry.server.id)">
          <div class="card-title-row">
            <h3>{{ entry.server.name }}</h3>
            <button
              type="button"
              class="favorite-star-button"
              :class="{ active: entry.is_favorite }"
              :title="entry.is_favorite ? 'Retirer des favoris' : 'Ajouter aux favoris'"
              :aria-label="entry.is_favorite ? 'Retirer des favoris' : 'Ajouter aux favoris'"
              @click.stop="toggleFavorite(entry.server.id)"
            >
              {{ entry.is_favorite ? '★' : '☆' }}
            </button>
          </div>
          <p><strong>Serveur de jeu:</strong> <a :href="entry.server.game_domain" target="_blank" @click.stop="openGameServer(entry.server)">{{ entry.server.game_domain }}</a></p>
          <p v-if="entry.server.description"><strong>Description:</strong> {{ entry.server.description }}</p>
          <p><strong>Visité le:</strong> {{ formatDate(entry.visited_at) }}</p>
          <p><strong>Status:</strong> {{ entry.server.is_active ? '✅ Actif' : '🔴 Inactif' }}</p>

          <div class="button-container">
            <button @click.stop="goToDashboard(entry.server.id)" class="voxicraft-button small primary">📊 Dashboard</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import type { Server } from '@/api'
import { useServerStore } from '@/stores/server'

const router = useRouter()
const serverStore = useServerStore()
const loading = ref(false)

onMounted(async () => {
  loading.value = true
  try {
    await serverStore.fetchRecentServers()
    await serverStore.fetchFavoriteServers()
  } finally {
    loading.value = false
  }
})

const goToDashboard = (serverId: string) => {
  router.push({ name: 'server-dashboard', params: { id: serverId } })
}

const openGameServer = async (server: Server) => {
  await serverStore.recordServerVisit(server.game_domain)
}

const toggleFavorite = async (serverId: string) => {
  const entry = serverStore.recentServers.find((item) => item.server.id === serverId)
  if (entry?.is_favorite) await serverStore.unfavoriteServer(serverId)
  else await serverStore.favoriteServer(serverId)
}

const formatDate = (value?: string) => value ? new Date(value).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' }) : 'N/A'
</script>

<style scoped>
.servers-history { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.voxicraft-container { max-width: 1200px; margin: 0 auto; }
.page-header { text-align: center; margin-bottom: 2rem; }
.header-actions { display: flex; justify-content: center; gap: .75rem; flex-wrap: wrap; margin-bottom: 1.25rem; }
.subtitle { font-size: 1.05rem; opacity: .9; margin-top: .5rem; }
.loading-message { text-align: center; font-size: 1.2rem; padding: 2rem; }
.empty-state { text-align: center; padding: 3rem 2rem; max-width: 620px; margin: 0 auto; }
.empty-icon { font-size: 4rem; margin-bottom: 1rem; }
.server-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 2rem; }
.server-card { padding: 2rem; cursor: pointer; transition: all .3s ease; text-align: left; }
.server-card:hover { transform: translateY(-5px); box-shadow: 0 10px 25px rgba(0, 0, 0, .3); }
.card-title-row { display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem; margin-bottom: 1rem; }
.server-card h3 { color: #64ffda; margin: 0; font-size: 1.4rem; overflow-wrap: anywhere; }
.favorite-star-button { flex: 0 0 auto; width: 2.35rem; height: 2.35rem; display: inline-flex; align-items: center; justify-content: center; border: 2px solid #ffb300; border-radius: 8px; background: rgba(0, 0, 0, .32); color: #ffca28; font-size: 1.35rem; line-height: 1; cursor: pointer; box-shadow: 3px 3px 0 rgba(0, 0, 0, .32); transition: transform .15s ease, background-color .15s ease, color .15s ease; }
.favorite-star-button:hover { transform: translateY(-2px); background: rgba(255, 179, 0, .18); }
.favorite-star-button.active { background: #ffb300; color: #1a1a1a; border-color: #ff8f00; }
.server-card p { margin-bottom: .5rem; color: #d7ccc8; overflow-wrap: anywhere; }
.server-card a { color: #64ffda; text-decoration: none; }
.server-card a:hover { text-decoration: underline; }
.button-container { display: flex; gap: .5rem; margin-top: 1.5rem; flex-wrap: wrap; }
.voxicraft-button.small { padding: .5rem 1rem; font-size: .8rem; }
.voxicraft-button.primary { background-color: #2196f3; border-color: #1565c0; }
.voxicraft-button.secondary { background-color: #6d4c41; border-color: #4e342e; }
@media (max-width: 768px) {
  .server-grid { grid-template-columns: 1fr; }
  .button-container, .header-actions { flex-direction: column; }
}
</style>
