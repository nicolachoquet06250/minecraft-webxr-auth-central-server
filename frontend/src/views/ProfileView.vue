<template>
  <div class="profile voxicraft-bg">
    <div class="voxicraft-container">
      <div class="profile-header voxicraft-panel">
        <div class="avatar-display">
          <div class="avatar-frame">
            <AvatarHeadImage :avatar="authStore.user?.avatar" />
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
            <span class="meta-item">📅 Membre depuis {{ formatDate(authStore.user?.created_at) }}</span>
          </div>
        </div>

        <div class="header-actions">
          <router-link to="/profile/avatar-builder" class="btn-avatar primary-avatar-action">🎨 Créer mon avatar</router-link>
          <router-link to="/profile/avatar-builder" class="btn-avatar secondary-avatar-action">🧩 Modifier mon avatar</router-link>
          <button @click="showEditForm = !showEditForm" class="btn-edit">{{ showEditForm ? 'Annuler' : 'Modifier mon profile' }}</button>
        </div>
      </div>

      <div v-if="authStore.user" class="profile-content">
        <div class="left-column">
          <div class="voxicraft-panel">
            <h2 class="card-title">👤 Informations personnelles</h2>
            <div class="info-list">
              <div class="info-item"><span>🎭 Avatar</span><strong>{{ authStore.user.avatar }}</strong></div>
              <div class="info-item"><span>🎂 Date de naissance</span><strong>{{ formatDate(authStore.user.birthdate) }}</strong></div>
              <div class="info-item"><span>✅ Âge vérifié</span><strong>{{ authStore.user.age_verified ? 'Oui' : 'Non' }}</strong></div>
              <div class="info-item" v-if="authStore.user.discord_username"><span>💬 Discord</span><strong>{{ authStore.user.discord_username }}</strong></div>
            </div>
          </div>

          <div class="voxicraft-panel">
            <h2 class="card-title">📝 À propos</h2>
            <div class="bio-content">{{ authStore.user.bio || 'Aucune bio renseignée. Parlez-nous de vous !' }}</div>
          </div>

          <div class="voxicraft-panel">
            <h2 class="card-title">⚡ Actions rapides</h2>
            <div class="actions-grid">
              <router-link to="/profile/avatar-builder" class="btn-avatar primary-avatar-action"><span class="btn-icon">🎨</span>Créer mon avatar</router-link>
              <router-link to="/profile/avatar-builder" class="btn-avatar secondary-avatar-action"><span class="btn-icon">🧩</span>Modifier mon avatar</router-link>
              <router-link to="/servers" class="action-btn">🖥️<span>Mes serveurs</span></router-link>
              <button @click="showEditForm = true" class="action-btn" v-if="!showEditForm">✏️<span style="line-height: 25px;">Modifier profil</span></button>
            </div>
          </div>
        </div>

        <div class="right-column">
          <div v-if="showEditForm" class="voxicraft-panel">
            <h2 class="card-title">✏️ Modifier mon profil</h2>
            <form @submit.prevent="handleUpdate" class="edit-form">
              <label class="form-label">Pseudo</label>
              <input v-model="editData.username" type="text" class="form-input" minlength="3" maxlength="20" />

              <label class="form-label">Avatar rapide (genre)</label>
              <div class="avatar-selector">
                <label class="avatar-option" :class="{ selected: editData.avatar === 'steve' }">
                  <input type="radio" v-model="editData.avatar" value="steve" />
                  <AvatarHeadImage avatar="steve" class="avatar-preview-img" />
                  <span>Steve</span>
                </label>
                <label class="avatar-option" :class="{ selected: editData.avatar === 'alex' }">
                  <input type="radio" v-model="editData.avatar" value="alex" />
                  <AvatarHeadImage avatar="alex" class="avatar-preview-img" />
                  <span>Alex</span>
                </label>
              </div>

              <template v-if="customAvatars.length > 0">
                <label class="form-label">Avatars personnalisés</label>
                <div class="custom-avatar-selector">
                  <button
                    v-for="avatar in customAvatars"
                    :key="avatar.id"
                    type="button"
                    class="custom-avatar-option"
                    :class="{ selected: avatar.is_active }"
                    :disabled="selectingCustomAvatarId === avatar.id"
                    @click="selectCustomAvatar(avatar.id)"
                  >
                    <span class="custom-avatar-name">{{ avatar.name }}</span>
                    <span class="custom-avatar-kind">{{ avatar.base_kind }}</span>
                    <span v-if="avatar.is_active" class="custom-avatar-active">Actif</span>
                  </button>
                </div>
              </template>

              <router-link to="/profile/avatar-builder" class="inline-avatar-builder-link">Ouvrir le builder graphique d'avatar</router-link>

              <label class="form-label">Bio</label>
              <textarea v-model="editData.bio" class="form-input form-textarea" rows="4" maxlength="500"></textarea>

              <div class="form-actions">
                <button type="submit" class="btn-submit" :disabled="authStore.loading">Enregistrer</button>
                <button type="button" @click="showEditForm = false" class="btn-cancel">Annuler</button>
              </div>
            </form>
          </div>

          <div v-else class="voxicraft-panel">
            <h2 class="card-title">📊 Statistiques</h2>
            <div class="stats-grid">
              <div class="stat-item"><strong>{{ serverCount }}</strong><span>Serveurs</span></div>
              <div class="stat-item"><strong>{{ daysSinceJoined }}</strong><span>Jours</span></div>
            </div>
          </div>

          <div v-if="!showEditForm" class="voxicraft-panel security-panel">
            <h2 class="card-title">🔒 Sécurité et confidentialité</h2>
            <div class="bio-content security-value">JWT Token actif</div>
            <div class="bio-content security-value">Email : <span class="security-email">{{ authStore.user.email }}</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import AvatarHeadImage from '@/components/AvatarHeadImage.vue'
