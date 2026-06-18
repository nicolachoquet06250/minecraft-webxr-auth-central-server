<template>
  <Teleport to="body">
    <div v-if="authStore.isAuthenticated" class="global-friends-root">
      <button
        class="friends-dock-button"
        type="button"
        :aria-expanded="drawerOpen"
        aria-label="Ouvrir la liste d'amis"
        @click="toggleDrawer"
      >
        <span class="dock-icon">👥</span>
        <span v-if="friendsStore.incomingRequestCount > 0" class="dock-badge">{{ friendsStore.incomingRequestCount }}</span>
      </button>

      <div v-if="drawerOpen" class="friends-drawer-backdrop" @click.self="closeDrawer">
        <aside class="friends-drawer" role="dialog" aria-modal="true" aria-labelledby="global-friends-title">
          <header class="drawer-header">
            <div>
              <p class="drawer-eyebrow">VoxiCraft social</p>
              <h2 id="global-friends-title">Amis</h2>
            </div>
            <button class="drawer-close" type="button" aria-label="Fermer" @click="closeDrawer">✕</button>
          </header>

          <div class="drawer-actions">
            <router-link class="drawer-action-primary" to="/friends" title="Ajouter" aria-label="Ajouter" @click="closeDrawer">➕</router-link>
            <button class="drawer-action-secondary" type="button" title="Rafraîchir" aria-label="Rafraîchir" :disabled="friendsStore.loading" @click="refreshAll">
              {{ friendsStore.loading ? '…' : '↻' }}
            </button>
          </div>

          <section v-if="friendsStore.incomingRequests.length > 0" class="drawer-section alert-section">
            <div class="section-title-row">
              <h3>📥 Demandes reçues</h3>
              <span class="section-count">{{ friendsStore.incomingRequests.length }}</span>
            </div>
            <div class="request-list compact-list">
              <article v-for="request in friendsStore.incomingRequests" :key="request.id" class="mini-card request-mini-card">
                <div class="mini-user">
                  <img :src="avatarSrc(request.requester.avatar.url)" :alt="request.requester.avatar.name" class="mini-avatar" />
                  <div class="mini-user-info">
                    <strong :title="request.requester.username">{{ request.requester.username }}</strong>
                    <span>Le {{ formatDate(request.created_at) }}</span>
                  </div>
                </div>
                <div class="mini-actions two-actions">
                  <button class="mini-button accept" type="button" title="Accepter" aria-label="Accepter" @click="acceptRequest(request.id)">✓</button>
                  <button class="mini-button refuse" type="button" title="Refuser" aria-label="Refuser" @click="refuseRequest(request.id)">✕</button>
                </div>
              </article>
            </div>
          </section>

          <section class="drawer-section">
            <div class="section-title-row">
              <h3>✅ Mes amis</h3>
              <span class="section-count">{{ friendsStore.friends.length }}</span>
            </div>
            <div v-if="friendsStore.friends.length === 0" class="empty-drawer-state">
              <div>👥</div>
              <p>Aucun ami pour le moment.</p>
              <router-link to="/friends" @click="closeDrawer">Ajouter un ami</router-link>
            </div>
            <div v-else class="compact-list friends-compact-list">
              <article v-for="entry in friendsStore.friends" :key="entry.user.id" class="mini-card friend-mini-card">
                <div class="mini-user">
                  <img :src="avatarSrc(entry.user.avatar.url)" :alt="entry.user.avatar.name" class="mini-avatar" />
                  <div class="mini-user-info">
                    <strong :title="entry.user.username">{{ entry.user.username }}</strong>
                    <span>{{ entry.user.avatar.name }}</span>
                    <span v-if="friendPresenceServer(entry.user.id)" class="mini-presence online" :title="friendPresenceServer(entry.user.id)?.name">📍 {{ friendPresenceServer(entry.user.id)?.name }}</span>
                    <span v-else class="mini-presence offline">hors jeu</span>
                  </div>
                </div>
                <div class="mini-actions friend-mini-actions">
                  <button
                    v-if="friendPresenceServer(entry.user.id)"
                    class="mini-join-button"
                    type="button"
                    title="Rejoindre"
                    aria-label="Rejoindre"
                    @click="joinServer(friendPresenceServer(entry.user.id))"
                  >🚪</button>
                  <router-link
                    class="mini-profile-button"
                    :to="`/users/${entry.user.id}`"
                    title="Voir le profil"
                    aria-label="Voir le profil"
                    @click="closeDrawer"
                  >👤</router-link>
                  <button class="mini-trash" type="button" title="Supprimer" aria-label="Supprimer cet ami" @click="removeFriend(entry.user.id)">🗑️</button>
                </div>
              </article>
            </div>
          </section>

          <section v-if="friendsStore.outgoingRequests.length > 0" class="drawer-section">
            <div class="section-title-row">
              <h3>📤 Envoyées</h3>
              <span class="section-count">{{ friendsStore.outgoingRequests.length }}</span>
            </div>
            <div class="compact-list">
              <article v-for="request in friendsStore.outgoingRequests" :key="request.id" class="mini-card friend-mini-card">
                <div class="mini-user">
                  <img :src="avatarSrc(request.receiver.avatar.url)" :alt="request.receiver.avatar.name" class="mini-avatar" />
                  <div class="mini-user-info">
                    <strong :title="request.receiver.username">{{ request.receiver.username }}</strong>
                    <span>En attente</span>
                  </div>
                </div>
                <button class="mini-trash" type="button" title="Annuler" aria-label="Annuler la demande" @click="removeFriend(request.receiver.id)">✕</button>
              </article>
            </div>
          </section>
        </aside>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { type FriendPresenceServer } from '@/api'
