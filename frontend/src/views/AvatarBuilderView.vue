<template>
  <div class="avatar-builder voxicraft-bg">
    <div class="voxicraft-container">
      <div class="builder-header">
        <button @click="goBack" class="voxicraft-button back-button">
          ← Retour au profil
        </button>
        <div>
          <h1 class="voxicraft-title">🎭 Builder d'avatar</h1>
          <p class="voxicraft-text subtitle">
            Scène Babylon.js dédiée à la personnalisation graphique de l'avatar.
          </p>
        </div>
      </div>

      <div class="builder-layout">
        <section class="viewport-card voxicraft-panel">
          <canvas ref="canvasRef" class="avatar-canvas" />
        </section>

        <aside class="tools-card voxicraft-panel">
          <h2>Configuration</h2>
          <p class="voxicraft-text">
            La scène 3D est initialisée. Les contrôles de personnalisation seront branchés dans les prochaines étapes.
          </p>
          <div class="status-list">
            <div class="status-item">✅ Engine Babylon.js</div>
            <div class="status-item">✅ Scene</div>
            <div class="status-item">✅ Camera orbitale</div>
            <div class="status-item">✅ Lumières</div>
            <div class="status-item">✅ Avatar voxel</div>
            <div class="status-item">✅ Export SVG du mesh</div>
          </div>
        </aside>
      </div>

      <section class="custom-avatar-svg-card voxicraft-panel">
        <div class="svg-card-header">
          <div>
            <h2>Avatar custom connecté en SVG</h2>
            <p class="voxicraft-text">
              Conversion du mesh de personnage courant en SVG via le même pipeline que celui du repo Minecraft WebXR.
            </p>
          </div>
          <span :class="['custom-status', hasCustomAvatar ? 'enabled' : 'disabled']">
            {{ hasCustomAvatar ? 'Custom détecté' : 'Aucun custom' }}
          </span>
        </div>

        <div v-if="hasCustomAvatar && connectedAvatarSvg" class="svg-preview-box" v-html="connectedAvatarSvg"></div>
        <div v-else class="svg-empty-state">
          Aucun avatar custom à convertir pour l'utilisateur connecté.
          Les avatars standards Steve/Alex restent affichés avec leurs images classiques.
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import '@babylonjs/loaders'
import {
  ArcRotateCamera,
  Color3,
  Color4,
  DynamicTexture,
  Engine,
  HemisphericLight,
  Mesh,
  MeshBuilder,
  Scene,
  StandardMaterial,
  TransformNode,
  Vector3,
} from '@babylonjs/core'
import { useAuthStore } from '@/stores/auth'
import { generateCharacterPerspectiveSvg } from '@/avatar-builder/svg-export'

const router = useRouter()
const authStore = useAuthStore()
const canvasRef = ref<HTMLCanvasElement | null>(null)
const connectedAvatarSvg = ref('')

let engine: Engine | null = null
let scene: Scene | null = null
let avatarRoot: Mesh | null = null

const hasCustomAvatar = computed(() => {
  const avatar = authStore.user?.avatar?.trim()
  return !!avatar && avatar !== 'steve' && avatar !== 'alex'
})

const goBack = () => {
  router.push({ name: 'profile' })
}

const initializeBabylon = () => {
  const canvas = canvasRef.value
  if (!canvas) {
    return
  }

  engine = new Engine(canvas, true, {
    preserveDrawingBuffer: true,
    stencil: true,
    antialias: true,
  })

  scene = new Scene(engine)
  scene.clearColor = new Color4(0.03, 0.04, 0.08, 1)

  const camera = new ArcRotateCamera(
    'avatar-builder-camera',
    Math.PI / 2,
    Math.PI / 2.35,
    6,
    new Vector3(0, 1.2, 0),
    scene,
  )
  camera.attachControl(canvas, true)
  camera.lowerRadiusLimit = 3
  camera.upperRadiusLimit = 9
  camera.wheelDeltaPercentage = 0.01

  const light = new HemisphericLight('avatar-builder-light', new Vector3(0, 1, 0), scene)
  light.intensity = 0.95

  avatarRoot = createVoxelAvatarPlaceholder(scene)
  refreshConnectedAvatarSvg()

  engine.runRenderLoop(() => {
    scene?.render()
  })

  window.addEventListener('resize', resizeEngine)
}

