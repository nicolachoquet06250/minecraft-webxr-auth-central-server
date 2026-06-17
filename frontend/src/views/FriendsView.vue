<template>
  <div class="friends-page voxicraft-bg">
    <div class="voxicraft-container">
      <div class="page-header">
        <h1 class="voxicraft-title">👥 Amis</h1>
        <p class="voxicraft-text subtitle">Retrouve tes amis, accepte les demandes et ajoute de nouveaux joueurs.</p>
      </div>

      <div v-if="friendsStore.error" class="alert error-alert">{{ friendsStore.error }}</div>
      <div v-if="friendsStore.loading" class="voxicraft-text loading-message">⏳ Chargement...</div>

      <section class="friends-layout">
        <div class="voxicraft-panel search-panel">
          <div class="panel-title-row">
            <h2>🔎 Rechercher des joueurs</h2>
            <span class="count-badge" v-if="friendsStore.searchResults">{{ friendsStore.searchResults.total }}</span>
          </div>
          <form class="search-form" @submit.prevent="runSearch(1)">
            <input
              v-model="searchQuery"
              class="voxicraft-input"
              type="search"
              placeholder="Pseudo, ou vide pour tout lister"
            />
            <button class="voxicraft-button" type="submit" :disabled="friendsStore.searchLoading">
              {{ friendsStore.searchLoading ? 'Recherche...' : 'Rechercher' }}
            </button>
          </form>

          <p class="voxicraft-text hint">Laisse le champ vide pour afficher tous les utilisateurs paginés.</p>

          <div v-if="friendsStore.searchLoading" class="voxicraft-text small-message">Recherche en cours...</div>
          <div v-else-if="friendsStore.searchResults && friendsStore.searchResults.items.length === 0" class="empty-inline">Aucun utilisateur trouvé.</div>
          <div v-else-if="friendsStore.searchResults" class="user-list">
            <div v-for="user in friendsStore.searchResults.items" :key="user.id" class="user-row">
              <img :src="apiAsset(user.avatar.url)" :alt="user.avatar.name" class="avatar-img" />
              <div class="user-info">
                <strong>{{ user.username }}</strong>
                <span>{{ user.avatar.name }}</span>
              </div>
              <button
                v-if="relationStatus(user) === 'none'"
                class="voxicraft-button small"
                @click="sendRequest(user.id)"
              >Ajouter</button>
              <span v-else-if="relationStatus(user) === 'friend'" class="status-pill success">Ami</span>
              <span v-else-if="relationStatus(user) === 'outgoing'" class="status-pill pending">Demande envoyée</span>
              <span v-else class="status-pill pending">Demande reçue</span>
            </div>

            <div class="pagination-row">
              <button class="voxicraft-button small secondary" :disabled="!friendsStore.searchResults.previous_url" @click="goToSearchPage(searchPage - 1)">← Précédent</button>
              <span>Page {{ friendsStore.searchResults.page }} / {{ friendsStore.searchResults.total_pages || 1 }}</span>
              <button class="voxicraft-button small secondary" :disabled="!friendsStore.searchResults.next_url" @click="goToSearchPage(searchPage + 1)">Suivant →</button>
            </div>
          </div>
        </div>

        <div class="side-column">
          <section class="voxicraft-panel">
            <div class="panel-title-row">
              <h2>📥 Demandes reçues</h2>
              <span class="count-badge">{{ friendsStore.incomingRequests.length }}</span>
            </div>
            <div v-if="friendsStore.incomingRequests.length === 0" class="empty-inline">Aucune demande reçue.</div>
            <div v-else class="request-list">
              <div v-for="request in friendsStore.incomingRequests" :key="request.id" class="request-card">
                <img :src="apiAsset(request.requester.avatar.url)" :alt="request.requester.avatar.name" class="avatar-img" />
                <div class="user-info">
                  <strong>{{ request.requester.username }}</strong>
                  <span>Envoyée le {{ formatDate(request.created_at) }}</span>
                </div>
                <div class="action-row compact">
                  <button class="voxicraft-button small" @click="friendsStore.acceptRequest(request.id)">Accepter</button>
                  <button class="voxicraft-button small danger" @click="friendsStore.refuseRequest(request.id)">Refuser</button>
                </div>
              </div>
            </div>
          </section>

          <section class="voxicraft-panel">
            <div class="panel-title-row">
              <h2>📤 Demandes envoyées</h2>
              <span class="count-badge">{{ friendsStore.outgoingRequests.length }}</span>
            </div>
            <div v-if="friendsStore.outgoingRequests.length === 0" class="empty-inline">Aucune demande envoyée.</div>
            <div v-else class="request-list">
              <div v-for="request in friendsStore.outgoingRequests" :key="request.id" class="request-card">
                <img :src="apiAsset(request.receiver.avatar.url)" :alt="request.receiver.avatar.name" class="avatar-img" />
                <div class="user-info">
                  <strong>{{ request.receiver.username }}</strong>
                  <span>En attente depuis le {{ formatDate(request.created_at) }}</span>
                </div>
                <button class="voxicraft-button small danger" @click="friendsStore.removeFriend(request.receiver.id)">Annuler</button>
              </div>
            </div>
          </section>
        </div>
      </section>

      <section class="voxicraft-panel friends-list-panel">
        <div class="panel-title-row">
          <h2>✅ Mes amis</h2>
          <span class="count-badge">{{ friendsStore.friends.length }}</span>
        </div>
        <div v-if="friendsStore.friends.length === 0" class="empty-state">
          <div class="empty-icon">👥</div>
          <h3>Aucun ami pour le moment</h3>
          <p class="voxicraft-text">Recherche un joueur puis envoie-lui une demande d’ami.</p>
        </div>
        <div v-else class="friend-grid">
          <div v-for="entry in friendsStore.friends" :key="entry.user.id" class="friend-card">
            <img :src="apiAsset(entry.user.avatar.url)" :alt="entry.user.avatar.name" class="avatar-large" />
            <div>
              <h3>{{ entry.user.username }}</h3>
              <p>{{ entry.user.avatar.name }}</p>
              <p class="muted">Amis depuis le {{ formatDate(entry.created_at) }}</p>
            </div>
            <button class="voxicraft-button small danger" @click="friendsStore.removeFriend(entry.user.id)">Supprimer</button>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { FriendUser } from '@/api'
