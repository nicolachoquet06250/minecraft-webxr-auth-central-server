<template>
  <div class="friends-page voxicraft-bg">
    <div class="voxicraft-container">
      <div class="page-header">
        <h1 class="voxicraft-title">👥 Amis</h1>
        <p class="voxicraft-text subtitle">Retrouve tes amis, accepte les demandes et ajoute de nouveaux joueurs.</p>
        <button class="voxicraft-button add-friend-button" @click="openAddModal">➕ Ajouter</button>
      </div>

      <div v-if="friendsStore.error" class="alert error-alert">{{ friendsStore.error }}</div>
      <div v-if="friendsStore.loading" class="voxicraft-text loading-message">⏳ Chargement...</div>

      <section class="friends-layout">
        <div class="side-column">
          <section class="voxicraft-panel requests-panel">
            <div class="panel-title-row">
              <h2>📥 Demandes reçues</h2>
              <span class="count-badge">{{ friendsStore.incomingRequests.length }}</span>
            </div>
            <div v-if="friendsStore.incomingRequests.length === 0" class="empty-inline">Aucune demande reçue.</div>
            <div v-else class="request-list">
              <div v-for="request in friendsStore.incomingRequests" :key="request.id" class="request-card incoming-request-card">
                <div class="request-user">
                  <img :src="avatarSrc(request.requester.avatar.url)" :alt="request.requester.avatar.name" class="avatar-img" />
                  <div class="user-info request-user-info">
                    <strong :title="request.requester.username">{{ request.requester.username }}</strong>
                    <span>Le {{ formatDate(request.created_at) }}</span>
                  </div>
                </div>
                <div class="action-row compact request-actions">
                  <button class="voxicraft-button small" title="Accepter" aria-label="Accepter" @click="acceptRequest(request.id)">✓</button>
                  <button class="voxicraft-button small danger" title="Refuser" aria-label="Refuser" @click="refuseRequest(request.id)">✕</button>
                </div>
              </div>
            </div>
          </section>

          <section class="voxicraft-panel requests-panel">
            <div class="panel-title-row">
              <h2>📤 Demandes envoyées</h2>
              <span class="count-badge">{{ friendsStore.outgoingRequests.length }}</span>
            </div>
            <div v-if="friendsStore.outgoingRequests.length === 0" class="empty-inline">Aucune demande envoyée.</div>
            <div v-else class="request-list">
              <div v-for="request in friendsStore.outgoingRequests" :key="request.id" class="request-card outgoing-request-card">
                <div class="request-user">
                  <img :src="avatarSrc(request.receiver.avatar.url)" :alt="request.receiver.avatar.name" class="avatar-img" />
                  <div class="user-info request-user-info">
                    <strong :title="request.receiver.username">{{ request.receiver.username }}</strong>
                    <span>En attente depuis le {{ formatDate(request.created_at) }}</span>
                  </div>
                </div>
                <button class="voxicraft-button small danger" title="Annuler" aria-label="Annuler" @click="removeFriend(request.receiver.id)">✕</button>
              </div>
            </div>
          </section>
        </div>

        <section class="voxicraft-panel friends-list-panel">
          <div class="panel-title-row">
            <h2>✅ Mes amis</h2>
            <span class="count-badge">{{ friendsStore.friends.length }}</span>
          </div>
          <div v-if="friendsStore.friends.length === 0" class="empty-state">
            <div class="empty-icon">👥</div>
            <h3>Aucun ami pour le moment</h3>
            <p class="voxicraft-text">Clique sur “Ajouter” pour rechercher un joueur et lui envoyer une demande d’ami.</p>
            <button class="voxicraft-button" @click="openAddModal">➕ Ajouter un ami</button>
          </div>
          <div v-else class="friend-grid">
            <div v-for="entry in friendsStore.friends" :key="entry.user.id" class="friend-card">
              <div class="friend-user">
                <img :src="avatarSrc(entry.user.avatar.url)" :alt="entry.user.avatar.name" class="avatar-large" />
                <div class="friend-user-info">
                  <router-link class="friend-profile-link" :to="`/users/${entry.user.id}`" :title="entry.user.username">
                    {{ entry.user.username }}
                  </router-link>
                  <p>{{ entry.user.avatar.name }}</p>
                  <p v-if="friendPresenceServer(entry.user.id)" class="friend-presence online" :title="friendPresenceServer(entry.user.id)?.name">
                    <span class="presence-marker">📍</span>
                    <span>{{ friendPresenceServer(entry.user.id)?.name }}</span>
                  </p>
                  <p v-else class="friend-presence offline">hors jeu</p>
                  <p class="muted">Amis depuis {{ formatApproxDurationSince(entry.created_at) }}</p>
                </div>
              </div>
              <button
                v-if="friendPresenceServer(entry.user.id)"
                class="icon-join-button"
                type="button"
                title="Rejoindre"
                aria-label="Rejoindre"
                @click="joinServer(friendPresenceServer(entry.user.id))"
              >🚪</button>
              <router-link
                class="icon-profile-button"
                :to="`/users/${entry.user.id}`"
                title="Voir le profil"
                aria-label="Voir le profil"
              >👤</router-link>
              <button
                class="icon-danger-button"
                type="button"
                title="Supprimer"
                aria-label="Supprimer cet ami"
                @click="removeFriend(entry.user.id)"
              >🗑️</button>
            </div>
          </div>
        </section>
      </section>
    </div>

    <Teleport to="body">
      <div v-if="addModalOpen" class="friends-modal-backdrop" @click.self="closeAddModal">
        <section class="friends-modal voxicraft-panel" role="dialog" aria-modal="true" aria-labelledby="friends-modal-title">
          <div class="modal-header">
            <div>
              <h2 id="friends-modal-title">➕ Ajouter un ami</h2>
              <p class="voxicraft-text hint">Recherche un pseudo, ou laisse vide pour lister les utilisateurs paginés.</p>
            </div>
            <button class="modal-close" type="button" aria-label="Fermer" @click="closeAddModal">✕</button>
          </div>

          <form class="search-form" @submit.prevent="runSearch(1)">
            <input
              v-model="searchQuery"
              class="voxicraft-input"
              type="search"
              placeholder="Pseudo, ou vide pour tout lister"
              autofocus
            />
            <button class="voxicraft-button" type="submit" :disabled="friendsStore.searchLoading" title="Rechercher" aria-label="Rechercher">
              {{ friendsStore.searchLoading ? '…' : '🔎' }}
            </button>
          </form>

          <div v-if="friendsStore.searchLoading" class="voxicraft-text small-message">Recherche en cours...</div>
          <div v-else-if="friendsStore.searchResults && friendsStore.searchResults.items.length === 0" class="empty-inline">Aucun utilisateur trouvé.</div>
          <div v-else-if="friendsStore.searchResults" class="user-list modal-user-list">
            <div v-for="user in friendsStore.searchResults.items" :key="user.id" class="user-row">
              <img :src="avatarSrc(user.avatar.url)" :alt="user.avatar.name" class="avatar-img" />
              <div class="user-info">
                <strong>{{ user.username }}</strong>
                <span>{{ user.avatar.name }}</span>
              </div>
              <button
                v-if="relationStatus(user) === 'none'"
                class="voxicraft-button small"
                title="Ajouter"
                aria-label="Ajouter"
                @click="sendRequest(user.id)"
              >🤝</button>
              <span v-else-if="relationStatus(user) === 'friend'" class="status-pill success">Ami</span>
              <span v-else-if="relationStatus(user) === 'outgoing'" class="status-pill pending">Demande envoyée</span>
              <span v-else class="status-pill pending">Demande reçue</span>
            </div>

            <div class="pagination-row">
              <button class="voxicraft-button small secondary" :disabled="!friendsStore.searchResults.previous_url" title="Précédent" aria-label="Précédent" @click="goToSearchPage(searchPage - 1)">←</button>
              <span>Page {{ friendsStore.searchResults.page }} / {{ friendsStore.searchResults.total_pages || 1 }}</span>
              <button class="voxicraft-button small secondary" :disabled="!friendsStore.searchResults.next_url" title="Suivant" aria-label="Suivant" @click="goToSearchPage(searchPage + 1)">→</button>
            </div>
          </div>
        </section>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { type FriendPresenceServer, type FriendUser } from '@/api'
