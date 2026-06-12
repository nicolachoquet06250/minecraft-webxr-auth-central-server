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
            Première scène Babylon.js dédiée à la personnalisation graphique de l'avatar.
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
            <div class="status-item">✅ Avatar placeholder voxel</div>
          </div>
        </aside>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import '@babylonjs/loaders'
import {
  ArcRotateCamera,
  Color3,
  Color4,
  Engine,
  HemisphericLight,
  MeshBuilder,
  Scene,
  StandardMaterial,
  Vector3,
} from '@babylonjs/core'

const router = useRouter()
const canvasRef = ref<HTMLCanvasElement | null>(null)

let engine: Engine | null = null
let scene: Scene | null = null

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

  createVoxelAvatarPlaceholder(scene)

  engine.runRenderLoop(() => {
    scene?.render()
  })

  window.addEventListener('resize', resizeEngine)
}

const createVoxelAvatarPlaceholder = (targetScene: Scene) => {
  const skinMaterial = createMaterial(targetScene, 'skin', new Color3(0.72, 0.52, 0.39))
  const hairMaterial = createMaterial(targetScene, 'hair', new Color3(0.22, 0.13, 0.08))
  const shirtMaterial = createMaterial(targetScene, 'shirt', new Color3(0.08, 0.48, 0.63))
  const pantsMaterial = createMaterial(targetScene, 'pants', new Color3(0.15, 0.20, 0.42))
  const shoesMaterial = createMaterial(targetScene, 'shoes', new Color3(0.08, 0.08, 0.08))

  const head = MeshBuilder.CreateBox('avatar-head', { width: 0.9, height: 0.9, depth: 0.9 }, targetScene)
  head.position.y = 2.45
  head.material = skinMaterial

  const hair = MeshBuilder.CreateBox('avatar-hair', { width: 0.94, height: 0.22, depth: 0.94 }, targetScene)
  hair.position.y = 2.92
  hair.material = hairMaterial

  const body = MeshBuilder.CreateBox('avatar-body', { width: 0.9, height: 1.2, depth: 0.45 }, targetScene)
  body.position.y = 1.55
  body.material = shirtMaterial

  const leftArm = MeshBuilder.CreateBox('avatar-left-arm', { width: 0.32, height: 1.15, depth: 0.38 }, targetScene)
  leftArm.position = new Vector3(-0.68, 1.55, 0)
  leftArm.material = skinMaterial

  const rightArm = MeshBuilder.CreateBox('avatar-right-arm', { width: 0.32, height: 1.15, depth: 0.38 }, targetScene)
  rightArm.position = new Vector3(0.68, 1.55, 0)
  rightArm.material = skinMaterial

  const leftLeg = MeshBuilder.CreateBox('avatar-left-leg', { width: 0.38, height: 1.1, depth: 0.4 }, targetScene)
  leftLeg.position = new Vector3(-0.23, 0.4, 0)
  leftLeg.material = pantsMaterial

  const rightLeg = MeshBuilder.CreateBox('avatar-right-leg', { width: 0.38, height: 1.1, depth: 0.4 }, targetScene)
  rightLeg.position = new Vector3(0.23, 0.4, 0)
  rightLeg.material = pantsMaterial

  const leftShoe = MeshBuilder.CreateBox('avatar-left-shoe', { width: 0.4, height: 0.16, depth: 0.44 }, targetScene)
  leftShoe.position = new Vector3(-0.23, -0.23, 0.02)
  leftShoe.material = shoesMaterial

  const rightShoe = MeshBuilder.CreateBox('avatar-right-shoe', { width: 0.4, height: 0.16, depth: 0.44 }, targetScene)
  rightShoe.position = new Vector3(0.23, -0.23, 0.02)
  rightShoe.material = shoesMaterial

  const ground = MeshBuilder.CreateGround('avatar-builder-ground', { width: 5, height: 5 }, targetScene)
  ground.position.y = -0.32
  ground.material = createMaterial(targetScene, 'ground', new Color3(0.11, 0.15, 0.18))
}

const createMaterial = (targetScene: Scene, name: string, color: Color3) => {
  const material = new StandardMaterial(`avatar-builder-${name}`, targetScene)
  material.diffuseColor = color
  material.specularColor = new Color3(0.08, 0.08, 0.08)
  return material
}

const resizeEngine = () => {
  engine?.resize()
}

onMounted(() => {
  initializeBabylon()
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', resizeEngine)
  scene?.dispose()
  engine?.dispose()
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

.tools-card {
  padding: 1.5rem;
}

.tools-card h2 {
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
  .builder-header {
    align-items: flex-start;
    flex-direction: column;
  }

  .avatar-builder {
    padding: 1rem;
  }
}
</style>