import { useFriendsStore } from '@/stores/friends'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const friendsStore = useFriendsStore()
const searchQuery = ref('')
const searchPage = ref(1)
const pageSize = 20

onMounted(async () => {
  await friendsStore.fetchAll()
  await runSearch(1)
})

const runSearch = async (page: number) => {
  searchPage.value = page
  await friendsStore.searchUsers({ q: searchQuery.value, page, page_size: pageSize })
}

const goToSearchPage = async (page: number) => {
  if (page < 1) return
  await runSearch(page)
}

const sendRequest = async (userId: string) => {
  const sent = await friendsStore.sendRequest(userId)
  if (sent) await runSearch(searchPage.value)
}

const relationStatus = (user: FriendUser) => friendsStore.relationStatus(user)
const formatDate = (value: string) => new Date(value).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })
const apiAsset = (path: string) => path.startsWith('http') ? path : `${API_BASE_URL}${path.startsWith('/api') ? path.slice(4) : path}`
</script>

<style scoped>
.friends-page { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.voxicraft-container { max-width: 1200px; margin: 0 auto; }
.page-header { text-align: center; margin-bottom: 2rem; }
.subtitle { font-size: 1.05rem; opacity: .9; margin-top: .5rem; }
.loading-message, .small-message { text-align: center; padding: 1rem; }
.alert { border: 3px solid #b71c1c; background: rgba(183, 28, 28, .25); color: #ffcdd2; padding: 1rem; margin-bottom: 1rem; border-radius: 8px; }
.friends-layout { display: grid; grid-template-columns: minmax(0, 1.15fr) minmax(320px, .85fr); gap: 1.5rem; margin-bottom: 1.5rem; }
.side-column { display: flex; flex-direction: column; gap: 1.5rem; }
.search-panel, .friends-list-panel, .side-column .voxicraft-panel { padding: 1.5rem; }
.panel-title-row { display: flex; align-items: center; justify-content: space-between; gap: 1rem; margin-bottom: 1rem; }
h2, h3 { color: #64ffda; margin: 0; }
.count-badge { background: #2e7d32; color: white; border: 2px solid #1b5e20; padding: .25rem .5rem; border-radius: 999px; font-family: monospace; font-weight: bold; }
.search-form { display: flex; gap: .75rem; }
.voxicraft-input { flex: 1; background: rgba(0,0,0,.45); color: #fff; border: 3px solid #4a4a4a; border-radius: 6px; padding: .85rem 1rem; font-family: monospace; }
.voxicraft-input:focus { outline: none; border-color: #64ffda; }
.hint { margin: .75rem 0 1rem; opacity: .8; }
.user-list, .request-list { display: flex; flex-direction: column; gap: .75rem; }
.user-row, .request-card, .friend-card { display: flex; align-items: center; gap: .85rem; background: rgba(0,0,0,.25); border: 2px solid rgba(255,255,255,.12); border-radius: 8px; padding: .85rem; }
.user-info { flex: 1; display: flex; flex-direction: column; gap: .25rem; min-width: 0; }
.user-info strong { color: #fff; overflow-wrap: anywhere; }
.user-info span, .muted { color: #d7ccc8; font-size: .85rem; }
.avatar-img { width: 48px; height: 48px; object-fit: contain; background: rgba(255,255,255,.08); border: 2px solid #4a4a4a; border-radius: 8px; }
.avatar-large { width: 72px; height: 72px; object-fit: contain; background: rgba(255,255,255,.08); border: 2px solid #4a4a4a; border-radius: 10px; }
.status-pill { border-radius: 999px; padding: .35rem .55rem; font-size: .75rem; font-family: monospace; white-space: nowrap; }
.status-pill.success { background: rgba(76, 175, 80, .25); color: #a5d6a7; border: 1px solid #4caf50; }
.status-pill.pending { background: rgba(255, 179, 0, .22); color: #ffe082; border: 1px solid #ffb300; }
.action-row { display: flex; gap: .5rem; flex-wrap: wrap; }
.action-row.compact { justify-content: flex-end; }
.pagination-row { display: flex; justify-content: center; align-items: center; gap: 1rem; margin-top: 1rem; color: #d7ccc8; flex-wrap: wrap; }
.friend-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1rem; }
.friend-card { align-items: flex-start; }
.friend-card > div { flex: 1; min-width: 0; }
.empty-state, .empty-inline { text-align: center; color: #d7ccc8; padding: 1.5rem; }
.empty-icon { font-size: 4rem; margin-bottom: 1rem; }
.voxicraft-button.small { padding: .5rem .85rem; font-size: .8rem; }
.voxicraft-button.secondary { background-color: #6d4c41; border-color: #4e342e; }
.voxicraft-button.danger { background-color: #f44336; border-color: #c62828; }
.voxicraft-button:disabled { opacity: .55; cursor: not-allowed; }
@media (max-width: 900px) {
  .friends-layout { grid-template-columns: 1fr; }
}
@media (max-width: 640px) {
  .search-form, .user-row, .request-card, .friend-card { flex-direction: column; align-items: stretch; text-align: center; }
  .avatar-img, .avatar-large { align-self: center; }
  .action-row.compact { justify-content: center; }
}
</style>