import { createJoinTicket } from '@/api/join-ticket'
import { useFriendsStore } from '@/stores/friends'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const friendsStore = useFriendsStore()
const searchQuery = ref('')
const searchPage = ref(1)
const pageSize = 20
const addModalOpen = ref(false)
const avatarObjectUrls = ref<Record<string, string>>({})
const loadingAvatarUrls = new Set<string>()
const avatarPlaceholder = 'data:image/svg+xml;utf8,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%2048%2048%22%3E%3Crect%20width=%2248%22%20height=%2248%22%20rx=%228%22%20fill=%22%23212121%22/%3E%3Crect%20x=%2214%22%20y=%2210%22%20width=%2220%22%20height=%2220%22%20fill=%22%2364ffda%22%20opacity=%22.75%22/%3E%3Crect%20x=%2210%22%20y=%2234%22%20width=%2228%22%20height=%228%22%20fill=%22%2364ffda%22%20opacity=%22.45%22/%3E%3C/svg%3E'

onMounted(async () => {
  await friendsStore.fetchAll()
  await loadVisibleAvatars()
})

watch(
  () => [
    friendsStore.searchResults?.items.map((user) => user.avatar.url).join('|'),
    friendsStore.incomingRequests.map((request) => request.requester.avatar.url).join('|'),
    friendsStore.outgoingRequests.map((request) => request.receiver.avatar.url).join('|'),
    friendsStore.friends.map((entry) => entry.user.avatar.url).join('|'),
  ],
  () => { void loadVisibleAvatars() },
  { deep: false }
)

