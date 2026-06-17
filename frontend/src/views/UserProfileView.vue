<template>
  <div class="user-profile-page voxicraft-bg">
    <div class="voxicraft-container profile-layout">
      <section class="voxicraft-panel profile-info-card">
        <div v-if="loading" class="state-message">⏳ Chargement du profil...</div>
        <div v-else-if="error" class="error-message">{{ error }}</div>
        <template v-else-if="user">
          <div v-if="!isOwnProfile" class="profile-relation-corner">
            <span v-if="relationStatus === 'friend'" class="friend-badge" title="Ami">✓ Ami</span>
            <button
              v-else-if="relationStatus === 'none'"
              class="profile-icon-button add"
              type="button"
              title="Demander en ami"
              aria-label="Demander en ami"
              :disabled="relationLoading"
              @click="sendFriendRequest"
            >🤝</button>
            <span v-else-if="relationStatus === 'outgoing'" class="relation-state-badge" title="Demande envoyée">⏳</span>
            <button
              v-else-if="relationStatus === 'incoming'"
              class="profile-icon-button accept"
              type="button"
              title="Accepter la demande"
              aria-label="Accepter la demande"
              :disabled="relationLoading"
              @click="acceptIncomingRequest"
            >✓</button>
          </div>

          <div class="identity-row">
            <img :src="profileImageSrc" :alt="`Avatar de ${user.username}`" class="profile-picture" />
            <div class="identity-copy">
              <h1 class="voxicraft-title username-title">{{ user.username }}</h1>
              <p class="meta-line">{{ genderLabel }} · {{ formattedBirthdate }} <span v-if="age !== null">[{{ age }} ans]</span></p>
            </div>
          </div>

          <div v-if="user.bio" class="bio-block">
            <h2>Bio</h2>
            <p>{{ user.bio }}</p>
          </div>
          <div v-else class="bio-block empty-bio">
            <h2>Bio</h2>
            <p>Aucune bio renseignée.</p>
          </div>
        </template>
      </section>

      <section class="voxicraft-panel avatar-viewer-card">
        <div class="viewer-header">
          <div>
            <p class="viewer-eyebrow">Avatar 3D</p>
            <h2>{{ user?.username || 'Utilisateur' }}</h2>
          </div>
          <span class="viewer-badge">Marche lente</span>
        </div>
        <canvas ref="canvasRef" class="avatar-canvas" />
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { ArcRotateCamera, Color4, Engine, HemisphericLight, Mesh, Scene, Vector3 } from '@babylonjs/core'
import request, { type ActiveAvatarResponse, type User } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { useFriendsStore } from '@/stores/friends'
import { buildCharacter, getAllBodyParts } from '@/character-builder/character-builder'
import { createCharacterModelFromAvatar, createEditableAvatar, createEditableAvatarFromApi, type EditableAvatar } from '@/character-builder/avatar-editor'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const route = useRoute()
const authStore = useAuthStore()
const friendsStore = useFriendsStore()
const user = ref<User | null>(null)
const activeAvatar = ref<ActiveAvatarResponse | null>(null)
const loading = ref(true)
const relationLoading = ref(false)
const error = ref('')
const canvasRef = ref<HTMLCanvasElement | null>(null)
const profileImageSrc = ref('')

let engine: Engine | null = null
let scene: Scene | null = null
let avatarRoot: Mesh | null = null
let bodyParts = new Map<string, Mesh>()
let startedAt = performance.now()

const userId = computed(() => String(route.params.id || ''))
const baseAvatarKind = computed(() => activeAvatar.value?.avatar?.base_kind || user.value?.avatar || 'alex')
const genderLabel = computed(() => baseAvatarKind.value === 'steve' ? 'Homme' : 'Femme')
const isOwnProfile = computed(() => !!user.value && authStore.user?.id === user.value.id)
const relationStatus = computed(() => {
  if (!user.value || isOwnProfile.value) return 'self'
  if (friendsStore.friendIds.has(user.value.id)) return 'friend'
  if (friendsStore.outgoingRequestReceiverIds.has(user.value.id)) return 'outgoing'
  if (friendsStore.incomingRequestRequesterIds.has(user.value.id)) return 'incoming'
  return 'none'
})
const incomingRequest = computed(() => user.value ? friendsStore.incomingRequests.find((request) => request.requester.id === user.value?.id) : undefined)
const formattedBirthdate = computed(() => {
  if (!user.value?.birthdate) return 'Date inconnue'
  const date = new Date(user.value.birthdate)
  if (Number.isNaN(date.getTime())) return user.value.birthdate
  return date.toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })
})
const age = computed(() => {
  if (!user.value?.birthdate) return null
  const birthdate = new Date(user.value.birthdate)
  if (Number.isNaN(birthdate.getTime())) return null
  const today = new Date()
  let value = today.getFullYear() - birthdate.getFullYear()
  const monthDelta = today.getMonth() - birthdate.getMonth()
  if (monthDelta < 0 || (monthDelta === 0 && today.getDate() < birthdate.getDate())) value -= 1
  return value
})