const createVoxelAvatarPlaceholder = (targetScene: Scene): Mesh => {
  const root = new Mesh('connected-avatar-root', targetScene)
  const avatarSeed = authStore.user?.avatar || 'custom'
  const palette = buildAvatarPalette(avatarSeed)

  const skinMaterial = createMatrixMaterial(targetScene, 'skin', palette.skin)
  const hairMaterial = createMatrixMaterial(targetScene, 'hair', palette.hair)
  const shirtMaterial = createMatrixMaterial(targetScene, 'shirt', palette.shirt)
  const pantsMaterial = createMatrixMaterial(targetScene, 'pants', palette.pants)
  const shoesMaterial = createMatrixMaterial(targetScene, 'shoes', palette.shoes)

  const head = createBodyBox('avatar-head', { width: 0.9, height: 0.9, depth: 0.9 }, targetScene, root)
  head.position.y = 2.45
  head.material = skinMaterial

  const hair = createBodyBox('avatar-hair', { width: 0.94, height: 0.22, depth: 0.94 }, targetScene, root)
  hair.position.y = 2.92
  hair.material = hairMaterial

  const body = createBodyBox('avatar-body', { width: 0.9, height: 1.2, depth: 0.45 }, targetScene, root)
  body.position.y = 1.55
  body.material = shirtMaterial

  const leftArm = createBodyBox('avatar-left-arm', { width: 0.32, height: 1.15, depth: 0.38 }, targetScene, root)
  leftArm.position = new Vector3(-0.68, 1.55, 0)
  leftArm.material = skinMaterial

  const rightArm = createBodyBox('avatar-right-arm', { width: 0.32, height: 1.15, depth: 0.38 }, targetScene, root)
  rightArm.position = new Vector3(0.68, 1.55, 0)
  rightArm.material = skinMaterial

  const leftLeg = createBodyBox('avatar-left-leg', { width: 0.38, height: 1.1, depth: 0.4 }, targetScene, root)
  leftLeg.position = new Vector3(-0.23, 0.4, 0)
  leftLeg.material = pantsMaterial

  const rightLeg = createBodyBox('avatar-right-leg', { width: 0.38, height: 1.1, depth: 0.4 }, targetScene, root)
  rightLeg.position = new Vector3(0.23, 0.4, 0)
  rightLeg.material = pantsMaterial

  const leftShoe = createBodyBox('avatar-left-shoe', { width: 0.4, height: 0.16, depth: 0.44 }, targetScene, root)
  leftShoe.position = new Vector3(-0.23, -0.23, 0.02)
  leftShoe.material = shoesMaterial

  const rightShoe = createBodyBox('avatar-right-shoe', { width: 0.4, height: 0.16, depth: 0.44 }, targetScene, root)
  rightShoe.position = new Vector3(0.23, -0.23, 0.02)
  rightShoe.material = shoesMaterial

  const ground = MeshBuilder.CreateGround('avatar-builder-ground', { width: 5, height: 5 }, targetScene)
  ground.position.y = -0.32
  ground.material = createMatrixMaterial(targetScene, 'ground', new Color3(0.11, 0.15, 0.18))

  root.computeWorldMatrix(true)
  return root
}

const createBodyBox = (
  name: string,
  options: { width: number; height: number; depth: number },
  targetScene: Scene,
  parent: TransformNode,
): Mesh => {
  const mesh = MeshBuilder.CreateBox(name, options, targetScene)
  mesh.parent = parent
  return mesh
}

const refreshConnectedAvatarSvg = () => {
  if (!hasCustomAvatar.value || !avatarRoot) {
    connectedAvatarSvg.value = ''
    return
  }

  connectedAvatarSvg.value = generateCharacterPerspectiveSvg(avatarRoot, {
    width: 360,
    height: 360,
    padding: 18,
    background: 'rgba(3, 4, 8, 1)',
    yaw: -Math.PI / 4,
    pitch: 0.4,
    distanceFactor: 2.7,
    cellOverlap: 0.8,
  })
}

const createMatrixMaterial = (targetScene: Scene, name: string, baseColor: Color3) => {
  const material = new StandardMaterial(`avatar-builder-${name}`, targetScene)
  const texture = new DynamicTexture(`avatar-builder-${name}-texture`, { width: 64, height: 64 }, targetScene, false)
  const context = texture.getContext()
  const matrixSize = 8
  const cellSize = 8

  for (let y = 0; y < matrixSize; y++) {
    for (let x = 0; x < matrixSize; x++) {
      const variation = ((x * 17 + y * 11 + name.length * 7) % 5 - 2) * 0.045
      const color = varyColor(baseColor, variation)
      context.fillStyle = color.toHexString()
      context.fillRect(x * cellSize, y * cellSize, cellSize, cellSize)
    }
  }

  texture.update(false)
  texture.metadata = {
    matrixWidth: matrixSize,
    matrixHeight: matrixSize,
    pixelScale: cellSize,
  }

  material.diffuseTexture = texture
  material.diffuseColor = Color3.White()
  material.specularColor = new Color3(0.08, 0.08, 0.08)
  return material
}

const varyColor = (color: Color3, delta: number) => {
  return new Color3(
    clamp01(color.r + delta),
    clamp01(color.g + delta),
    clamp01(color.b + delta),
  )
}