import { createJoinTicket } from '@/api/join-ticket'
import { useAuthStore } from '@/stores/auth'
import { useFriendsStore } from '@/stores/friends'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const authStore = useAuthStore()
const friendsStore = useFriendsStore()
const drawerOpen = ref(false)
const avatarObjectUrls = ref<Record<string, string>>({})
const loadingAvatarUrls = new Set<string>()
const avatarPlaceholder = 'data:image/svg+xml;utf8,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20viewBox=%220%200%2048%2048%22%3E%3Crect%20width=%2248%22%20height=%2248%22%20rx=%228%22%20fill=%22%23212121%22/%3E%3Crect%20x=%2214%22%20y=%2210%22%20width=%2220%22%20height=%2220%22%20fill=%22%2364ffda%22%20opacity=%22.75%22/%3E%3Crect%20x=%2210%22%20y=%2234%22%20width=%2228%22%20height=%228%22%20fill=%22%2364ffda%22%20opacity=%22.45%22/%3E%3C/svg%3E'

async function toggleDrawer() {
  drawerOpen.value = !drawerOpen.value
  if (drawerOpen.value) await refreshDrawerPresence()
}

function closeDrawer() {
  drawerOpen.value = false
}

async function refreshAll() {
  await friendsStore.fetchAll()
  await loadVisibleAvatars()
}

async function refreshDrawerPresence() {
  if (friendsStore.friends.length === 0 && !friendsStore.loading) {
    await friendsStore.fetchAll()
  } else {
    await friendsStore.fetchPresence()
  }
  await loadVisibleAvatars()
}

async function acceptRequest(requestId: string) {
  await friendsStore.acceptRequest(requestId)
  await loadVisibleAvatars()
}

async function refuseRequest(requestId: string) {
  await friendsStore.refuseRequest(requestId)
  await loadVisibleAvatars()
}

async function removeFriend(userId: string) {
  await friendsStore.removeFriend(userId)
  await loadVisibleAvatars()
}

function friendPresenceServer(userId: string) {
  return friendsStore.presenceFor(userId)
}

async function joinServer(server: FriendPresenceServer | null | undefined) {
  if (!server) return
  const ticket = await createJoinTicket(server.id)
  window.open(ticket.join_url, '_blank', 'noopener,noreferrer')
}

function formatDate(value: string) {
  return new Date(value).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })
}

function avatarSrc(path: string) {
  return avatarObjectUrls.value[path] || avatarPlaceholder
}

