<template>
  <nav class="navbar">
    <div class="navbar-container">
      <router-link to="/" class="navbar-brand">
        <span class="brand-icon">⛏️</span>
        <span class="brand-text">Voxicraft VR</span>
      </router-link>
      
      <div class="navbar-menu">
        <template v-if="authStore.isAuthenticated">
          <router-link to="/servers" class="navbar-link">
            🖥️ Mes Serveurs
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
  </nav>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
const router = useRouter()
const mobileMenuOpen = ref(false)

const toggleMobileMenu = () => {
  mobileMenuOpen.value = !mobileMenuOpen.value
}

const closeMobileMenu = () => {
  mobileMenuOpen.value = false
}

const handleLogout = () => {
  authStore.logout()
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
  font-size: 1.5rem;
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
}
</style>
