<template>
  <div class="profile voxicraft-bg">
    <div class="voxicraft-container">
      <div class="profile-header voxicraft-panel">
        <div class="avatar-display">
          <div class="avatar-frame">
            <AvatarHeadImage :avatar="authStore.user?.avatar" :custom-avatar="activeProfileAvatar || undefined" />
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
              <div class="info-item avatar-info-item">
                <span>🎭 Avatar</span>
                <div class="profile-avatar-card">
                  <AvatarSvgPreview :avatar="authStore.user.avatar" :custom-avatar="activeProfileAvatar || undefined" pose="walking" class="profile-avatar-preview" />
                  <strong>{{ activeProfileAvatar?.name || authStore.user.avatar }}</strong>
                </div>
              </div>
              <div class="info-item"><span>🎂 Date de naissance</span><strong>{{ formatDate(authStore.user.birthdate) }}</strong></div>
              <div class="info-item"><span>✅ Âge vérifié</span><strong>{{ authStore.user.age_verified ? 'Oui' : 'Non' }}</strong></div>
              <div class="info-item discord-info-item">
                <span>💬 Discord</span>
                <div v-if="authStore.user.discord_username" class="discord-linked-card">
                  <div class="discord-linked-content">
                    <strong>Discord lié</strong>
                    <small>{{ authStore.user.discord_username }}</small>
                  </div>
                  <button
                    type="button"
                    class="discord-unlink-button"
                    :disabled="unlinkingDiscord"
                    title="Supprimer le lien Discord"
                    aria-label="Supprimer le lien Discord"
                    @click="unlinkDiscord"
                  >
                    <span class="discord-unlink-icon" aria-hidden="true">🗑️</span>
                    <span class="sr-only">{{ unlinkingDiscord ? 'Suppression...' : 'Supprimer' }}</span>
                  </button>
                </div>
                <button v-else type="button" class="discord-link-button" :disabled="linkingDiscord" @click="linkDiscord">
                  {{ linkingDiscord ? 'Redirection...' : 'Lier mon Discord' }}
                </button>
              </div>
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
                <label class="avatar-option" :class="{ selected: editData.avatar === 'steve' && !selectedCustomAvatarId }">
                  <input type="radio" v-model="editData.avatar" value="steve" @change="selectedCustomAvatarId = ''" />
                  <AvatarHeadImage avatar="steve" class="avatar-preview-img" />
                  <span>Steve</span>
                </label>
                <label class="avatar-option" :class="{ selected: editData.avatar === 'alex' && !selectedCustomAvatarId }">
                  <input type="radio" v-model="editData.avatar" value="alex" @change="selectedCustomAvatarId = ''" />
                  <AvatarHeadImage avatar="alex" class="avatar-preview-img" />
                  <span>Alex</span>
                </label>
              </div>

              <template v-if="customAvatars.length > 0">
                <div class="custom-avatar-heading">
                  <label class="form-label">Avatars personnalisés</label>
                  <button v-if="selectedCustomAvatarId" type="button" class="custom-avatar-clear" @click="clearPendingCustomAvatar">Annuler la sélection</button>
                </div>
                <div class="custom-avatar-scroll" aria-label="Avatars personnalisés">
                  <button
                    v-for="avatar in customAvatars"
                    :key="avatar.id"
                    type="button"
                    class="custom-avatar-card"
                    :class="{ selected: selectedCustomAvatarId === avatar.id, active: avatar.is_active }"
                    @click="selectPendingCustomAvatar(avatar.id)"
                  >
                    <AvatarSvgPreview :custom-avatar="avatar" class="custom-avatar-preview" />
                    <span class="custom-avatar-name">{{ avatar.name }}</span>
                    <span class="custom-avatar-state">{{ selectedCustomAvatarId === avatar.id ? 'Sélectionné' : avatar.is_active ? 'Actif' : 'Disponible' }}</span>
                  </button>
                </div>
              </template>

              <router-link to="/profile/avatar-builder" class="inline-avatar-builder-link">Ouvrir le builder graphique d'avatar</router-link>

              <label class="form-label">Bio</label>
              <textarea v-model="editData.bio" class="form-input form-textarea" rows="4" maxlength="500"></textarea>

              <div class="form-actions">
                <button type="submit" class="btn-submit" :disabled="authStore.loading || savingAvatarSelection">Enregistrer</button>
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
            <div class="info-list">
              <div class="info-item bio-content security-value">JWT Token actif</div>
              <div class="info-item bio-content security-value">Email : <span class="security-email">{{ authStore.user.email }}</span></div>
            </div>
            <AccountSecretForm />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import AccountSecretForm from '@/components/AccountSecretForm.vue'
