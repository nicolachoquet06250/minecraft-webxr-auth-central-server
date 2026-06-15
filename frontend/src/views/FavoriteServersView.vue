<template>
  <div class="servers-history voxicraft-bg">
    <div class="voxicraft-container">
      <div class="page-header">
        <div class="header-actions">
          <router-link to="/servers" class="voxicraft-button secondary">← Tous les serveurs</router-link>
          <router-link to="/servers/recent" class="voxicraft-button secondary">🕘 Récemment visités</router-link>
        </div>
        <h1 class="voxicraft-title">⭐ Serveurs favoris</h1>
        <p class="voxicraft-text subtitle">Tes serveurs gardés en favori, même s’ils ne sont plus dans les récents.</p>
      </div>

      <div v-if="loading" class="voxicraft-text loading-message">⏳ Chargement...</div>

      <div v-else-if="serverStore.favoriteServers.length === 0" class="empty-state voxicraft-panel">
        <div class="empty-icon">⭐</div>
        <h3>Aucun serveur favori</h3>
        <p class="voxicraft-text">Ajoute un serveur en favori depuis la page des serveurs.</p>
        <router-link to="/servers" class="voxicraft-button">Voir les serveurs</router-link>
      </div>

      <div v-else class="server-grid">
        <div v-for="entry in serverStore.favoriteServers" :key="entry.server.id" class="server-card voxicraft-panel">
          <div class="card-title-row">
            <h3>{{ entry.server.name }}</h3>
            <span class="favorite-badge">⭐ Favori</span>
          </div>
          <p><strong>Serveur de jeu:</strong> <a :href="entry.server.game_domain" target="_blank" @click.stop="openGameServer(entry.server)">{{ entry.server.game_domain }}</a></p>
          <p v-if="entry.server.description"><strong>Description:</strong> {{ entry.server.description }}</p>
          <p><strong>Ajouté le:</strong> {{ formatDate(entry.favorited_at) }}</p>
          <p><strong>Status:</strong> {{ entry.server.is_active ? '✅ Actif' : '🔴 Inactif' }}</p>

          <div class="button-container">
            <button @click.stop="visitServer(entry.server)" class="voxicraft-button small primary">🌍 Visiter</button>
            <button @click.stop="unfavorite(entry.server.id)" class="voxicraft-button small danger">Retirer des favoris</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { Server } from '@/api'
import { useServerStore } from '@/stores/server'

const serverStore = useServerStore()
const loading = ref(false)

onMounted(async () => {
  loading.value = true
  try {
    await serverStore.fetchFavoriteServers()
  } finally {
    loading.value = false
  }
})

const openGameServer = async (server: Server) => {
  await serverStore.recordServerVisit(server.game_domain)
}

const visitServer = async (server: Server) => {
  await serverStore.recordServerVisit(server.game_domain)
  window.open(server.game_domain, '_blank', 'noopener,noreferrer')
}

const unfavorite = async (serverId: string) => {
  await serverStore.unfavoriteServer(serverId)
}

const formatDate = (value: string) => new Date(value).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })
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
.server-card { padding: 2rem; text-align: left; }
.card-title-row { display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem; margin-bottom: 1rem; }
.server-card h3 { color: #64ffda; margin: 0; font-size: 1.4rem; overflow-wrap: anywhere; }
.favorite-badge { background: #ffb300; color: #1a1a1a; border: 2px solid #ff8f00; border-radius: 6px; padding: .3rem .45rem; font-size: .7rem; font-family: 'Press Start 2P', cursive; white-space: nowrap; }
.server-card p { margin-bottom: .5rem; color: #d7ccc8; overflow-wrap: anywhere; }
.server-card a { color: #64ffda; text-decoration: none; }
.server-card a:hover { text-decoration: underline; }
.button-container { display: flex; gap: .5rem; margin-top: 1.5rem; flex-wrap: wrap; }
.voxicraft-button.small { padding: .5rem 1rem; font-size: .8rem; }
.voxicraft-button.primary { background-color: #2196f3; border-color: #1565c0; }
.voxicraft-button.secondary { background-color: #6d4c41; border-color: #4e342e; }
.voxicraft-button.danger { background-color: #f44336; border-color: #c62828; }
@media (max-width: 768px) {
  .server-grid { grid-template-columns: 1fr; }
  .button-container, .header-actions { flex-direction: column; }
}
</style>