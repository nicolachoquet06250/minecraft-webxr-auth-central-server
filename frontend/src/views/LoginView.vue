<template>
  <div class="login minecraft-bg">
    <div class="minecraft-panel">
      <h1 class="minecraft-title">Connexion</h1>
      
      <form @submit.prevent="handleLogin" class="login-form">
        <div class="form-group">
          <label class="minecraft-label">Email</label>
          <input 
            v-model="loginData.email" 
            type="email" 
            class="minecraft-input" 
            required 
          />
        </div>
        
        <div class="form-group">
          <label class="minecraft-label">Mot de passe</label>
          <input 
            v-model="loginData.password" 
            type="password" 
            class="minecraft-input" 
            required 
          />
        </div>
        
        <button type="submit" class="minecraft-button" :disabled="authStore.loading">
          {{ authStore.loading ? 'Connexion...' : 'Se connecter' }}
        </button>
        
        <div v-if="authStore.error" class="error-message">
          {{ authStore.error }}
        </div>
      </form>
      
      <div class="divider">
        <span>OU</span>
      </div>
      
      <button @click="loginWithDiscord" class="minecraft-button discord-button">
        Se connecter avec Discord
      </button>
      
      <p class="minecraft-text">
        Pas encore de compte ? 
        <router-link to="/register" class="link">S'inscrire</router-link>
      </p>
      
      <router-link to="/" class="back-link">
        ← Retour à l'accueil
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const loginData = ref({
  email: '',
  password: '',
})

const handleLogin = async () => {
  const success = await authStore.login(loginData.value)
  if (success) {
    router.push('/profile')
  }
}

const loginWithDiscord = async () => {
  const url = await authStore.getDiscordAuthUrl()
  if (url) {
    window.location.href = url
  }
}
</script>

<style scoped>
.login {
  min-height: calc(100vh - 80px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem 1rem;
}

.minecraft-panel {
  width: 100%;
  max-width: 450px;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  text-align: left;
}

.divider {
  text-align: center;
  margin: 1rem 0;
  position: relative;
}

.divider span {
  background-color: rgba(0, 0, 0, 0.8);
  padding: 0 1rem;
  position: relative;
  z-index: 1;
}

.divider::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 0;
  right: 0;
  height: 2px;
  background-color: #4a4a4a;
}

.discord-button {
  background-color: #5865F2;
  border-color: #4752c4;
  margin-bottom: 1rem;
}

.discord-button:hover {
  background-color: #6b77ff;
}

.error-message {
  color: #ff4444;
  margin-top: 1rem;
  background-color: rgba(255, 68, 68, 0.1);
  padding: 0.75rem;
  border-radius: 4px;
  border: 2px solid #ff4444;
}

.link {
  color: #64ffda;
  text-decoration: none;
  font-weight: bold;
}

.link:hover {
  text-decoration: underline;
}

.minecraft-text {
  text-align: center;
  margin-bottom: 1rem;
}

.back-link {
  display: block;
  text-align: center;
  color: #aaa;
  text-decoration: none;
  margin-top: 1rem;
}

.back-link:hover {
  color: #64ffda;
}
</style>