onBeforeUnmount(() => {
  Object.values(avatarObjectUrls.value).forEach((url) => URL.revokeObjectURL(url))
})

const openAddModal = async () => {
  addModalOpen.value = true
  if (!friendsStore.searchResults) await runSearch(1)
  else await loadVisibleAvatars()
}

const closeAddModal = () => {
  addModalOpen.value = false
}

const runSearch = async (page: number) => {
  searchPage.value = page
  await friendsStore.searchUsers({ q: searchQuery.value, page, page_size: pageSize })
  await loadVisibleAvatars()
}

const goToSearchPage = async (page: number) => {
  if (page < 1) return
  await runSearch(page)
}

const sendRequest = async (userId: string) => {
  const sent = await friendsStore.sendRequest(userId)
  if (sent) await runSearch(searchPage.value)
}

const acceptRequest = async (requestId: string) => {
  await friendsStore.acceptRequest(requestId)
  await loadVisibleAvatars()
}

const refuseRequest = async (requestId: string) => {
  await friendsStore.refuseRequest(requestId)
  await loadVisibleAvatars()
}

const removeFriend = async (userId: string) => {
  await friendsStore.removeFriend(userId)
  await loadVisibleAvatars()
}

const relationStatus = (user: FriendUser) => friendsStore.relationStatus(user)
const friendPresenceServer = (userId: string) => friendsStore.presenceFor(userId)
const joinServer = async (server: FriendPresenceServer | null | undefined) => {
  if (!server) return
  const ticket = await createJoinTicket(server.id)
  window.open(ticket.join_url, '_blank', 'noopener,noreferrer')
}
const formatDate = (value: string) => new Date(value).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })
const startOfLocalDay = (date: Date) => new Date(date.getFullYear(), date.getMonth(), date.getDate())
const formatApproxDurationSince = (value: string) => {
  const start = new Date(value)
  if (Number.isNaN(start.getTime())) return 'moins de 1 jour'

  const startDay = startOfLocalDay(start)
  const today = startOfLocalDay(new Date())
  const totalDays = Math.floor((today.getTime() - startDay.getTime()) / 86400000)
  if (totalDays <= 0) return 'moins de 1 jour'

  const years = Math.floor(totalDays / 365)
  const months = Math.floor((totalDays % 365) / 30)
  const days = totalDays % 30
  const yearLabel = years > 1 ? 'ans' : 'an'
  const dayLabel = days > 1 ? 'jours' : 'jour'

  if (years > 0 && months > 0) return `${years} ${yearLabel} et ${months} mois`
  if (years > 0) return `${years} ${yearLabel}`
  if (months > 0 && days > 0) return `${months} mois et ${days} ${dayLabel}`
  if (months > 0) return `${months} mois`
  return `${totalDays} ${totalDays > 1 ? 'jours' : 'jour'}`
}
const avatarSrc = (path: string) => avatarObjectUrls.value[path] || avatarPlaceholder