function collectVisibleAvatarPaths() {
  return Array.from(new Set([
    ...friendsStore.incomingRequests.map((request) => request.requester.avatar.url),
    ...friendsStore.outgoingRequests.map((request) => request.receiver.avatar.url),
    ...friendsStore.friends.map((entry) => entry.user.avatar.url),
  ]))
}

async function loadVisibleAvatars() {
  await Promise.all(collectVisibleAvatarPaths().map((path) => loadProtectedAvatar(path)))
}

async function loadProtectedAvatar(path: string) {
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

function apiAsset(path: string) {
  return path.startsWith('http') ? path : `${API_BASE_URL}${path.startsWith('/api') ? path.slice(4) : path}`
}

function clearAvatarUrls() {
  Object.values(avatarObjectUrls.value).forEach((url) => URL.revokeObjectURL(url))
  avatarObjectUrls.value = {}
}

function handleAuthenticatedState(isAuthenticated: boolean) {
  if (isAuthenticated) {
    void refreshAll()
  } else {
    drawerOpen.value = false
    clearAvatarUrls()
  }
}

onMounted(() => {
  handleAuthenticatedState(authStore.isAuthenticated)
})

watch(
  () => authStore.isAuthenticated,
  (isAuthenticated) => {
    handleAuthenticatedState(isAuthenticated)
  }
)

watch(
  () => [
    friendsStore.incomingRequests.map((request) => request.requester.avatar.url).join('|'),
    friendsStore.outgoingRequests.map((request) => request.receiver.avatar.url).join('|'),
    friendsStore.friends.map((entry) => entry.user.avatar.url).join('|'),
  ],
  () => { void loadVisibleAvatars() },
  { deep: false }
)

onBeforeUnmount(() => clearAvatarUrls())
</script>

<style scoped>
.global-friends-root { position: relative; }
.friends-dock-button { position: fixed; right: 1.25rem; bottom: 1.25rem; z-index: 2100; width: 58px; height: 58px; display: flex; align-items: center; justify-content: center; background: linear-gradient(135deg, #8b4513, #5d2a08); border: 4px solid #5d4037; border-bottom-color: #2f1c10; border-right-color: #2f1c10; border-radius: 14px; color: #fff; box-shadow: 6px 6px 0 rgba(0,0,0,.55); cursor: pointer; }
.friends-dock-button:hover { transform: translate(-1px, -1px); box-shadow: 7px 7px 0 rgba(0,0,0,.6); }
.dock-icon { font-size: 1.7rem; filter: drop-shadow(2px 2px 0 #000); }
.dock-badge { position: absolute; top: -10px; right: -10px; min-width: 24px; height: 24px; padding: 0 .35rem; display: inline-flex; align-items: center; justify-content: center; background: #2e7d32; border: 3px solid #64ffda; color: #fff; border-radius: 999px; font-family: monospace; font-weight: 900; font-size: .78rem; box-shadow: 3px 3px 0 rgba(0,0,0,.6); }
.friends-drawer-backdrop { position: fixed; inset: 0; z-index: 2050; display: flex; justify-content: flex-end; background: rgba(0,0,0,.22); }
.friends-drawer { width: min(390px, calc(100vw - 1rem)); height: calc(100vh - 1rem); margin: .5rem; display: flex; flex-direction: column; gap: 1rem; padding: 1rem; background: linear-gradient(180deg, rgba(139,69,19,.98), rgba(94,45,10,.98)); border: 4px solid #5d4037; border-radius: 12px; box-shadow: -8px 8px 0 rgba(0,0,0,.55); overflow: hidden; color: #fff; font-family: monospace; }
.drawer-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem; padding-bottom: .75rem; border-bottom: 2px solid rgba(255,255,255,.14); }
.drawer-eyebrow { margin: 0 0 .25rem; color: #d7ccc8; font-size: .75rem; text-transform: uppercase; letter-spacing: .08em; }
.drawer-header h2 { margin: 0; color: #64ffda; text-shadow: 3px 3px 0 rgba(0,0,0,.45); }
.drawer-close { width: 34px; height: 34px; border: 2px solid #b71c1c; background: #f44336; color: #fff; box-shadow: 3px 3px 0 rgba(0,0,0,.45); cursor: pointer; font-weight: 900; }
.drawer-actions { display: grid; grid-template-columns: minmax(0, 1fr) 44px; gap: .65rem; }
.drawer-action-primary, .drawer-action-secondary { min-height: 40px; display: inline-flex; align-items: center; justify-content: center; color: #fff; text-decoration: none; background: #2e7d32; border: 3px solid #1b5e20; box-shadow: 3px 3px 0 rgba(0,0,0,.45); font-family: monospace; font-weight: 900; cursor: pointer; }
.drawer-action-secondary { background: #6d4c41; border-color: #4e342e; }
.drawer-action-secondary:disabled { opacity: .55; cursor: not-allowed; }
.drawer-section { min-height: 0; display: flex; flex-direction: column; gap: .65rem; }
.alert-section { flex: 0 0 auto; }
.section-title-row { display: flex; align-items: center; justify-content: space-between; gap: 1rem; }
.section-title-row h3 { margin: 0; color: #ffd700; font-size: .95rem; }
.section-count { min-width: 22px; height: 22px; display: inline-flex; align-items: center; justify-content: center; border-radius: 999px; background: #2e7d32; border: 2px solid #64ffda; color: #fff; font-size: .75rem; font-weight: 900; }
.compact-list { display: flex; flex-direction: column; gap: .6rem; min-height: 0; overflow: auto; padding-right: .15rem; }
.friends-compact-list { flex: 1; }
.mini-card { display: grid; grid-template-columns: minmax(0, 1fr) auto; align-items: center; gap: .65rem; padding: .65rem; background: rgba(0,0,0,.25); border: 2px solid rgba(255,255,255,.12); border-radius: 8px; }
.request-mini-card { grid-template-columns: 1fr; }
.mini-user { display: grid; grid-template-columns: 42px minmax(0, 1fr); align-items: center; gap: .65rem; min-width: 0; }
.mini-avatar { width: 42px; height: 42px; object-fit: contain; background: rgba(255,255,255,.08); border: 2px solid #4a4a4a; border-radius: 8px; }
.mini-user-info { min-width: 0; }
.mini-user-info strong { display: block; color: #fff; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; line-height: 1.1; }
.mini-user-info span { display: block; color: #d7ccc8; font-size: .78rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; margin-top: .2rem; }
.mini-presence.online { color: #a5d6a7; }
.mini-presence.offline { color: #a8a8a8; font-style: italic; }
.mini-actions { display: flex; gap: .45rem; }
.friend-mini-actions { flex: 0 0 auto; }
.two-actions { display: grid; grid-template-columns: 1fr 1fr; }
.mini-button, .mini-trash, .mini-profile-button, .mini-join-button { min-width: 36px; height: 34px; display: inline-flex; align-items: center; justify-content: center; border: 2px solid #3e2723; color: #fff; font-weight: 900; box-shadow: 2px 2px 0 rgba(0,0,0,.5); cursor: pointer; text-decoration: none; }
.mini-join-button { background: #1976d2; border-color: #0d47a1; }
.mini-join-button:hover { background: #2196f3; }
.mini-button.accept, .mini-profile-button { background: #2e7d32; border-color: #1b5e20; }
.mini-profile-button:hover { background: #43a047; }
.mini-button.refuse, .mini-trash { background: #f44336; border-color: #c62828; }
.empty-drawer-state { padding: 1rem; text-align: center; color: #d7ccc8; background: rgba(0,0,0,.18); border: 2px dashed rgba(255,255,255,.18); border-radius: 8px; }
.empty-drawer-state div { font-size: 2.25rem; }
.empty-drawer-state p { margin: .5rem 0; }
.empty-drawer-state a { color: #64ffda; font-weight: 900; }
@media (max-width: 640px) { .friends-dock-button { right: .85rem; bottom: .85rem; width: 52px; height: 52px; } .friends-drawer-backdrop { justify-content: center; align-items: flex-end; } .friends-drawer { width: calc(100vw - .75rem); height: min(78vh, 680px); margin: .375rem; border-radius: 12px 12px 0 0; box-shadow: 0 -6px 0 rgba(0,0,0,.45); } }
</style>
