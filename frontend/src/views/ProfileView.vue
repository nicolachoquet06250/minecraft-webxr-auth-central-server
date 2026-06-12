<template>
  <div class="profile voxicraft-bg">
    <div class="voxicraft-container">
      <!-- Header avec avatar -->
      <div class="profile-header">
        <div class="avatar-display">
          <div class="avatar-frame">
            <img 
              :src="`/avatars/${authStore.user?.avatar || 'steve'}.png`" 
              :alt="authStore.user?.avatar"
              class="avatar-image"
            />
          </div>
          <div class="user-badge" v-if="authStore.user?.age_verified">
            <span class="badge-icon">✓</span>
            <span class="badge-text">Vérifié</span>
          </div>
        </div>
        <div class="header-info">
          <h1 class="username">{{ authStore.user?.username }}</h1>
          <p class="user-email">{{ authStore.user?.email }}</p>
          <div class="user-meta">
            <span class="meta-item">
              <span class="meta-icon">📅</span>
              Membre depuis {{ formatDate(authStore.user?.created_at) }}
            </span>
          </div>
        </div>
        <div class="header-actions">
          <button @click="showEditForm = !showEditForm" class="btn-edit">
            <span class="btn-icon">{{ showEditForm ? '❌' : '✏️' }}</span>
            {{ showEditForm ? 'Annuler' : 'Modifier' }}
          </button>
        </div>
      </div>

      <div v-if="authStore.user" class="profile-content">
        <!-- Colonne gauche - Informations -->
        <div class="left-column">
          <!-- Carte Informations personnelles -->
          <div class="info-card voxicraft-panel">
            <h2 class="card-title">
              <span class="title-icon">👤</span>
              Informations personnelles
            </h2>
            <div class="info-list">
              <div class="info-item">
                <span class="info-label">🎭 Avatar</span>
                <span class="info-value avatar-name">{{ authStore.user.avatar }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">🎂 Date de naissance</span>
                <span class="info-value">{{ formatDate(authStore.user.birthdate) }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">✅ Âge vérifié</span>
                <span class="info-value" :class="authStore.user.age_verified ? 'verified' : 'not-verified'">
                  {{ authStore.user.age_verified ? 'Oui' : 'Non' }}
                </span>
              </div>
              <div class="info-item" v-if="authStore.user.discord_username">
                <span class="info-label">💬 Discord</span>
                <span class="info-value">{{ authStore.user.discord_username }}</span>
              </div>
            </div>
          </div>

          <!-- Carte Bio -->
          <div class="bio-card voxicraft-panel">
            <h2 class="card-title">
              <span class="title-icon">📝</span>
              À propos
            </h2>
            <div class="bio-content">
              {{ authStore.user.bio || 'Aucune bio renseignée. Parlez-nous de vous !' }}
            </div>
          </div>

          <!-- Carte Actions rapides -->
          <div class="quick-actions-card voxicraft-panel">
            <h2 class="card-title">
              <span class="title-icon">⚡</span>
              Actions rapides
            </h2>
            <div class="actions-grid">
              <router-link to="/servers" class="action-btn">
                <span class="action-icon">🖥️</span>
                <span class="action-label">Mes serveurs</span>
              </router-link>
              <button @click="showEditForm = true" class="action-btn" v-if="!showEditForm">
                <span class="action-icon">✏️</span>
                <span class="action-label">Modifier profil</span>
              </button>
              <a href="https://github.com/nicolachoquet06250/minecraft-webxr" target="_blank" class="action-btn">
                <span class="action-icon">⭐</span>
                <span class="action-label">GitHub</span>
              </a>
            </div>
          </div>
        </div>

        <!-- Colonne droite - Édition -->
        <div class="right-column">
          <!-- Formulaire d'édition -->
          <div v-if="showEditForm" class="edit-card voxicraft-panel">
            <h2 class="card-title">
              <span class="title-icon">✏️</span>
              Modifier mon profil
            </h2>
            <form @submit.prevent="handleUpdate" class="edit-form">
              <div class="form-group">
                <label class="form-label">
                  <span class="label-icon">👤</span>
                  Pseudo
                </label>
                <input 
                  v-model="editData.username" 
                  type="text" 
                  class="form-input"
                  minlength="3"
                  maxlength="20"
                  placeholder="Votre pseudo"
                />
              </div>
              
              <div class="form-group">
                <label class="form-label">
                  <span class="label-icon">🎭</span>
                  Avatar
                </label>
                <div class="avatar-selector">
                  <label class="avatar-option" :class="{ selected: editData.avatar === 'steve' }">
                    <input type="radio" v-model="editData.avatar" value="steve" />
                    <div class="avatar-preview">
                      <img src="/avatars/steve.png" alt="Steve" class="avatar-preview-img" />
                      <span class="avatar-name">Steve</span>
                    </div>
                  </label>
                  <label class="avatar-option" :class="{ selected: editData.avatar === 'alex' }">
                    <input type="radio" v-model="editData.avatar" value="alex" />
                    <div class="avatar-preview">
                      <img src="/avatars/alex.png" alt="Alex" class="avatar-preview-img" />
                      <span class="avatar-name">Alex</span>
                    </div>
                  </label>
                </div>
              </div>
              
              <div class="form-group">
                <label class="form-label">
                  <span class="label-icon">📝</span>
                  Bio
                </label>
                <textarea 
                  v-model="editData.bio" 
                  class="form-input form-textarea" 
                  rows="4"
                  placeholder="Parlez-nous de vous..."
                  maxlength="500"
                ></textarea>
                <div class="char-count">{{ editData.bio?.length || 0 }} / 500</div>
              </div>
              
              <div class="form-actions">
                <button type="submit" class="btn-submit" :disabled="authStore.loading">
                  <span class="btn-icon">💾</span>
                  {{ authStore.loading ? 'Enregistrement...' : 'Enregistrer' }}
                </button>
                <button type="button" @click="showEditForm = false" class="btn-cancel">
                  <span class="btn-icon">❌</span>
                  Annuler
                </button>
              </div>
              
              <div v-if="authStore.error" class="error-message">
                <span class="error-icon">⚠️</span>
                {{ authStore.error }}
              </div>
            </form>
          </div>

          <!-- Carte Statistiques (si pas en mode édition) -->
          <div v-else class="stats-card voxicraft-panel">
            <h2 class="card-title">
              <span class="title-icon">📊</span>
              Statistiques
            </h2>
            <div class="stats-grid">
              <div class="stat-item">
                <div class="stat-icon">🖥️</div>
                <div class="stat-content">
                  <div class="stat-value">{{ serverCount }}</div>
                  <div class="stat-label">Serveurs</div>
                </div>
              </div>
              <div class="stat-item">
                <div class="stat-icon">⏱️</div>
                <div class="stat-content">
                  <div class="stat-value">{{ daysSinceJoined }}</div>
                  <div class="stat-label">Jours</div>
                </div>
              </div>
              <div class="stat-item">
                <div class="stat-icon">🎮</div>
                <div class="stat-content">
                  <div class="stat-value">{{ authStore.user.age_verified ? 'Oui' : 'Non' }}</div>
                  <div class="stat-label">Vérifié</div>
                </div>
              </div>
            </div>
          </div>

          <!-- Carte Sécurité -->
          <div v-if="!showEditForm" class="security-card voxicraft-panel">
            <h2 class="card-title">
              <span class="title-icon">🔒</span>
              Sécurité et confidentialité
            </h2>
            <div class="security-list">
              <div class="security-item">
                <span class="security-icon">🔑</span>
                <div class="security-content">
                  <div class="security-title">Mot de passe</div>
                  <div class="security-desc">Dernière modification: Il y a 30 jours</div>
                </div>
              </div>
              <div class="security-item">
                <span class="security-icon">🛡️</span>
                <div class="security-content">
                  <div class="security-title">Authentification</div>
                  <div class="security-desc">JWT Token actif</div>
                </div>
              </div>
              <div class="security-item">
                <span class="security-icon">📧</span>
                <div class="security-content">
                  <div class="security-title">Email</div>
                  <div class="security-desc">{{ authStore.user.email }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useServerStore } from '@/stores/server'

const authStore = useAuthStore()
const serverStore = useServerStore()
const showEditForm = ref(false)
const editData = ref({
  username: '',
  avatar: 'steve',
  bio: '',
})

const serverCount = computed(() => serverStore.servers.length)

const daysSinceJoined = computed(() => {
  if (!authStore.user?.created_at) return 0
  const joinDate = new Date(authStore.user.created_at)
  const now = new Date()
  const diff = now.getTime() - joinDate.getTime()
  return Math.floor(diff / (1000 * 60 * 60 * 24))
})

const formatDate = (dateString: string | undefined) => {
  if (!dateString) return 'N/A'
  const date = new Date(dateString)
  return date.toLocaleDateString('fr-FR', { 
    day: '2-digit', 
    month: 'long', 
    year: 'numeric' 
  })
}

onMounted(async () => {
  if (!authStore.user) {
    await authStore.fetchProfile()
  }
  if (authStore.user) {
    editData.value = {
      username: authStore.user.username,
      avatar: authStore.user.avatar,
      bio: authStore.user.bio || '',
    }
  }
  // Charger les serveurs pour les stats
  if (serverStore.servers.length === 0) {
    await serverStore.fetchUserServers()
  }
})

const handleUpdate = async () => {
  const success = await authStore.updateProfile(editData.value)
  if (success) {
    showEditForm.value = false
  }
}
</script>

<style scoped>
.profile {
  min-height: calc(100vh - 80px);
  padding: 2rem 1rem;
  max-width: 100%;
  overflow-x: hidden;
  box-sizing: border-box;
}

/* ========== Profile Header ========== */
.profile-header {
  max-width: 1200px;
  width: 100%;
  margin: 0 auto 3rem;
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: 2rem;
  align-items: center;
  background: rgba(0, 0, 0, 0.4);
  border: 3px solid rgba(100, 255, 218, 0.3);
  border-radius: 15px;
  padding: 2rem;
  box-sizing: border-box;
}

.avatar-display {
  position: relative;
}

.avatar-frame {
  width: 120px;
  height: 120px;
  background: linear-gradient(135deg, #64ffda, #4caf50);
  border: 4px solid #1a1a1a;
  border-radius: 15px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.4);
  overflow: hidden;
  padding: 5px;
}

.avatar-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 10px;
  image-rendering: pixelated;
  image-rendering: -moz-crisp-edges;
  image-rendering: crisp-edges;
}

.avatar-icon {
  font-size: 4rem;
}

.user-badge {
  position: absolute;
  bottom: -10px;
  right: -10px;
  background: linear-gradient(135deg, #4caf50, #66bb6a);
  border: 3px solid #1a1a1a;
  border-radius: 50px;
  padding: 0.3rem 0.7rem;
  display: flex;
  align-items: center;
  gap: 0.3rem;
  font-size: 0.6rem;
  color: #fff;
  text-transform: uppercase;
  font-weight: bold;
}

.badge-icon {
  font-size: 0.8rem;
}

.header-info {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.username {
  font-size: 2rem;
  color: #64ffda;
  margin-bottom: 0.5rem;
  text-shadow: 3px 3px 0 rgba(0, 0, 0, 0.5);
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.user-email {
  font-size: 0.85rem;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 1rem;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.user-meta {
  display: flex;
  gap: 1.5rem;
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.8);
  background: rgba(0, 0, 0, 0.3);
  padding: 0.5rem 1rem;
  border-radius: 50px;
  border: 2px solid rgba(100, 255, 218, 0.2);
}

.meta-icon {
  font-size: 1rem;
}

.header-actions {
  display: flex;
  gap: 1rem;
}

.btn-edit {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.8rem 1.5rem;
  background: linear-gradient(135deg, #2196f3, #42a5f5);
  border: 3px solid #1565c0;
  color: #fff;
  font-family: 'Press Start 2P', cursive;
  font-size: 0.75rem;
  cursor: pointer;
  border-radius: 5px;
  transition: all 0.3s ease;
}

.btn-edit:hover {
  transform: translateY(-3px);
  box-shadow: 0 6px 15px rgba(33, 150, 243, 0.4);
}

.btn-icon {
  font-size: 1rem;
}

/* ========== Profile Content ========== */
.profile-content {
  max-width: 1200px;
  width: 100%;
  margin: 0 auto;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  box-sizing: border-box;
}

.left-column,
.right-column {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  min-width: 0;
  box-sizing: border-box;
}

/* ========== Cards ========== */
.voxicraft-panel {
  background: rgba(139, 69, 19, 0.9);
  border: 4px solid #5D4037;
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 8px 8px 0 rgba(0, 0, 0, 0.5);
  box-sizing: border-box;
  min-width: 0;
}

.card-title {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  font-size: 1.2rem;
  color: #64ffda;
  margin-bottom: 1.5rem;
  text-shadow: 2px 2px 0 rgba(0, 0, 0, 0.5);
}

.title-icon {
  font-size: 1.5rem;
}

/* ========== Info Card ========== */
.info-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: rgba(0, 0, 0, 0.3);
  border: 2px solid rgba(100, 255, 218, 0.2);
  border-radius: 8px;
}

.info-label {
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.info-value {
  font-size: 0.8rem;
  color: #fff;
  font-weight: bold;
  text-transform: capitalize;
}

.info-value.verified {
  color: #4caf50;
}

.info-value.not-verified {
  color: #ff6b6b;
}

.avatar-name {
  text-transform: capitalize;
  color: #64ffda;
}

/* ========== Bio Card ========== */
.bio-content {
  padding: 1.5rem;
  background: rgba(0, 0, 0, 0.3);
  border: 2px solid rgba(100, 255, 218, 0.2);
  border-radius: 8px;
  font-size: 0.8rem;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.9);
  font-style: italic;
  min-height: 100px;
}

/* ========== Quick Actions ========== */
.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 1rem;
}

.action-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1.5rem 1rem;
  background: rgba(0, 0, 0, 0.3);
  border: 3px solid rgba(100, 255, 218, 0.2);
  border-radius: 10px;
  color: #fff;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.3s ease;
  font-family: 'Press Start 2P', cursive;
}

.action-btn:hover {
  transform: translateY(-5px);
  border-color: #64ffda;
  background: rgba(100, 255, 218, 0.1);
}

.action-icon {
  font-size: 2rem;
}

.action-label {
  font-size: 0.65rem;
  text-align: center;
}

/* ========== Edit Form ========== */
.edit-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.form-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.8rem;
  color: #FFD700;
  text-shadow: 2px 2px 0 rgba(0, 0, 0, 0.5);
}

.label-icon {
  font-size: 1rem;
}

.form-input {
  background: rgba(0, 0, 0, 0.6);
  border: 3px solid #424242;
  color: white;
  padding: 0.75rem;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  width: 100%;
  max-width: 100%;
  outline: none;
  border-radius: 5px;
  transition: border-color 0.3s;
  box-sizing: border-box;
}

.form-input:focus {
  border-color: #64ffda;
}

.form-textarea {
  resize: vertical;
  min-height: 100px;
  font-family: 'Press Start 2P', cursive;
  line-height: 1.8;
  font-size: 0.7rem;
}

.char-count {
  text-align: right;
  font-size: 0.65rem;
  color: rgba(255, 255, 255, 0.5);
}

/* ========== Avatar Selector ========== */
.avatar-selector {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
  gap: 1rem;
}

.avatar-option {
  cursor: pointer;
  position: relative;
}

.avatar-option input[type="radio"] {
  position: absolute;
  opacity: 0;
}

.avatar-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem;
  background: rgba(0, 0, 0, 0.4);
  border: 3px solid rgba(100, 255, 218, 0.2);
  border-radius: 10px;
  transition: all 0.3s ease;
}

