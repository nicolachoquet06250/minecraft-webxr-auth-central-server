<template>
  <nav class="navbar">
    <div class="navbar-container">
      <router-link to="/" class="navbar-brand">
        <span class="brand-icon"><img src="/favicon.png" alt="Voxicraft VR" /></span>
        <span class="brand-text">Voxicraft VR</span>
      </router-link>
      
      <div class="navbar-menu">
        <template v-if="authStore.isAuthenticated">
          <router-link to="/servers" class="navbar-link">
            🖥️ Mes Serveurs
          </router-link>
          <router-link to="/friends" class="navbar-link friends-nav-link">
            <span>👥 Amis</span>
            <span v-if="friendsStore.incomingRequestCount > 0" class="friend-request-badge">{{ friendsStore.incomingRequestCount }}</span>
          </router-link>
          <router-link to="/profile" class="navbar-link">
            👤 Profil
          </router-link>
          <button @click="handleLogout" class="navbar-link logout-btn">
            🚪 Déconnexion
          </button>
        </template>
        <template v-else>
          <router-link to="/login" class="navbar-link">
            🔐 Connexion
          </router-link>
          <router-link to="/register" class="navbar-link navbar-link-primary">
            ✨ S'inscrire
          </router-link>
        </template>
      </div>
      
      <!-- Mobile menu toggle -->
      <button @click="toggleMobileMenu" class="mobile-menu-toggle">
        {{ mobileMenuOpen ? '✕' : '☰' }}
      </button>
    </div>
    
    <!-- Mobile menu -->
    <div v-if="mobileMenuOpen" class="mobile-menu">
      <template v-if="authStore.isAuthenticated">
        <router-link to="/servers" class="mobile-link" @click="closeMobileMenu">
          🖥️ Mes Serveurs
        </router-link>
        <router-link to="/friends" class="mobile-link mobile-friends-link" @click="closeMobileMenu">
          <span>👥 Amis</span>
          <span v-if="friendsStore.incomingRequestCount > 0" class="friend-request-badge">{{ friendsStore.incomingRequestCount }}</span>
        </router-link>
        <router-link to="/profile" class="mobile-link" @click="closeMobileMenu">
          👤 Profil
        </router-link>
        <button @click="handleLogout" class="mobile-link logout-btn">
          🚪 Déconnexion
        </button>
      </template>
      <template v-else>
        <router-link to="/login" class="mobile-link" @click="closeMobileMenu">
          🔐 Connexion
        </router-link>
        <router-link to="/register" class="mobile-link" @click="closeMobileMenu">
          ✨ S'inscrire
        </router-link>
      </template>
    </div>

    <Teleport to="body">
      <div v-if="friendNotification" class="friend-notification" role="status" aria-live="polite">
        <div class="notification-icon">📥</div>
        <div>
          <strong>Nouvelle demande d'ami</strong>
          <p>{{ friendNotification }}</p>
        </div>
        <router-link to="/friends" class="notification-action" @click="friendNotification = null">Voir</router-link>
      </div>
    </Teleport>
  </nav>
</template>

<script setup lang="ts">
import { onBeforeUnmount, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useFriendsStore } from '@/stores/friends'

const authStore = useAuthStore()
const friendsStore = useFriendsStore()
const router = useRouter()
const mobileMenuOpen = ref(false)
const friendNotification = ref<string | null>(null)
let pollInterval: number | undefined
let notificationTimeout: number | undefined
let initializedFriendPolling = false

const toggleMobileMenu = () => {
  mobileMenuOpen.value = !mobileMenuOpen.value
}

const closeMobileMenu = () => {
  mobileMenuOpen.value = false
}

const showFriendNotification = (username: string) => {
  friendNotification.value = `${username} vous a envoyé une invitation.`
  if (notificationTimeout) window.clearTimeout(notificationTimeout)
  notificationTimeout = window.setTimeout(() => {
    friendNotification.value = null
  }, 6500)
}

const refreshIncomingRequests = async () => {
  if (!authStore.isAuthenticated) return
  const newRequests = await friendsStore.refreshIncomingRequests()
  if (initializedFriendPolling && newRequests.length > 0) {
    showFriendNotification(newRequests[0].requester.username)
  }
  initializedFriendPolling = true
}

const startFriendPolling = async () => {
  if (pollInterval || !authStore.isAuthenticated) return
  await refreshIncomingRequests()
  pollInterval = window.setInterval(() => {
    void refreshIncomingRequests()
  }, 20000)
}

const stopFriendPolling = () => {
  if (pollInterval) window.clearInterval(pollInterval)
  pollInterval = undefined
  initializedFriendPolling = false
  friendNotification.value = null
}

watch(
  () => authStore.isAuthenticated,
  (isAuthenticated) => {
    if (isAuthenticated) void startFriendPolling()
    else stopFriendPolling()
  },
  { immediate: true }
)