onMounted(async () => {
  await Promise.all([loadProfile(), friendsStore.fetchAll()])
  await nextTick()
  initScene()
  renderAvatar()
})

watch(userId, async () => {
  await Promise.all([loadProfile(), friendsStore.fetchAll()])
  renderAvatar()
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', resize)
  avatarRoot?.dispose()
  scene?.dispose()
  engine?.dispose()
  if (profileImageSrc.value.startsWith('blob:')) URL.revokeObjectURL(profileImageSrc.value)
})

async function loadProfile() {
  if (!userId.value) return
  loading.value = true
  error.value = ''
  try {
    const [userResponse, avatarResponse] = await Promise.all([
      request<User>(`/users/${userId.value}`),
      request<ActiveAvatarResponse>(`/users/${userId.value}/avatar`).catch(() => ({ data: null as unknown as ActiveAvatarResponse })),
    ])
    user.value = userResponse.data
    activeAvatar.value = avatarResponse.data
    await loadProfilePicture()
  } catch {
    error.value = 'Impossible de charger ce profil utilisateur.'
  } finally {
    loading.value = false
  }
}

async function loadProfilePicture() {
  if (!userId.value) return
  const token = localStorage.getItem('auth_token')
  if (!token) return
  if (profileImageSrc.value.startsWith('blob:')) URL.revokeObjectURL(profileImageSrc.value)
  const response = await fetch(`${API_BASE_URL}/users/${userId.value}/profile-pic.svg`, {
    headers: { Authorization: `Bearer ${token}` },
    credentials: 'include',
  })
  if (!response.ok) return
  profileImageSrc.value = URL.createObjectURL(await response.blob())
}

async function sendFriendRequest() {
  if (!user.value || relationLoading.value) return
  relationLoading.value = true
  await friendsStore.sendRequest(user.value.id)
  await friendsStore.fetchAll()
  relationLoading.value = false
}

async function acceptIncomingRequest() {
  if (!incomingRequest.value || relationLoading.value) return
  relationLoading.value = true
  await friendsStore.acceptRequest(incomingRequest.value.id)
  await friendsStore.fetchAll()
  relationLoading.value = false
}

function currentEditableAvatar(): EditableAvatar {
  if (activeAvatar.value?.avatar) return createEditableAvatarFromApi(activeAvatar.value.avatar)
  return createEditableAvatar(user.value?.avatar || 'alex')
}

function initScene() {
  const canvas = canvasRef.value
  if (!canvas) return
  engine = new Engine(canvas, true, { preserveDrawingBuffer: true, stencil: true, antialias: true })
  scene = new Scene(engine)
  scene.clearColor = new Color4(0.03, 0.04, 0.08, 1)

  const camera = new ArcRotateCamera('user-profile-avatar-camera', Math.PI / 2, Math.PI / 2.35, 4.4, new Vector3(0, 1, 0), scene)
  camera.attachControl(canvas, true)
  camera.lowerRadiusLimit = 3
  camera.upperRadiusLimit = 6
  camera.wheelDeltaPercentage = 0.01

  const light = new HemisphericLight('user-profile-avatar-light', new Vector3(0, 1, 0), scene)
  light.intensity = 1

  startedAt = performance.now()
  engine.runRenderLoop(() => {
    animateAvatar()
    scene?.render()
  })
  window.addEventListener('resize', resize)
}

function renderAvatar() {
  if (!scene || !user.value) return
  avatarRoot?.dispose()
  avatarRoot = buildCharacter(scene, createCharacterModelFromAvatar(currentEditableAvatar()), Vector3.Zero(), { physics: false })
  bodyParts = getAllBodyParts(avatarRoot)
}

function animateAvatar() {
  if (!avatarRoot) return
  const t = (performance.now() - startedAt) / 1000
  avatarRoot.rotation.y = t * 0.35
  const walk = Math.sin(t * 2.2) * 0.42
  const counterWalk = Math.sin(t * 2.2 + Math.PI) * 0.42

  const rightArm = bodyParts.get('rightArm')
  const leftArm = bodyParts.get('leftArm')
  const rightLeg = bodyParts.get('rightLeg')
  const leftLeg = bodyParts.get('leftLeg')

  if (rightArm) rightArm.rotation.x = walk
  if (leftArm) leftArm.rotation.x = counterWalk
  if (rightLeg) rightLeg.rotation.x = counterWalk
  if (leftLeg) leftLeg.rotation.x = walk
}

