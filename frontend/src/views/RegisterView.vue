<template>
  <div class="register voxicraft-bg">
    <div class="voxicraft-panel">
      <h1 class="voxicraft-title">Inscription</h1>
      
      <form @submit.prevent="handleRegister" class="register-form">
        <div class="form-group">
          <label class="voxicraft-label">Pseudo</label>
          <input 
            v-model="registerData.username" 
            type="text" 
            class="voxicraft-input" 
            required 
            minlength="3"
            maxlength="20"
          />
        </div>
        
        <div class="form-group">
          <label class="voxicraft-label">Email</label>
          <input 
            v-model="registerData.email" 
            type="email" 
            class="voxicraft-input" 
            required 
          />
        </div>
        
        <div class="form-group">
          <label class="voxicraft-label">Mot de passe</label>
          <input 
            v-model="registerData.password" 
            type="password" 
            class="voxicraft-input" 
            required 
            minlength="8"
          />
        </div>
        
        <div class="form-group">
          <label class="voxicraft-label">Avatar</label>
          <div class="avatar-selector">
            <label class="avatar-option">
              <input type="radio" v-model="registerData.avatar" value="steve" />
              <span>Steve</span>
            </label>
            <label class="avatar-option">
              <input type="radio" v-model="registerData.avatar" value="alex" />
              <span>Alex</span>
            </label>
          </div>
        </div>
        
        <div class="form-group">
          <label class="voxicraft-label">Date de naissance</label>
          <input 
            v-model="registerData.birthdate" 
            type="date" 
            class="voxicraft-input" 
            required 
          />
        </div>
        
        <div class="form-group">
          <label class="voxicraft-label">Bio (optionnel)</label>
          <textarea 
            v-model="registerData.bio" 
            class="voxicraft-input" 
            rows="3"
          ></textarea>
        </div>
        
        <button type="submit" class="voxicraft-button" :disabled="authStore.loading">
          {{ authStore.loading ? 'Inscription...' : 'S\'inscrire' }}
        </button>
        
        <div v-if="authStore.error" class="error-message">
          {{ authStore.error }}
        </div>
      </form>
      
      <p class="voxicraft-text text-center">
        Déjà un compte ? 
        <router-link to="/login" class="link">Se connecter</router-link>
      </p>
      
      <div class="divider">
        <span>OU</span>
      </div>
      
      <router-link to="/" class="voxicraft-button secondary-btn">
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

const registerData = ref({
  username: '',
  email: '',
  password: '',
  avatar: 'steve',
  birthdate: '',
  bio: '',
})

const handleRegister = async () => {
  const success = await authStore.register(registerData.value)
  if (success) {
    router.push('/profile')
  }
}
</script>

<style scoped>
.register {
  min-height: calc(100vh - 80px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem 1rem;
}

.voxicraft-panel {
  width: 100%;
  max-width: 500px;
}

.register-form {
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

.avatar-selector {
  display: flex;
  gap: 1rem;
}

.avatar-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.error-message {
  color: #ff4444;
  margin-top: 1rem;
}

.link {
  color: #64ffda;
  text-decoration: none;
  font-weight: bold;
}

.link:hover {
  text-decoration: underline;
}

.text-center {
  text-align: center;
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

.secondary-btn {
  background-color: #757575;
  border-color: #424242;
  display: block;
  text-align: center;
  text-decoration: none;
}

.secondary-btn:hover {
  background-color: #9e9e9e;
}
</style>