onBeforeUnmount(() => {
  stopFriendPolling()
  if (notificationTimeout) window.clearTimeout(notificationTimeout)
})

const handleLogout = () => {
  authStore.logout()
  stopFriendPolling()
  closeMobileMenu()
  router.push('/')
}
</script>

<style scoped>
.navbar {
  background-color: rgba(0, 0, 0, 0.8);
  border-bottom: 4px solid #4a4a4a;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
  position: sticky;
  top: 0;
  z-index: 1000;
}

.navbar-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 1rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.navbar-brand {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  text-decoration: none;
  color: #fff;
  font-family: 'Voxicraft', monospace;
  font-size: 1.25rem;
  font-weight: bold;
  text-shadow: 2px 2px 0 #000;
  transition: transform 0.2s;
}

.navbar-brand:hover {
  transform: scale(1.05);
}

.brand-icon {
  display: flex;
  
  img {
    width: 2.5rem;
    height: 2.5rem;
  }
}

.navbar-menu {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.navbar-link {
  color: #fff;
  text-decoration: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  font-family: 'Voxicraft', monospace;
  text-shadow: 1px 1px 0 #000;
  transition: all 0.2s;
  background: none;
  border: 2px solid transparent;
  cursor: pointer;
  font-size: 1rem;
}

.navbar-link:hover {
  background-color: rgba(255, 255, 255, 0.1);
  border-color: #64ffda;
}

.navbar-link.router-link-active {
  background-color: rgba(100, 255, 218, 0.2);
  border-color: #64ffda;
}

.friends-nav-link,
.mobile-friends-link {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: .45rem;
}

.friend-request-badge {
  min-width: 1.25rem;
  height: 1.25rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0 .35rem;
  border-radius: 999px;
  background: #2e7d32;
  border: 2px solid #64ffda;
  color: #fff;
  font-size: .72rem;
  line-height: 1;
  font-family: monospace;
  font-weight: 900;
  box-shadow: 2px 2px 0 rgba(0,0,0,.65);
}

.navbar-link-primary {
  background-color: #4caf50;
  border-color: #2e7d32;
}

.navbar-link-primary:hover {
  background-color: #66bb6a;
  border-color: #4caf50;
}

.logout-btn {
  color: #ff6b6b;
}

.logout-btn:hover {
  background-color: rgba(255, 107, 107, 0.2);
  border-color: #ff6b6b;
}

.mobile-menu-toggle {
  display: none;
  background: none;
  border: 2px solid #64ffda;
  color: #fff;
  font-size: 1.5rem;
  padding: 0.5rem;
  border-radius: 4px;
  cursor: pointer;
  font-family: 'Voxicraft', monospace;
}

.mobile-menu {
  display: none;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1rem 2rem;
  background-color: rgba(0, 0, 0, 0.95);
  border-top: 2px solid #4a4a4a;
}

.mobile-link {
  color: #fff;
  text-decoration: none;
  padding: 0.75rem 1rem;
  border-radius: 4px;
  font-family: 'Voxicraft', monospace;
  text-shadow: 1px 1px 0 #000;
  transition: all 0.2s;
  background: none;
  border: 2px solid transparent;
  cursor: pointer;
  font-size: 1rem;
  text-align: left;
  width: 100%;
}

.mobile-link:hover {
  background-color: rgba(255, 255, 255, 0.1);
  border-color: #64ffda;
}

.friend-notification {
  position: fixed;
  right: 1rem;
  top: 5.5rem;
  z-index: 3000;
  width: min(360px, calc(100vw - 2rem));
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: .8rem;
  align-items: center;
  padding: 1rem;
  background: rgba(20, 55, 18, .96);
  border: 3px solid #64ffda;
  border-radius: 10px;
  box-shadow: 6px 6px 0 rgba(0,0,0,.55);
  color: #fff;
  font-family: monospace;
}

.notification-icon { font-size: 1.5rem; }
.friend-notification strong { color: #64ffda; display: block; margin-bottom: .25rem; }
.friend-notification p { margin: 0; color: #d7ccc8; font-size: .85rem; }
.notification-action { color: #101820; background: #64ffda; border: 2px solid #3e2723; padding: .45rem .65rem; text-decoration: none; font-weight: 900; box-shadow: 2px 2px 0 rgba(0,0,0,.55); }

@media (max-width: 768px) {
  .navbar-menu {
    display: none;
  }
  
  .mobile-menu-toggle {
    display: block;
  }
  
  .mobile-menu {
    display: flex;
  }

  .mobile-friends-link {
    justify-content: space-between;
  }

  .friend-notification {
    top: auto;
    right: .5rem;
    left: .5rem;
    bottom: .75rem;
    width: auto;
  }
}
</style>
