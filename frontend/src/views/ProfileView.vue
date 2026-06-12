<template>
  <div class="profile voxicraft-bg">
    <div class="voxicraft-container">
      <div class="profile-header voxicraft-panel">
        <div class="avatar-display">
          <div class="avatar-frame">
            <img
              :src="`/avatars/${authStore.user?.avatar || 'steve'}.png`"
              :alt="authStore.user?.avatar || 'avatar'"
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
          <router-link to="/profile/avatar-builder" class="btn-avatar primary-avatar-action">
            <span class="btn-icon">🎨</span>
            Créer son avatar
          </router-link>
          <router-link to="/profile/avatar-builder" class="btn-avatar secondary-avatar-action">
            <span class="btn-icon">🧩</span>
            Modifier son avatar
          </router-link>
          <button @click="showEditForm = !showEditForm" class="btn-edit">
            <span class="btn-icon">{{ showEditForm ? '❌' : '✏️' }}</span>
            {{ showEditForm ? 'Annuler' : 'Modifier' }}
          </button>
        </div>
      </div>

      <div v-if="authStore.user" class="profile-content">
        <div class="left-column">
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

          <div class="bio-card voxicraft-panel">
            <h2 class="card-title">
              <span class="title-icon">📝</span>
              À propos
            </h2>
            <div class="bio-content">
              {{ authStore.user.bio || 'Aucune bio renseignée. Parlez-nous de vous !' }}
            </div>
          </div>

          <div class="quick-actions-card voxicraft-panel">
            <h2 class="card-title">
              <span class="title-icon">⚡</span>
              Actions rapides
            </h2>
            <div class="actions-grid">
              <router-link to="/profile/avatar-builder" class="action-btn avatar-action primary-avatar-action">
                <span class="action-icon">🎨</span>
                <span class="action-label">Créer son avatar</span>
              </router-link>
              <router-link to="/profile/avatar-builder" class="action-btn avatar-action secondary-avatar-action">
                <span class="action-icon">🧩</span>
                <span class="action-label">Modifier son avatar</span>
              </router-link>
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

        <div class="right-column">
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
                  Avatar rapide
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
                <router-link to="/profile/avatar-builder" class="inline-avatar-builder-link">
                  Ouvrir le builder graphique d'avatar
                </router-link>
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
import { computed, onMounted, ref } from 'vue'
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
    year: 'numeric',
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

.profile-header,
.profile-content {
  max-width: 1200px;
  width: 100%;
  margin: 0 auto;
}

.profile-header {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: 2rem;
  align-items: center;
  margin-bottom: 3rem;
}

.voxicraft-panel {
  background: rgba(139, 69, 19, 0.9);
  border: 4px solid #5d4037;
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 8px 8px 0 rgba(0, 0, 0, 0.5);
  box-sizing: border-box;
  min-width: 0;
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

.avatar-image,
.avatar-preview-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  image-rendering: pixelated;
  image-rendering: -moz-crisp-edges;
  image-rendering: crisp-edges;
}