import { avatarApi, type UserAvatar } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { useServerStore } from '@/stores/server'

const authStore = useAuthStore()
const serverStore = useServerStore()
const showEditForm = ref(false)
const editData = ref({ username: '', avatar: 'steve', bio: '' })
const customAvatars = ref<UserAvatar[]>([])
const selectingCustomAvatarId = ref('')

const serverCount = computed(() => serverStore.servers.length)
const daysSinceJoined = computed(() => {
  if (!authStore.user?.created_at) return 0
  return Math.floor((Date.now() - new Date(authStore.user.created_at).getTime()) / 86400000)
})

const formatDate = (dateString: string | undefined) => {
  if (!dateString) return 'N/A'
  return new Date(dateString).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })
}

onMounted(async () => {
  if (!authStore.user) await authStore.fetchProfile()
  if (authStore.user) editData.value = { username: authStore.user.username, avatar: authStore.user.avatar, bio: authStore.user.bio || '' }
  if (serverStore.servers.length === 0) await serverStore.fetchUserServers()
  await loadCustomAvatars()
})

const loadCustomAvatars = async () => {
  customAvatars.value = await avatarApi.list().then((response) => response.data).catch(() => [])
}

const selectCustomAvatar = async (avatarId: string) => {
  selectingCustomAvatarId.value = avatarId
  try {
    await avatarApi.select(avatarId)
    customAvatars.value = customAvatars.value.map((avatar) => ({ ...avatar, is_active: avatar.id === avatarId }))
  } finally {
    selectingCustomAvatarId.value = ''
  }
}

const handleUpdate = async () => {
  const success = await authStore.updateProfile(editData.value)
  if (success) showEditForm.value = false
}
</script>