import AvatarHeadImage from '@/components/AvatarHeadImage.vue'
import AvatarSvgPreview from '@/components/AvatarSvgPreview.vue'
import { avatarApi, type UserAvatar } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { useServerStore } from '@/stores/server'

const authStore = useAuthStore()
const serverStore = useServerStore()
const showEditForm = ref(false)
const editData = ref({ username: '', avatar: 'steve', bio: '' })
const customAvatars = ref<UserAvatar[]>([])
const activeProfileAvatar = ref<UserAvatar | null>(null)
const selectedCustomAvatarId = ref('')
const savingAvatarSelection = ref(false)
const linkingDiscord = ref(false)
const unlinkingDiscord = ref(false)

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
  activeProfileAvatar.value = customAvatars.value.find((avatar) => avatar.is_active) || null
  selectedCustomAvatarId.value = activeProfileAvatar.value?.id || ''
}

const linkDiscord = async () => {
  linkingDiscord.value = true
  try {
    const url = await authStore.getDiscordAuthUrl()
    if (url) window.location.href = url
  } finally {
    linkingDiscord.value = false
  }
}

const unlinkDiscord = async () => {
  unlinkingDiscord.value = true
  try {
    await authStore.unlinkDiscord()
  } finally {
    unlinkingDiscord.value = false
  }
}

const selectPendingCustomAvatar = (avatarId: string) => {
  selectedCustomAvatarId.value = avatarId
}

const clearPendingCustomAvatar = () => {
  selectedCustomAvatarId.value = ''
}

const handleUpdate = async () => {
  savingAvatarSelection.value = true
  try {
    const success = await authStore.updateProfile(editData.value)
    if (!success) return

    const activeAvatarId = customAvatars.value.find((avatar) => avatar.is_active)?.id
    if (selectedCustomAvatarId.value && selectedCustomAvatarId.value !== activeAvatarId) {
      await avatarApi.select(selectedCustomAvatarId.value)
      customAvatars.value = customAvatars.value.map((avatar) => ({ ...avatar, is_active: avatar.id === selectedCustomAvatarId.value }))
      activeProfileAvatar.value = customAvatars.value.find((avatar) => avatar.is_active) || null
    } else if (!selectedCustomAvatarId.value && activeAvatarId) {
      await avatarApi.clearActive()
      customAvatars.value = customAvatars.value.map((avatar) => ({ ...avatar, is_active: false }))
      activeProfileAvatar.value = null
    }

    showEditForm.value = false
  } finally {
    savingAvatarSelection.value = false
  }
}
</script>

