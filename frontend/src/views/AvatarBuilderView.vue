<template>
  <div class="avatar-builder voxicraft-bg">
    <div class="voxicraft-container">
      <div class="builder-header">
        <button @click="goBack" class="voxicraft-button back-button">← Retour au profil</button>
        <div>
          <h1 class="voxicraft-title">🎭 Builder d'avatar</h1>
          <p class="voxicraft-text subtitle">Personnage généré avec le character-builder du repo Minecraft WebXR.</p>
        </div>
      </div>

      <div class="builder-layout">
        <section class="viewport-card voxicraft-panel">
          <canvas ref="canvasRef" class="avatar-canvas" />
        </section>
        <aside class="tools-card voxicraft-panel">
          <h2>Configuration</h2>
          <div class="status-list">
            <div class="status-item">✅ Babylon.js</div>
            <div class="status-item">✅ buildCharacter()</div>
            <div class="status-item">✅ matrices Steve/Alex</div>
            <div class="status-item">✅ zoom canvas isolé</div>
            <div class="status-item">✅ aucun sol</div>
          </div>
        </aside>
      </div>

      <section class="custom-avatar-svg-card voxicraft-panel">
        <div class="svg-card-header">
          <div>
            <h2>Avatar custom connecté en SVG</h2>
            <p class="voxicraft-text">Conversion du mesh courant en SVG.</p>
          </div>
          <span :class="['custom-status', hasCustomAvatar ? 'enabled' : 'disabled']">
            {{ hasCustomAvatar ? 'Custom détecté' : 'Aucun custom' }}
          </span>
        </div>
        <div v-if="hasCustomAvatar && connectedAvatarSvg" class="svg-preview-box" v-html="connectedAvatarSvg"></div>
        <div v-else class="svg-empty-state">Aucun avatar custom à convertir pour l'utilisateur connecté.</div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import '@babylonjs/loaders'
import { ArcRotateCamera, Color4, Engine, HemisphericLight, Mesh, Scene, Vector3 } from '@babylonjs/core'
import { useAuthStore } from '@/stores/auth'
import { buildCharacter } from '@/character-builder/character-builder'
import { generateCharacterPerspectiveSvg } from '@/character-builder/svg-export'
import { alexModelTextures } from '@/character-builder/alex-color-matrices'
import { steveModelTextures } from '@/character-builder/steve-color-matrices'

const router = useRouter()
const authStore = useAuthStore()
const canvasRef = ref<HTMLCanvasElement | null>(null)
const connectedAvatarSvg = ref('')

let engine: Engine | null = null
let scene: Scene | null = null
let avatarRoot: Mesh | null = null

const avatarName = computed(() => authStore.user?.avatar?.trim() || 'alex')
const isSteveAvatar = computed(() => avatarName.value === 'steve')
const hasCustomAvatar = computed(() => !!avatarName.value && avatarName.value !== 'steve' && avatarName.value !== 'alex')

const goBack = () => router.push({ name: 'profile' })

const preventCanvasPageScroll = (event: WheelEvent) => {
  event.preventDefault()
}

const initializeBabylon = () => {
  const canvas = canvasRef.value
  if (!canvas) return

  engine = new Engine(canvas, true, { preserveDrawingBuffer: true, stencil: true, antialias: true })
  scene = new Scene(engine)
  scene.clearColor = new Color4(0.03, 0.04, 0.08, 1)

  const camera = new ArcRotateCamera('avatar-builder-camera', Math.PI / 2, Math.PI / 2.35, 4.2, new Vector3(0, 1, 0), scene)
  camera.attachControl(canvas, true)
  camera.lowerRadiusLimit = 2.4
  camera.upperRadiusLimit = 7
  camera.wheelDeltaPercentage = 0.01

  const light = new HemisphericLight('avatar-builder-light', new Vector3(0, 1, 0), scene)
  light.intensity = 0.95

  avatarRoot = buildCharacter(scene, model(), Vector3.Zero(), { physics: false })
  refreshSvg()

  canvas.addEventListener('wheel', preventCanvasPageScroll, { passive: false })
  engine.runRenderLoop(() => scene?.render())
  window.addEventListener('resize', resizeEngine)
}

const refreshSvg = () => {
  connectedAvatarSvg.value = hasCustomAvatar.value && avatarRoot
    ? generateCharacterPerspectiveSvg(avatarRoot, { width: 360, height: 360, padding: 18, background: 'rgba(3, 4, 8, 1)' })
    : ''
}