.avatar-option:hover .avatar-preview {
  border-color: #64ffda;
  background: rgba(100, 255, 218, 0.1);
}

.avatar-option.selected .avatar-preview {
  border-color: #64ffda;
  background: rgba(100, 255, 218, 0.2);
  box-shadow: 0 0 20px rgba(100, 255, 218, 0.4);
}

.avatar-emoji {
  font-size: 2.5rem;
}

.avatar-preview-img {
  width: 80px;
  height: 80px;
  object-fit: cover;
  border-radius: 8px;
  image-rendering: pixelated;
  image-rendering: -moz-crisp-edges;
  image-rendering: crisp-edges;
}

.avatar-name {
  font-size: 0.7rem;
  color: #fff;
}

/* ========== Form Actions ========== */
.form-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

.btn-submit {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.9rem 1.5rem;
  background: linear-gradient(135deg, #4caf50, #66bb6a);
  border: 3px solid #2e7d32;
  color: #fff;
  font-family: 'Press Start 2P', cursive;
  font-size: 0.75rem;
  cursor: pointer;
  border-radius: 5px;
  transition: all 0.3s ease;
}

.btn-submit:hover:not(:disabled) {
  transform: translateY(-3px);
  box-shadow: 0 6px 15px rgba(76, 175, 80, 0.4);
}

.btn-submit:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-cancel {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.9rem 1.5rem;
  background: rgba(255, 107, 107, 0.2);
  border: 3px solid #ff6b6b;
  color: #fff;
  font-family: 'Press Start 2P', cursive;
  font-size: 0.75rem;
  cursor: pointer;
  border-radius: 5px;
  transition: all 0.3s ease;
}

.btn-cancel:hover {
  background: rgba(255, 107, 107, 0.3);
  transform: translateY(-3px);
}

/* ========== Stats Card ========== */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 1rem;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem 1rem;
  background: rgba(0, 0, 0, 0.3);
  border: 3px solid rgba(100, 255, 218, 0.2);
  border-radius: 10px;
  min-width: 0;
  overflow: hidden;
}