<style scoped>
.profile { min-height: calc(100vh - 80px); padding: 2rem 1rem; max-width: 100%; overflow-x: hidden; box-sizing: border-box; }
.profile :deep(*) { box-sizing: border-box; }
.profile-header, .profile-content { max-width: 1200px; width: 100%; margin: 0 auto; min-width: 0; }
.profile-header { display: grid; grid-template-columns: auto minmax(0, 1fr) auto; gap: 2rem; align-items: center; margin-bottom: 3rem; }
.voxicraft-panel { background: rgba(139, 69, 19, 0.9); border: 4px solid #5d4037; border-radius: 12px; padding: 2rem; box-shadow: 8px 8px 0 rgba(0, 0, 0, 0.5); min-width: 0; width: 100%; max-width: 100%; overflow: hidden; }
.avatar-display { position: relative; }
.avatar-frame { width: 120px; height: 120px; background: linear-gradient(135deg, #64ffda, #4caf50); border: 4px solid #1a1a1a; border-radius: 15px; display: flex; align-items: center; justify-content: center; box-shadow: 0 8px 20px rgba(0, 0, 0, 0.4); overflow: hidden; padding: 5px; }
.user-badge { position: absolute; bottom: -10px; right: -10px; background: linear-gradient(135deg, #4caf50, #66bb6a); border: 3px solid #1a1a1a; border-radius: 50px; padding: 0.3rem 0.7rem; font-size: 0.6rem; color: #fff; font-weight: bold; }
.header-info { min-width: 0; }
.username { font-size: 2rem; color: #64ffda; margin-bottom: 0.5rem; text-shadow: 3px 3px 0 rgba(0, 0, 0, 0.5); overflow-wrap: anywhere; }
.user-email, .meta-item, .bio-content, .stat-item span { color: rgba(255, 255, 255, 0.75); }
.user-email { max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.header-actions, .left-column, .right-column, .info-list, .edit-form { display: flex; flex-direction: column; gap: 1rem; min-width: 0; max-width: 100%; }
.profile-content { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); gap: 2rem; }
.card-title { color: #64ffda; margin-bottom: 1.5rem; max-width: 100%; overflow-wrap: anywhere; }
.info-item, .bio-content, .stat-item { padding: 1rem; background: rgba(0, 0, 0, 0.3); border: 2px solid rgba(100, 255, 218, 0.2); border-radius: 8px; min-width: 0; max-width: 100%; }
.info-item { display: flex; justify-content: space-between; gap: 1rem; color: #fff; }
.info-item span, .info-item strong { min-width: 0; overflow-wrap: anywhere; }
.actions-grid, .stats-grid, .avatar-selector { display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 1rem; min-width: 0; }
.actions-grid { display: flex; flex-wrap: wrap; }
.bio-content { width: 100%; overflow: hidden; overflow-wrap: anywhere; }
.security-panel { width: 100%; min-width: 0; overflow: hidden; }
.security-value { display: block; white-space: nowrap; text-overflow: ellipsis; overflow: hidden; }
.security-email { display: inline; min-width: 0; }
.custom-avatar-selector { display: grid; gap: .75rem; min-width: 0; }
.custom-avatar-option { display: grid; grid-template-columns: minmax(0, 1fr) auto auto; align-items: center; gap: .5rem; width: 100%; padding: .75rem; border-radius: 8px; border: 2px solid rgba(100, 255, 218, 0.2); background: rgba(0, 0, 0, 0.35); color: #fff; cursor: pointer; text-align: left; }
.custom-avatar-option.selected { border-color: #64ffda; background: rgba(100, 255, 218, 0.18); }
.custom-avatar-name { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.custom-avatar-kind, .custom-avatar-active { font-size: .65rem; color: #64ffda; }
.btn-edit, .btn-avatar, .btn-submit, .btn-cancel, .action-btn { display: flex; align-items: center; justify-content: center; gap: 0.5rem; color: #fff; font-family: 'Press Start 2P', cursive; cursor: pointer; border-radius: 5px; text-decoration: none; transition: all 0.3s ease; padding: 0.9rem 1rem; border: 3px solid rgba(100, 255, 218, 0.2); background: rgba(0, 0, 0, 0.3); min-width: 0; max-width: 100%; }
.action-btn { flex-direction: column; min-height: 96px; padding-inline: 1rem; width: max-content; max-width: 200px; }
.primary-avatar-action { background: linear-gradient(135deg, #8e24aa, #ba68c8); border-color: #6a1b9a; }
.secondary-avatar-action { background: linear-gradient(135deg, #ff8f00, #ffb300); border-color: #ef6c00; color: #1a1a1a; }
.form-label { color: #ffd700; font-size: 0.75rem; }
.form-input { background: rgba(0, 0, 0, 0.6); border: 3px solid #424242; color: white; padding: 0.75rem; font-family: 'Courier New', monospace; font-size: 0.9rem; width: 100%; outline: none; border-radius: 5px; }
.form-textarea { resize: vertical; min-height: 100px; }
.avatar-option { cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 0.5rem; padding: 1rem; background: rgba(0, 0, 0, 0.4); border: 3px solid rgba(100, 255, 218, 0.2); border-radius: 10px; color: #fff; min-width: 0; }
.avatar-option input { display: none; }
.avatar-option.selected { border-color: #64ffda; background: rgba(100, 255, 218, 0.2); }
.avatar-preview-img { width: 80px; height: 80px; border-radius: 8px; }
.inline-avatar-builder-link { color: #64ffda; font-size: 0.75rem; text-decoration: underline; overflow-wrap: anywhere; }
.form-actions { display: flex; gap: 1rem; min-width: 0; }
.btn-submit { background: linear-gradient(135deg, #4caf50, #66bb6a); border-color: #2e7d32; flex: 1; }
.btn-cancel { background: rgba(255, 107, 107, 0.2); border-color: #ff6b6b; flex: 1; }
.stat-item { display: flex; flex-direction: column; gap: .5rem; text-align: center; }
.stat-item strong { color: #64ffda; font-size: 1.5rem; }
@media (max-width: 900px) { .profile { padding: 1rem .75rem; } .profile-header, .profile-content { grid-template-columns: 1fr; } .profile-header { gap: 1.25rem; } .voxicraft-panel { padding: 1.25rem; box-shadow: 6px 6px 0 rgba(0, 0, 0, 0.5); } .actions-grid { display: grid; grid-template-columns: minmax(0, 1fr); } .action-btn { width: 100%; max-width: 100%; } }
@media (max-width: 520px) { .profile { padding: .75rem .5rem; } .voxicraft-panel { padding: 1rem; } .username { font-size: 1.35rem; } .card-title { font-size: 1.1rem; } .info-item { flex-direction: column; align-items: flex-start; } .form-actions, .custom-avatar-option { grid-template-columns: 1fr; } .form-actions { flex-direction: column; } }
</style>