function resize() { engine?.resize() }
</script>

<style scoped>
.user-profile-page { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.profile-layout { max-width: 1180px; margin: 0 auto; display: grid; grid-template-columns: minmax(300px, .85fr) minmax(0, 1.15fr); gap: 1.5rem; align-items: stretch; }
.profile-info-card, .avatar-viewer-card { padding: 1.5rem; min-width: 0; }
.profile-info-card { position: relative; }
.profile-relation-corner { position: absolute; top: 1rem; right: 1rem; z-index: 2; display: flex; align-items: center; justify-content: flex-end; }
.friend-badge { display: inline-flex; align-items: center; justify-content: center; min-height: 32px; padding: .35rem .7rem; border-radius: 999px; background: rgba(46,125,50,.35); border: 2px solid #64ffda; color: #a5d6a7; font-family: monospace; font-weight: 900; box-shadow: 3px 3px 0 rgba(0,0,0,.45); }
.relation-state-badge { width: 38px; height: 34px; display: inline-flex; align-items: center; justify-content: center; background: rgba(255,179,0,.24); border: 2px solid #ffb300; color: #ffe082; box-shadow: 3px 3px 0 rgba(0,0,0,.45); }
.profile-icon-button { width: 42px; height: 38px; display: inline-flex; align-items: center; justify-content: center; border: 3px solid #3e2723; color: #fff; box-shadow: 3px 3px 0 rgba(0,0,0,.45); cursor: pointer; font-size: 1.05rem; }
.profile-icon-button.add { background: #2e7d32; border-color: #1b5e20; }
.profile-icon-button.accept { background: #2e7d32; border-color: #1b5e20; }
.profile-icon-button:disabled { opacity: .55; cursor: not-allowed; }
.identity-row { display: grid; grid-template-columns: 96px minmax(0, 1fr); gap: 1rem; align-items: start; padding-right: 5rem; }
.profile-picture { width: 96px; height: 96px; object-fit: contain; background: rgba(0,0,0,.35); border: 4px solid #4a4a4a; border-radius: 12px; image-rendering: pixelated; }
.identity-copy { min-width: 0; text-align: left; }
.username-title { margin: 0 0 .4rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.meta-line { margin: 0; color: #d7ccc8; line-height: 1.6; }
.bio-block { margin-top: 2rem; padding-top: 1.25rem; border-top: 2px solid rgba(255,255,255,.14); }
.bio-block h2 { margin: 0 0 .75rem; color: #64ffda; }
.bio-block p { margin: 0; color: #fff; line-height: 1.8; white-space: pre-wrap; overflow-wrap: anywhere; }
.empty-bio p { color: #d7ccc8; font-style: italic; }
.viewer-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem; margin-bottom: 1rem; }
.viewer-eyebrow { margin: 0 0 .25rem; color: #d7ccc8; font-family: monospace; text-transform: uppercase; letter-spacing: .08em; font-size: .8rem; }
.viewer-header h2 { margin: 0; color: #64ffda; }
.viewer-badge { background: rgba(46,125,50,.35); border: 2px solid #2e7d32; color: #a5d6a7; border-radius: 999px; padding: .3rem .55rem; font-family: monospace; font-size: .8rem; white-space: nowrap; }
.avatar-canvas { width: 100%; height: 520px; display: block; border: 4px solid #3e2723; border-radius: 12px; background: #080b13; touch-action: none; }
.state-message, .error-message { padding: 1rem; text-align: center; color: #d7ccc8; }
.error-message { color: #ff6b6b; border: 2px solid #ff6b6b; background: rgba(255,107,107,.12); border-radius: 8px; }
@media (max-width: 900px) { .profile-layout { grid-template-columns: 1fr; } .avatar-canvas { height: 420px; } }
@media (max-width: 560px) { .user-profile-page { padding: 1rem .55rem; } .profile-info-card, .avatar-viewer-card { padding: .85rem; } .profile-relation-corner { top: .7rem; right: .7rem; } .identity-row { grid-template-columns: 72px minmax(0, 1fr); gap: .75rem; padding-right: 3.25rem; } .profile-picture { width: 72px; height: 72px; } .avatar-canvas { height: 360px; } .viewer-header { flex-direction: column; } }
</style>