.stat-icon {
  font-size: 2.5rem;
  flex-shrink: 0;
}

.stat-content {
  text-align: center;
  min-width: 0;
  width: 100%;
}

.stat-value {
  font-size: 1.5rem;
  color: #64ffda;
  font-weight: bold;
  margin-bottom: 0.5rem;
  text-shadow: 2px 2px 0 rgba(0, 0, 0, 0.5);
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.stat-label {
  font-size: 0.65rem;
  color: rgba(255, 255, 255, 0.7);
  text-transform: uppercase;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

/* ========== Security Card ========== */
.security-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.security-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: rgba(0, 0, 0, 0.3);
  border: 2px solid rgba(100, 255, 218, 0.2);
  border-radius: 8px;
  min-width: 0;
  overflow: hidden;
}

.security-icon {
  font-size: 2rem;
  flex-shrink: 0;
}

.security-content {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.security-title {
  font-size: 0.8rem;
  color: #64ffda;
  margin-bottom: 0.3rem;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.security-desc {
  font-size: 0.65rem;
  color: rgba(255, 255, 255, 0.7);
  word-wrap: break-word;
  overflow-wrap: break-word;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ========== Error Message ========== */
.error-message {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #ff4444;
  background-color: rgba(255, 68, 68, 0.1);
  padding: 1rem;
  border-radius: 8px;
  border: 2px solid #ff4444;
  font-size: 0.7rem;
}

.error-icon {
  font-size: 1.2rem;
}

/* ========== Responsive ========== */
@media (max-width: 900px) {
  .profile-content {
    grid-template-columns: 1fr;
    gap: 1.5rem;
  }
}

@media (max-width: 768px) {
  .profile {
    padding: 1rem 0.5rem;
  }
  
  .profile-header {
    grid-template-columns: 1fr;
    text-align: center;
    gap: 1.5rem;
    padding: 1.5rem 1rem;
    margin: 0 0.5rem 2rem;
  }
  
  .header-info {
    order: 2;
  }
  
  .avatar-display {
    order: 1;
    margin: 0 auto;
  }
  
  .header-actions {
    order: 3;
    justify-content: center;
  }
  
  .user-meta {
    justify-content: center;
    flex-wrap: wrap;
  }
  
  .profile-content {
    grid-template-columns: 1fr;
    gap: 1.5rem;
    padding: 0 0.5rem;
  }
  
  .avatar-frame {
    width: 80px;
    height: 80px;
  }
  
  .avatar-image {
    border-radius: 8px;
  }
  
  .avatar-icon {
    font-size: 3rem;
  }
  
  .username {
    font-size: 1.3rem;
  }
  
  .user-email {
    font-size: 0.75rem;
  }
  
  .meta-item {
    font-size: 0.65rem;
    padding: 0.4rem 0.8rem;
  }
  
  .btn-edit {
    font-size: 0.65rem;
    padding: 0.7rem 1.2rem;
  }
  
  .voxicraft-panel {
    padding: 1.5rem;
  }
  
  .card-title {
    font-size: 1rem;
    gap: 0.5rem;
  }
  
  .title-icon {
    font-size: 1.2rem;
  }
  
  .info-item {
    padding: 0.8rem;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .info-label {
    font-size: 0.7rem;
  }
  
  .info-value {
    font-size: 0.75rem;
  }
  
  .bio-content {
    padding: 1rem;
    font-size: 0.7rem;
    min-height: 80px;
  }
  
  .actions-grid {
    grid-template-columns: 1fr;
    gap: 0.8rem;
  }
  
  .action-btn {
    padding: 1.2rem 1rem;
  }
  
  .action-icon {
    font-size: 1.8rem;
  }
  
  .action-label {
    font-size: 0.6rem;
  }
  
  .form-label {
    font-size: 0.7rem;
  }
  
  .form-input {
    font-size: 0.8rem;
    padding: 0.6rem;
  }
  
  .form-textarea {
    font-size: 0.65rem;
  }
  
  .char-count {
    font-size: 0.6rem;
  }
  
  .avatar-selector {
    grid-template-columns: 1fr;
  }
  
  .avatar-preview {
    padding: 0.8rem;
  }
  
  .avatar-preview-img {
    width: 60px;
    height: 60px;
  }
  
  .avatar-name {
    font-size: 0.65rem;
  }
  
  .form-actions {
    flex-direction: column;
  }
  
  .btn-submit,
  .btn-cancel {
    font-size: 0.7rem;
    padding: 0.8rem 1.2rem;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
    gap: 0.8rem;
  }
  
  .stat-item {
    padding: 1.2rem 0.8rem;
    width: 100%;
    max-width: 100%;
  }
  
  .stat-icon {
    font-size: 2rem;
  }
  
  .stat-value {
    font-size: 1.2rem;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .stat-label {
    font-size: 0.6rem;
  }
  
  .security-list {
    gap: 0.8rem;
  }
  
  .security-item {
    padding: 0.8rem;
    gap: 0.8rem;
    flex-wrap: nowrap;
    width: 100%;
    max-width: 100%;
  }
  
  .security-icon {
    font-size: 1.5rem;
  }
  
  .security-title {
    font-size: 0.7rem;
  }
  
  .security-desc {
    font-size: 0.6rem;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .error-message {
    font-size: 0.65rem;
    padding: 0.8rem;
  }
}

@media (max-width: 480px) {
  .profile-header {
    padding: 1rem;
  }
  
  .avatar-frame {
    width: 70px;
    height: 70px;
  }
  
  .username {
    font-size: 1.1rem;
  }
  
  .user-meta {
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .voxicraft-panel {
    padding: 1rem;
    max-width: 100%;
  }
  
  .card-title {
    font-size: 0.9rem;
  }
  
  .btn-edit {
    font-size: 0.6rem;
    padding: 0.6rem 1rem;
  }
}
</style>