const model = () => {
  const textures = isSteveAvatar.value ? steveModelTextures : alexModelTextures
  const armWidth = isSteveAvatar.value ? 0.25 : 0.1875
  const armOffset = isSteveAvatar.value ? 0.375 : 0.34375

  return {
    name: 'connectedAvatar',
    bodyType: isSteveAvatar.value ? 'masculine' : 'custom',
    bodyParts: [
      { name: 'head', dimensions: { width: 0.5, height: 0.5, depth: 0.5 }, position: { x: 0, y: 1.625, z: 0 }, textures: textures.head },
      { name: 'torso', dimensions: { width: 0.5, height: 0.75, depth: 0.25 }, position: { x: 0, y: 1, z: 0 }, textures: textures.torso },
      { name: 'rightArm', dimensions: { width: armWidth, height: 0.75, depth: 0.25 }, position: { x: -armOffset, y: 1, z: 0 }, pivot: { x: 0, y: 0.375, z: 0 }, textures: textures.rightArm },
      { name: 'leftArm', dimensions: { width: armWidth, height: 0.75, depth: 0.25 }, position: { x: armOffset, y: 1, z: 0 }, pivot: { x: 0, y: 0.375, z: 0 }, textures: textures.leftArm },
      { name: 'rightLeg', dimensions: { width: 0.25, height: 0.75, depth: 0.25 }, position: { x: -0.125, y: 0.25, z: 0 }, pivot: { x: 0, y: 0.375, z: 0 }, textures: textures.rightLeg },
      { name: 'leftLeg', dimensions: { width: 0.25, height: 0.75, depth: 0.25 }, position: { x: 0.125, y: 0.25, z: 0 }, pivot: { x: 0, y: 0.375, z: 0 }, textures: textures.leftLeg },
    ],
  }
}

const resizeEngine = () => engine?.resize()

onMounted(async () => {
  if (!authStore.user) await authStore.fetchProfile()
  initializeBabylon()
})

onBeforeUnmount(() => {
  canvasRef.value?.removeEventListener('wheel', preventCanvasPageScroll)
  window.removeEventListener('resize', resizeEngine)
  scene?.dispose()
  engine?.dispose()
})
</script>

<style scoped>
.avatar-builder { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.voxicraft-container { max-width: 1400px; margin: 0 auto; }
.builder-header { display: flex; align-items: center; gap: 1.5rem; margin-bottom: 2rem; }
.back-button { flex-shrink: 0; background-color: #424242; border-color: #212121; }
.subtitle { margin-top: 0.5rem; opacity: 0.85; }
.builder-layout { display: grid; grid-template-columns: minmax(0, 1fr) 360px; gap: 2rem; }
.viewport-card { min-height: 680px; padding: 0; overflow: hidden; }
.avatar-canvas { display: block; width: 100%; height: 680px; outline: none; touch-action: none; }
.tools-card, .custom-avatar-svg-card { padding: 1.5rem; }
.tools-card h2, .custom-avatar-svg-card h2 { margin-bottom: 1rem; color: #64ffda; }
.status-list { display: grid; gap: 0.75rem; margin-top: 1.5rem; }
.status-item { padding: 0.75rem; border: 1px solid rgba(100, 255, 218, 0.25); border-radius: 8px; background: rgba(0, 0, 0, 0.2); }
.custom-avatar-svg-card { margin-top: 2rem; }
.svg-card-header { display: flex; justify-content: space-between; gap: 1rem; align-items: flex-start; margin-bottom: 1.5rem; }
.custom-status { flex-shrink: 0; border-radius: 999px; padding: 0.45rem 0.75rem; font-size: 0.75rem; font-weight: bold; }
.custom-status.enabled { background: rgba(100, 255, 218, 0.16); color: #64ffda; border: 1px solid rgba(100, 255, 218, 0.45); }
.custom-status.disabled { background: rgba(255, 255, 255, 0.08); color: rgba(255, 255, 255, 0.7); border: 1px solid rgba(255, 255, 255, 0.15); }
.svg-preview-box { display: flex; justify-content: center; align-items: center; min-height: 420px; border: 3px solid rgba(100, 255, 218, 0.35); border-radius: 12px; background: rgba(0, 0, 0, 0.35); overflow: hidden; }
.svg-preview-box :deep(svg) { width: min(360px, 100%); height: auto; display: block; }
.svg-empty-state { padding: 2rem; border: 2px dashed rgba(255, 255, 255, 0.25); border-radius: 12px; color: rgba(255, 255, 255, 0.75); background: rgba(0, 0, 0, 0.25); line-height: 1.7; }
@media (max-width: 1024px) { .builder-layout { grid-template-columns: 1fr; } .viewport-card, .avatar-canvas { min-height: 520px; height: 520px; } }
@media (max-width: 768px) { .builder-header, .svg-card-header { align-items: flex-start; flex-direction: column; } .avatar-builder { padding: 1rem; } }
</style>