.avatar-image {
  border-radius: 10px;
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

.username {
  font-size: 2rem;
  color: #64ffda;
  margin-bottom: 0.5rem;
  text-shadow: 3px 3px 0 rgba(0, 0, 0, 0.5);
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.user-email,
.meta-item,
.bio-content,
.security-desc,
.stat-label {
  color: rgba(255, 255, 255, 0.75);
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
  background: rgba(0, 0, 0, 0.3);
  padding: 0.5rem 1rem;
  border-radius: 50px;
  border: 2px solid rgba(100, 255, 218, 0.2);
}

.header-actions {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.btn-edit,
.btn-avatar,
.btn-submit,
.btn-cancel,
.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  color: #fff;
  font-family: 'Press Start 2P', cursive;
  cursor: pointer;
  border-radius: 5px;
  text-decoration: none;
  transition: all 0.3s ease;
}

.btn-edit,
.btn-avatar {
  padding: 0.8rem 1.5rem;
  border: 3px solid;
  font-size: 0.7rem;
}

.btn-edit {
  background: linear-gradient(135deg, #2196f3, #42a5f5);
  border-color: #1565c0;
}

.btn-avatar.primary-avatar-action,
.action-btn.primary-avatar-action {
  background: linear-gradient(135deg, #8e24aa, #ba68c8);
  border-color: #6a1b9a;
}

.btn-avatar.secondary-avatar-action,
.action-btn.secondary-avatar-action {
  background: linear-gradient(135deg, #ff8f00, #ffb300);
  border-color: #ef6c00;
  color: #1a1a1a;
}

.btn-edit:hover,
.btn-avatar:hover,
.action-btn:hover,
.btn-submit:hover:not(:disabled),
.btn-cancel:hover {
  transform: translateY(-3px);
}

.profile-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.left-column,
.right-column {
  display: flex;
  flex-direction: column;
  gap: 2rem;
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

.info-list,
.edit-form,
.security-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.info-item,
.security-item,
.bio-content,
.stat-item {
  padding: 1rem;
  background: rgba(0, 0, 0, 0.3);
  border: 2px solid rgba(100, 255, 218, 0.2);
  border-radius: 8px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.info-label,
.form-label {
  color: #ffd700;
  font-size: 0.75rem;
}

.info-value {
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
  color: #64ffda;
  text-transform: capitalize;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 1rem;
}

.action-btn {
  flex-direction: column;
  padding: 1.5rem 1rem;
  background: rgba(0, 0, 0, 0.3);
  border: 3px solid rgba(100, 255, 218, 0.2);
}

.action-icon {
  font-size: 2rem;
}

.action-label {
  font-size: 0.65rem;
  text-align: center;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.form-input {
  background: rgba(0, 0, 0, 0.6);
  border: 3px solid #424242;
  color: white;
  padding: 0.75rem;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  width: 100%;
  outline: none;
  border-radius: 5px;
  box-sizing: border-box;
}

.form-input:focus {
  border-color: #64ffda;
}

.form-textarea {
  resize: vertical;
  min-height: 100px;
  line-height: 1.8;
}

.char-count {
  text-align: right;
  font-size: 0.65rem;
  color: rgba(255, 255, 255, 0.5);
}

.avatar-selector {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
  gap: 1rem;
}

.avatar-option {
  cursor: pointer;
  position: relative;
}

.avatar-option input[type='radio'] {
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
}

.avatar-option.selected .avatar-preview {
  border-color: #64ffda;
  background: rgba(100, 255, 218, 0.2);
}

.avatar-preview-img {
  width: 80px;
  height: 80px;
  border-radius: 8px;
}

.inline-avatar-builder-link {
  color: #64ffda;
  font-size: 0.75rem;
  text-decoration: underline;
}

.form-actions {
  display: flex;
  gap: 1rem;
}

.btn-submit,
.btn-cancel {
  flex: 1;
  padding: 0.9rem 1.5rem;
  border: 3px solid;
  font-size: 0.75rem;
}

.btn-submit {
  background: linear-gradient(135deg, #4caf50, #66bb6a);
  border-color: #2e7d32;
}

.btn-submit:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-cancel {
  background: rgba(255, 107, 107, 0.2);
  border-color: #ff6b6b;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 1rem;
}

.stat-item,
.security-item {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.stat-item {
  flex-direction: column;
  text-align: center;
}

.stat-icon,
.security-icon {
  font-size: 2rem;
}

.stat-value,
.security-title {
  color: #64ffda;
  font-weight: bold;
}

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

@media (max-width: 900px) {
  .profile-content,
  .profile-header {
    grid-template-columns: 1fr;
  }

  .profile-header {
    text-align: center;
  }

  .avatar-display {
    margin: 0 auto;
  }

  .header-actions {
    align-items: stretch;
  }
}

@media (max-width: 768px) {
  .profile {
    padding: 1rem 0.5rem;
  }

  .voxicraft-panel {
    padding: 1.5rem;
  }

  .actions-grid,
  .stats-grid,
  .avatar-selector {
    grid-template-columns: 1fr;
  }

  .form-actions {
    flex-direction: column;
  }

  .avatar-frame {
    width: 80px;
    height: 80px;
  }
}
</style>