<style scoped>
.profile { min-height: calc(100vh - 80px); padding: 1.2rem .75rem; max-width: 100%; overflow-x: hidden; box-sizing: border-box; font-size: .82rem; }
.profile :deep(*) { box-sizing: border-box; }
.profile-header, .profile-content { max-width: 1060px; width: 100%; margin: 0 auto; min-width: 0; }
.profile-header { display: grid; grid-template-columns: auto minmax(0, 1fr) auto; gap: 1.1rem; align-items: center; margin-bottom: 1.35rem; }
.voxicraft-panel { background: rgba(139, 69, 19, 0.9); border: 3px solid #5d4037; border-radius: 10px; padding: 1.05rem; box-shadow: 5px 5px 0 rgba(0, 0, 0, 0.42); min-width: 0; width: 100%; max-width: 100%; overflow: hidden; }
.avatar-display { position: relative; }
.avatar-frame { width: 82px; height: 82px; background: linear-gradient(135deg, #64ffda, #4caf50); border: 3px solid #1a1a1a; border-radius: 12px; display: flex; align-items: center; justify-content: center; box-shadow: 0 5px 12px rgba(0, 0, 0, 0.35); overflow: hidden; padding: 4px; }
.user-badge { position: absolute; bottom: -7px; right: -7px; background: linear-gradient(135deg, #4caf50, #66bb6a); border: 2px solid #1a1a1a; border-radius: 50px; padding: .22rem .48rem; font-size: .5rem; color: #fff; font-weight: bold; }
.header-info { min-width: 0; }
.username { font-size: 1.35rem; color: #64ffda; margin-bottom: .25rem; text-shadow: 2px 2px 0 rgba(0, 0, 0, 0.5); overflow-wrap: anywhere; }
.user-email, .meta-item, .bio-content, .stat-item span { color: rgba(255, 255, 255, 0.75); font-size: .72rem; }
.user-email { max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; margin: .15rem 0; }
.header-actions, .left-column, .right-column, .info-list, .edit-form { display: flex; flex-direction: column; gap: .65rem; min-width: 0; max-width: 100%; }
.profile-content { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); gap: 1.1rem; }
.card-title { color: #64ffda; margin-bottom: .8rem; max-width: 100%; overflow-wrap: anywhere; font-size: .92rem; line-height: 1.3; }
.info-item, .bio-content, .stat-item { padding: .65rem; background: rgba(0, 0, 0, 0.3); border: 1px solid rgba(100, 255, 218, 0.22); border-radius: 7px; min-width: 0; max-width: 100%; }
.info-item { display: flex; justify-content: space-between; gap: .75rem; color: #fff; font-size: .72rem; }
.avatar-info-item { align-items: flex-start; }
.profile-avatar-card { display: flex; flex-direction: column; align-items: center; gap: .35rem; min-width: 110px; max-width: 135px; }
.profile-avatar-preview { width: 100%; }
.profile-avatar-preview :deep(.svg-preview-box) { min-height: 145px; border-width: 2px; border-radius: 8px; }
.profile-avatar-preview :deep(.svg-image) { width: 95px; }
.profile-avatar-card strong { width: 100%; text-align: center; font-size: .66rem; overflow-wrap: anywhere; color: #fff; }
.info-item span, .info-item strong { min-width: 0; overflow-wrap: anywhere; }
.discord-info-item { align-items: center; }
.discord-linked-card { display: inline-grid; grid-template-columns: max-content 38px; align-items: stretch; width: auto; max-width: 100%; padding: 0; border: 2px solid rgba(100, 255, 218, .45); border-radius: 8px; background: rgba(88, 101, 242, .22); box-shadow: 3px 3px 0 rgba(0, 0, 0, .28); overflow: hidden; }
.discord-linked-content { display: flex; flex-direction: column; align-items: flex-end; gap: .18rem; min-width: 0; padding: .4rem .5rem .4rem .35rem; }
.discord-linked-card strong { color: #64ffda; font-size: .66rem; }
.discord-linked-card small { color: rgba(255, 255, 255, .75); font-size: .58rem; max-width: 145px; overflow-wrap: anywhere; text-align: right; }
.discord-unlink-button { width: 100%; min-height: 100%; display: flex; align-items: center; justify-content: center; border: 0; border-left: 2px solid rgba(0, 0, 0, .18); border-radius: 0; background: linear-gradient(135deg, #c62828, #ef5350); cursor: pointer; }
.discord-unlink-icon { display: block; font-size: .9rem; line-height: 1; filter: drop-shadow(1px 1px 0 rgba(0, 0, 0, .45)); }
.discord-unlink-button:hover { filter: brightness(1.08); }
.discord-unlink-button:disabled { opacity: .65; cursor: not-allowed; }
.discord-link-button { padding: .52rem .65rem; border-radius: 7px; border: 2px solid #5865f2; background: linear-gradient(135deg, #5865f2, #7289da); color: #fff; font-family: 'Press Start 2P', cursive; font-size: .52rem; line-height: 1.35; cursor: pointer; box-shadow: 3px 3px 0 rgba(0, 0, 0, .35); }
.discord-link-button:disabled { opacity: .65; cursor: not-allowed; }
.sr-only { position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0; }
.actions-grid, .stats-grid, .avatar-selector { display: grid; grid-template-columns: repeat(auto-fit, minmax(105px, 1fr)); gap: .65rem; min-width: 0; }
.actions-grid { display: flex; flex-wrap: wrap; }
.stats-grid { grid-template-columns: repeat(2, minmax(100px, 1fr)); }
.stat-item { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: .35rem; text-align: center; min-height: 88px; }
.stat-item strong { color: #64ffda; font-size: 1.15rem; line-height: 1.2; text-shadow: 2px 2px 0 rgba(0, 0, 0, .45); }
.stat-item span { font-size: .78rem; line-height: 1.25; }
.bio-content { width: 100%; overflow: hidden; overflow-wrap: anywhere; line-height: 1.45; }
.security-panel { width: 100%; min-width: 0; overflow: hidden; }
.security-value { display: block; white-space: nowrap; text-overflow: ellipsis; overflow: hidden; }
.security-email { display: inline; min-width: 0; }
.custom-avatar-heading { display: flex; justify-content: space-between; align-items: center; gap: .65rem; min-width: 0; }
.custom-avatar-clear { flex: 0 0 auto; padding: .38rem .52rem; border-radius: 6px; border: 2px solid #ff6b6b; background: rgba(255, 107, 107, 0.18); color: #fff; cursor: pointer; font-size: .55rem; }
.custom-avatar-scroll { display: flex; gap: .65rem; max-width: 100%; overflow-x: auto; overflow-y: hidden; padding: .2rem .2rem .55rem; scroll-snap-type: x proximity; }
.custom-avatar-card { flex: 0 0 112px; display: flex; flex-direction: column; gap: .35rem; align-items: center; padding: .55rem; border-radius: 9px; border: 2px solid rgba(100, 255, 218, 0.2); background: rgba(0, 0, 0, 0.35); color: #fff; cursor: pointer; scroll-snap-align: start; }
.custom-avatar-card.selected { border-color: #64ffda; background: rgba(100, 255, 218, 0.18); }
.custom-avatar-card.active:not(.selected) { border-color: rgba(255, 215, 0, .7); }
.custom-avatar-preview { width: 82px; }
.custom-avatar-preview :deep(.svg-preview-box) { min-height: 82px; border-width: 2px; border-radius: 7px; }
.custom-avatar-preview :deep(.svg-image) { width: 82px; }
.custom-avatar-name { width: 100%; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; text-align: center; font-size: .62rem; }
.custom-avatar-state { font-size: .52rem; color: #64ffda; }
.btn-edit, .btn-avatar, .btn-submit, .btn-cancel, .action-btn { display: flex; align-items: center; justify-content: center; gap: .38rem; color: #fff; font-family: 'Press Start 2P', cursive; cursor: pointer; border-radius: 5px; text-decoration: none; transition: all 0.3s ease; padding: .62rem .75rem; border: 2px solid rgba(100, 255, 218, 0.2); background: rgba(0, 0, 0, 0.3); min-width: 0; max-width: 100%; font-size: .58rem; line-height: 1.35; }
.action-btn { flex-direction: column; min-height: 70px; padding-inline: .75rem; width: max-content; max-width: 160px; }
.primary-avatar-action { background: linear-gradient(135deg, #8e24aa, #ba68c8); border-color: #6a1b9a; }
.secondary-avatar-action { background: linear-gradient(135deg, #ff8f00, #ffb300); border-color: #ef6c00; color: #1a1a1a; }
.form-label { color: #ffd700; font-size: .62rem; }
.form-input { background: rgba(0, 0, 0, 0.6); border: 2px solid #424242; color: white; padding: .55rem; font-family: 'Courier New', monospace; font-size: .78rem; width: 100%; outline: none; border-radius: 5px; }
.form-textarea { resize: vertical; min-height: 80px; }
</style>