const loadVisibleAvatars = async () => {
  const paths = collectVisibleAvatarPaths()
  await Promise.all(paths.map((path) => loadProtectedAvatar(path)))
}

const collectVisibleAvatarPaths = () => Array.from(new Set([
  ...(friendsStore.searchResults?.items ?? []).map((user) => user.avatar.url),
  ...friendsStore.incomingRequests.map((request) => request.requester.avatar.url),
  ...friendsStore.outgoingRequests.map((request) => request.receiver.avatar.url),
  ...friendsStore.friends.map((entry) => entry.user.avatar.url),
]))

const loadProtectedAvatar = async (path: string) => {
  if (!path || avatarObjectUrls.value[path] || loadingAvatarUrls.has(path)) return
  const token = localStorage.getItem('auth_token')
  if (!token) return

  loadingAvatarUrls.add(path)
  try {
    const response = await fetch(apiAsset(path), {
      headers: { Authorization: `Bearer ${token}` },
      credentials: 'include',
    })
    if (!response.ok) return

    const blobUrl = URL.createObjectURL(await response.blob())
    avatarObjectUrls.value = { ...avatarObjectUrls.value, [path]: blobUrl }
  } finally {
    loadingAvatarUrls.delete(path)
  }
}

const apiAsset = (path: string) => path.startsWith('http') ? path : `${API_BASE_URL}${path.startsWith('/api') ? path.slice(4) : path}`
</script>