const buildAvatarPalette = (avatarId: string) => {
  const hue = hashString(avatarId) % 360
  return {
    skin: new Color3(0.72, 0.52, 0.39),
    hair: colorFromHue((hue + 25) % 360, 0.42, 0.22),
    shirt: colorFromHue(hue, 0.65, 0.44),
    pants: colorFromHue((hue + 210) % 360, 0.54, 0.32),
    shoes: new Color3(0.08, 0.08, 0.08),
  }
}

const colorFromHue = (hue: number, saturation: number, lightness: number): Color3 => {
  return Color3.FromHexString(hslToHex(hue, saturation, lightness))
}

const hslToHex = (hue: number, saturation: number, lightness: number): string => {
  const c = (1 - Math.abs(2 * lightness - 1)) * saturation
  const x = c * (1 - Math.abs((hue / 60) % 2 - 1))
  const m = lightness - c / 2
  let r = 0
  let g = 0
  let b = 0

  if (hue < 60) [r, g, b] = [c, x, 0]
  else if (hue < 120) [r, g, b] = [x, c, 0]
  else if (hue < 180) [r, g, b] = [0, c, x]
  else if (hue < 240) [r, g, b] = [0, x, c]
  else if (hue < 300) [r, g, b] = [x, 0, c]
  else [r, g, b] = [c, 0, x]

  return `#${toHex(r + m)}${toHex(g + m)}${toHex(b + m)}`
}

const toHex = (value: number) => Math.round(clamp01(value) * 255).toString(16).padStart(2, '0')
const clamp01 = (value: number) => Math.min(1, Math.max(0, value))

const hashString = (value: string) => {
  let hash = 0
  for (let index = 0; index < value.length; index++) {
    hash = ((hash << 5) - hash + value.charCodeAt(index)) | 0
  }
  return Math.abs(hash)
}

const resizeEngine = () => {
  engine?.resize()
}

onMounted(async () => {
  if (!authStore.user) {
    await authStore.fetchProfile()
  }

  initializeBabylon()
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', resizeEngine)
  scene?.dispose()
  engine?.dispose()
  avatarRoot = null
  scene = null
  engine = null
})
</script>

<style scoped>
.avatar-builder {
  min-height: calc(100vh - 80px);
  padding: 2rem 1rem;
}

.voxicraft-container {
  max-width: 1400px;
  margin: 0 auto;
}

.builder-header {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.back-button {
  flex-shrink: 0;
  background-color: #424242;
  border-color: #212121;
}

.subtitle {
  margin-top: 0.5rem;
  opacity: 0.85;
}

.builder-layout {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 360px;
  gap: 2rem;
}

.viewport-card {
  min-height: 680px;
  padding: 0;
  overflow: hidden;
}

.avatar-canvas {
  display: block;
  width: 100%;
  height: 680px;
  outline: none;
  touch-action: none;
}

.tools-card,
.custom-avatar-svg-card {
  padding: 1.5rem;
}

.tools-card h2,
.custom-avatar-svg-card h2 {
  margin-bottom: 1rem;
  color: #64ffda;
}

.status-list {
  display: grid;
  gap: 0.75rem;
  margin-top: 1.5rem;
}

.status-item {
  padding: 0.75rem;
  border: 1px solid rgba(100, 255, 218, 0.25);
  border-radius: 8px;
  background: rgba(0, 0, 0, 0.2);
}

.custom-avatar-svg-card {
  margin-top: 2rem;
}

.svg-card-header {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: flex-start;
  margin-bottom: 1.5rem;
}

.custom-status {
  flex-shrink: 0;
  border-radius: 999px;
  padding: 0.45rem 0.75rem;
  font-size: 0.75rem;
  font-weight: bold;
}

.custom-status.enabled {
  background: rgba(100, 255, 218, 0.16);
  color: #64ffda;
  border: 1px solid rgba(100, 255, 218, 0.45);
}

.custom-status.disabled {
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(255, 255, 255, 0.15);
}

.svg-preview-box {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 420px;
  border: 3px solid rgba(100, 255, 218, 0.35);
  border-radius: 12px;
  background: rgba(0, 0, 0, 0.35);
  overflow: hidden;
}

.svg-preview-box :deep(svg) {
  width: min(360px, 100%);
  height: auto;
  display: block;
}

.svg-empty-state {
  padding: 2rem;
  border: 2px dashed rgba(255, 255, 255, 0.25);
  border-radius: 12px;
  color: rgba(255, 255, 255, 0.75);
  background: rgba(0, 0, 0, 0.25);
  line-height: 1.7;
}

@media (max-width: 1024px) {
  .builder-layout {
    grid-template-columns: 1fr;
  }

  .viewport-card,
  .avatar-canvas {
    min-height: 520px;
    height: 520px;
  }
}

@media (max-width: 768px) {
  .builder-header,
  .svg-card-header {
    align-items: flex-start;
    flex-direction: column;
  }

  .avatar-builder {
    padding: 1rem;
  }
}
</style>