<style scoped>
.friends-page { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.voxicraft-container { max-width: 1200px; margin: 0 auto; }
.page-header { text-align: center; margin-bottom: 2rem; }
.subtitle { font-size: 1.05rem; opacity: .9; margin-top: .5rem; }
.add-friend-button { margin-top: 1rem; }
.loading-message, .small-message { text-align: center; padding: 1rem; }
.alert { border: 3px solid #b71c1c; background: rgba(183, 28, 28, .25); color: #ffcdd2; padding: 1rem; margin-bottom: 1rem; border-radius: 8px; }
.friends-layout { display: grid; grid-template-columns: minmax(320px, .85fr) minmax(0, 1.15fr); gap: 1.5rem; margin-bottom: 1.5rem; }
.side-column { display: flex; flex-direction: column; gap: 1.5rem; min-width: 0; }
.friends-list-panel, .side-column .voxicraft-panel { padding: 1.5rem; min-width: 0; }
.requests-panel { overflow: hidden; }
.panel-title-row { display: flex; align-items: center; justify-content: space-between; gap: 1rem; margin-bottom: 1rem; }
h2, h3 { color: #64ffda; margin: 0; }
.count-badge { background: #2e7d32; color: white; border: 2px solid #1b5e20; padding: .25rem .5rem; border-radius: 999px; font-family: monospace; font-weight: bold; flex: 0 0 auto; }
.search-form { display: flex; gap: .75rem; }
.voxicraft-input { flex: 1; min-width: 0; background: rgba(0,0,0,.45); color: #fff; border: 3px solid #4a4a4a; border-radius: 6px; padding: .85rem 1rem; font-family: monospace; }
.voxicraft-input:focus { outline: none; border-color: #64ffda; }
.hint { margin: .35rem 0 0; opacity: .8; }
.user-list, .request-list { display: flex; flex-direction: column; gap: .75rem; min-width: 0; }
.user-row, .request-card, .friend-card { display: flex; align-items: center; gap: .85rem; background: rgba(0,0,0,.25); border: 2px solid rgba(255,255,255,.12); border-radius: 8px; padding: .85rem; min-width: 0; }
.request-card { display: grid; grid-template-columns: minmax(0, 1fr) auto; align-items: center; gap: .9rem; }
.incoming-request-card { display: flex; flex-direction: column; align-items: stretch; gap: .75rem; padding: 1rem; max-width: 100%; overflow: hidden; }
.incoming-request-card .request-user { display: grid; grid-template-columns: 48px minmax(0, 1fr); align-items: center; gap: .85rem; width: 100%; min-width: 0; }
.incoming-request-card .request-user-info { display: flex; flex-direction: column; justify-content: center; gap: .2rem; min-width: 0; text-align: left; }
.incoming-request-card .request-user-info strong { display: block; width: 100%; max-width: 100%; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; line-height: 1.1; }
.incoming-request-card .request-user-info span { display: block; max-width: 100%; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; color: #f2e6d8; line-height: 1.1; }
.incoming-request-card .request-actions { display: grid; grid-template-columns: 1fr 1fr; gap: .65rem; width: 100%; justify-content: stretch; }
.incoming-request-card .request-actions .voxicraft-button { width: 100%; min-width: 0; text-align: center; justify-content: center; padding: .55rem .7rem; }
.request-user { display: flex; align-items: center; gap: .85rem; min-width: 0; }
.request-user-info { min-width: 0; }
.request-user-info strong { display: block; max-width: 100%; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.request-user-info span { overflow-wrap: normal; }
.request-actions { flex-wrap: nowrap; justify-content: flex-end; }
.user-info { flex: 1; display: flex; flex-direction: column; gap: .25rem; min-width: 0; }
.user-info strong { color: #fff; overflow-wrap: anywhere; }
.user-info span, .muted { color: #d7ccc8; font-size: .85rem; }
.avatar-img { width: 48px; height: 48px; object-fit: contain; background: rgba(255,255,255,.08); border: 2px solid #4a4a4a; border-radius: 8px; flex: 0 0 auto; }
.avatar-large { width: 72px; height: 72px; object-fit: contain; background: rgba(255,255,255,.08); border: 2px solid #4a4a4a; border-radius: 10px; flex: 0 0 auto; }
.status-pill { border-radius: 999px; padding: .35rem .55rem; font-size: .75rem; font-family: monospace; white-space: nowrap; }
.status-pill.success { background: rgba(76, 175, 80, .25); color: #a5d6a7; border: 1px solid #4caf50; }
.status-pill.pending { background: rgba(255, 179, 0, .22); color: #ffe082; border: 1px solid #ffb300; }
.action-row { display: flex; gap: .5rem; flex-wrap: wrap; }
.action-row.compact { justify-content: flex-end; }
.pagination-row { display: flex; justify-content: center; align-items: center; gap: 1rem; margin-top: 1rem; color: #d7ccc8; flex-wrap: wrap; }
.friend-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1rem; }
.friend-card { display: grid; grid-template-columns: minmax(0, 1fr) 44px 44px 44px; align-items: start; gap: .75rem; }
.friend-user { display: grid; grid-template-columns: 72px minmax(0, 1fr); gap: .85rem; min-width: 0; }
.friend-user-info { min-width: 0; text-align: left; }
.friend-profile-link { display: block; max-width: 100%; color: #fff; text-decoration: none; font-size: 1.17em; font-weight: 700; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; line-height: 1.1; }
.friend-profile-link:hover { color: #64ffda; text-decoration: underline; text-underline-offset: 3px; }
.friend-user-info p { margin: .15rem 0; overflow-wrap: anywhere; }
.friend-presence { display: block; max-width: 100%; font-size: .78rem; line-height: 1.25; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.friend-presence.online { color: #a5d6a7; }
.friend-presence.offline { color: #a8a8a8; font-style: italic; }
.presence-marker { margin-right: .25rem; }
.icon-danger-button, .icon-profile-button, .icon-join-button { width: 44px; height: 38px; display: inline-flex; align-items: center; justify-content: center; border: 3px solid; color: #fff; box-shadow: 3px 3px 0 rgba(0,0,0,.45); cursor: pointer; font-size: 1rem; flex: 0 0 auto; text-decoration: none; }
.icon-join-button { background-color: #1976d2; border-color: #0d47a1; }
.icon-join-button:hover { background-color: #2196f3; }
.icon-profile-button { background-color: #2e7d32; border-color: #1b5e20; }
.icon-profile-button:hover { background-color: #43a047; }
.icon-danger-button { background-color: #f44336; border-color: #c62828; }
.icon-danger-button:hover { background-color: #ff6659; }
.empty-state, .empty-inline { text-align: center; color: #d7ccc8; padding: 1.5rem; }
.empty-icon { font-size: 4rem; margin-bottom: 1rem; }
.voxicraft-button.small { padding: .5rem .85rem; font-size: .8rem; }
.voxicraft-button.secondary { background-color: #6d4c41; border-color: #4e342e; }
.voxicraft-button.danger { background-color: #f44336; border-color: #c62828; }
.voxicraft-button:disabled { opacity: .55; cursor: not-allowed; }
.friends-modal-backdrop { position: fixed; inset: 0; z-index: 2000; display: flex; align-items: center; justify-content: center; padding: 1rem; background: rgba(0,0,0,.72); }
.friends-modal { width: min(760px, 100%); max-height: min(760px, calc(100vh - 2rem)); overflow: auto; padding: 1.5rem; }
.modal-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem; margin-bottom: 1rem; }
.modal-close { background: rgba(244, 67, 54, .25); color: #ffcdd2; border: 2px solid #f44336; border-radius: 6px; cursor: pointer; font-size: 1.1rem; padding: .35rem .55rem; }
.modal-user-list { margin-top: 1rem; }
@media (max-width: 900px) { .friends-layout { grid-template-columns: 1fr; } }
@media (max-width: 640px) { .friends-page { padding: 1rem .55rem; } .friends-list-panel, .side-column .voxicraft-panel { padding: .85rem; } .search-form, .user-row { flex-direction: column; align-items: stretch; text-align: center; } .request-card { grid-template-columns: 1fr; text-align: left; padding: .75rem; } .incoming-request-card { padding: .75rem; } .incoming-request-card .request-user { grid-template-columns: 44px minmax(0, 1fr); gap: .75rem; } .incoming-request-card .avatar-img { width: 44px; height: 44px; } .request-user { align-items: center; } .request-actions { display: grid; grid-template-columns: 1fr 1fr; gap: .5rem; width: 100%; } .request-actions .voxicraft-button { width: 100%; } .friend-card { grid-template-columns: minmax(0, 1fr) 40px 40px 40px; text-align: left; padding: .75rem; } .friend-user { grid-template-columns: 56px minmax(0, 1fr); gap: .7rem; } .friend-user .avatar-large { width: 56px; height: 56px; } .icon-danger-button, .icon-profile-button, .icon-join-button { width: 40px; height: 36px; } .avatar-img, .avatar-large { align-self: center; } .request-user .avatar-img { align-self: center; } .action-row.compact { justify-content: stretch; } .friends-modal-backdrop { padding: .5rem; align-items: flex-start; } .friends-modal { max-height: calc(100vh - 1rem); padding: .85rem; } .modal-header { align-items: flex-start; } }
</style>