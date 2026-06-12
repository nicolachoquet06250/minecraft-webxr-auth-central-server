<template>
  <div class="avatar-builder voxicraft-bg">
    <div class="voxicraft-container builder-layout">
      <section class="voxicraft-panel viewer-card">
        <div class="header-row">
          <div class="header-actions">
            <button class="voxicraft-button back-button" type="button" @click="router.push({ name: 'profile' })">Retour au profil</button>
            <span class="status-pill">{{ overwriteAllowed ? 'Original modifiable' : 'Original protege' }}</span>
          </div>
          <div class="header-title">
            <h1>Builder d'avatar</h1>
            <p class="voxicraft-text">Edition pixel par pixel avec couleur RGBA.</p>
          </div>
        </div>
        <canvas ref="canvasRef" class="avatar-canvas" />
      </section>

      <aside v-if="currentAvatar" class="voxicraft-panel editor-card">
        <label class="field-label">Nom</label>
        <input v-model="avatarName" class="text-input" maxlength="80" />

        <label class="field-label">Partie</label>
        <div class="chips">
          <button v-for="part in avatarPartNames" :key="part" class="chip" :class="{ active: selectedPart === part }" type="button" @click="selectedPart = part">
            {{ partLabels[part] }}
          </button>
        </div>

        <label class="field-label">Face</label>
        <div class="chips">
          <button v-for="face in avatarFaceNames" :key="face" class="chip" :class="{ active: selectedFace === face }" type="button" @click="selectedFace = face">
            {{ faceLabels[face] }}
          </button>
        </div>

        <label class="field-label">Couleur</label>
        <div class="color-row">
          <input v-model="selectedHex" type="color" class="color-input" />
          <label class="alpha-field">
            Alpha {{ selectedAlpha.toFixed(2) }}
            <input v-model.number="selectedAlpha" type="range" min="0" max="1" step="0.01" />
          </label>
          <span class="swatch" :style="{ background: selectedColorCss }"></span>
        </div>

        <div class="grid-title">{{ partLabels[selectedPart] }} / {{ faceLabels[selectedFace] }}</div>
        <div class="pixel-grid" :style="{ gridTemplateColumns: `repeat(${activeTexture.width}, 14px)` }">
          <button
            v-for="pixel in facePixels"
            :key="`${pixel.x}-${pixel.y}`"
            class="pixel"
            type="button"
            :style="{ background: pixel.cssColor }"
            @click="paint(pixel.x, pixel.y)"
            @contextmenu.prevent="pick(pixel.x, pixel.y)"
          />
        </div>
        <p class="help-text">Clic gauche: peindre. Clic droit: reprendre la couleur.</p>

        <div class="actions">
          <button class="action primary" type="button" :disabled="saving || !avatarName.trim()" @click="saveCopy">Enregistrer comme copie</button>
          <button class="action secondary" type="button" :disabled="saving || !overwriteAllowed || !avatarName.trim()" @click="overwrite">Ecraser l'original</button>
        </div>

        <p v-if="successMessage" class="success">{{ successMessage }}</p>
        <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import '@babylonjs/loaders'
import { ArcRotateCamera, Color4, Engine, HemisphericLight, Mesh, Scene, Vector3 } from '@babylonjs/core'
import { avatarApi } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { buildCharacter } from '@/character-builder/character-builder'
import {
  avatarFaceNames,
  avatarPartNames,
  colorToCss,
  createCharacterModelFromAvatar,
  createEditableAvatar,
  createEditableAvatarFromApi,
  getTexturePixelColor,
  hexToRgba,
  paintAvatarPixel,
  rgbaToHex,
  texturesToTextureData,
  type AvatarFaceName,
  type AvatarPartName,
  type EditableAvatar,
} from '@/character-builder/avatar-editor'

const router = useRouter()
const authStore = useAuthStore()
const canvasRef = ref<HTMLCanvasElement | null>(null)
const currentAvatar = ref<EditableAvatar | null>(null)
const avatarName = ref('')
const selectedPart = ref<AvatarPartName>('head')
const selectedFace = ref<AvatarFaceName>('front')
const selectedHex = ref('#000000')
const selectedAlpha = ref(1)
const saving = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

const partLabels: Record<AvatarPartName, string> = { head: 'Tete', torso: 'Torse', rightArm: 'Bras droit', leftArm: 'Bras gauche', rightLeg: 'Jambe droite', leftLeg: 'Jambe gauche' }
const faceLabels: Record<AvatarFaceName, string> = { front: 'Avant', back: 'Arriere', top: 'Dessus', bottom: 'Dessous', left: 'Gauche', right: 'Droite' }

let engine: Engine | null = null
let scene: Scene | null = null
let avatarRoot: Mesh | null = null

const overwriteAllowed = computed(() => !!currentAvatar.value && !currentAvatar.value.overwriteLocked)
const activeTexture = computed(() => currentAvatar.value!.textures[selectedPart.value][selectedFace.value])
const selectedColor = computed(() => hexToRgba(selectedHex.value, selectedAlpha.value))
const selectedColorCss = computed(() => colorToCss(selectedColor.value))
const facePixels = computed(() => {
  const texture = activeTexture.value
  const pixels: Array<{ x: number; y: number; cssColor: string }> = []
  for (let y = 0; y < texture.matrix.length; y += 1) {
    for (let x = 0; x < texture.matrix[y].length; x += 1) {
      pixels.push({ x, y, cssColor: colorToCss(getTexturePixelColor(texture, x, y)) })
    }
  }
  return pixels
})

onMounted(async () => {
  await loadAvatar()
  await nextTick()
  initScene()
  renderAvatar()
})

onBeforeUnmount(() => {
  canvasRef.value?.removeEventListener('wheel', preventCanvasPageScroll)
  window.removeEventListener('resize', resize)
  avatarRoot?.dispose()
  scene?.dispose()
  engine?.dispose()
})

watch(() => currentAvatar.value, () => {
  if (!currentAvatar.value) return
  avatarName.value = currentAvatar.value.name
  syncColorFromPixel()
  renderAvatar()
}, { deep: true })

watch([selectedPart, selectedFace], syncColorFromPixel)

async function loadAvatar() {
  if (!authStore.user) await authStore.fetchProfile()
  const active = await avatarApi.getActive().then((response) => response.data).catch(() => null)
  if (active?.avatar) {
    currentAvatar.value = createEditableAvatarFromApi(active.avatar)
  } else {
    currentAvatar.value = createEditableAvatar(authStore.user?.avatar || 'alex')
  }
  avatarName.value = currentAvatar.value.name
}

function preventCanvasPageScroll(event: WheelEvent) {
  event.preventDefault()
}

function initScene() {
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
  canvas.addEventListener('wheel', preventCanvasPageScroll, { passive: false })
  engine.runRenderLoop(() => scene?.render())
  window.addEventListener('resize', resize)
}

function renderAvatar() {
  if (!scene || !currentAvatar.value) return
  avatarRoot?.dispose()
  avatarRoot = buildCharacter(scene, createCharacterModelFromAvatar(currentAvatar.value), Vector3.Zero(), { physics: false })
}

function resize() { engine?.resize() }

function syncColorFromPixel() {
  if (!currentAvatar.value) return
  const color = getTexturePixelColor(activeTexture.value, 0, 0)
  selectedHex.value = rgbaToHex(color)
  selectedAlpha.value = color[3]
}

function paint(x: number, y: number) {
  if (!currentAvatar.value) return
  currentAvatar.value = paintAvatarPixel(currentAvatar.value, selectedPart.value, selectedFace.value, x, y, selectedColor.value)
}

function pick(x: number, y: number) {
  const color = getTexturePixelColor(activeTexture.value, x, y)
  selectedHex.value = rgbaToHex(color)
  selectedAlpha.value = color[3]
}

async function saveCopy() {
  if (!currentAvatar.value || !avatarName.value.trim()) return
  saving.value = true
  successMessage.value = ''
  errorMessage.value = ''
  try {
    const created = await avatarApi.createCopy({ name: avatarName.value.trim(), base_kind: currentAvatar.value.baseKind, texture_data: texturesToTextureData({ ...currentAvatar.value, name: avatarName.value.trim() }) }).then((response) => response.data)
    await avatarApi.select(created.id)
    currentAvatar.value = createEditableAvatarFromApi(created)
    successMessage.value = 'Copie enregistree.'
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Erreur lors de la sauvegarde.'
  } finally {
    saving.value = false
  }
}

async function overwrite() {
  if (!currentAvatar.value || !avatarName.value.trim() || currentAvatar.value.overwriteLocked) return
  saving.value = true
  successMessage.value = ''
  errorMessage.value = ''
  try {
    const updated = await avatarApi.update(currentAvatar.value.id, { name: avatarName.value.trim(), texture_data: texturesToTextureData({ ...currentAvatar.value, name: avatarName.value.trim() }) }).then((response) => response.data)
    currentAvatar.value = createEditableAvatarFromApi(updated)
    successMessage.value = 'Avatar ecrase.'
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Erreur lors de la sauvegarde.'
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.avatar-builder { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.voxicraft-container { max-width: 1500px; margin: 0 auto; }
.builder-layout { display: grid; grid-template-columns: minmax(0, 1.15fr) minmax(420px, 0.85fr); gap: 2rem; align-items: start; }
.viewer-card, .editor-card { padding: 1.5rem; }
.header-row { display: flex; flex-direction: column; gap: 1rem; margin-bottom: 1rem; overflow: hidden; }
.header-actions { display: flex; flex-direction: column; align-items: flex-start; gap: .75rem; min-width: 0; max-width: 100%; }
.header-title { min-width: 0; max-width: 100%; }
.header-title h1 { margin: 0 0 .5rem; color: #64ffda; font-size: 1.35rem; line-height: 1.15; overflow-wrap: anywhere; }
.back-button { max-width: 100%; width: fit-content; }
.avatar-canvas { display: block; width: 100%; height: 680px; outline: none; touch-action: none; border-radius: 12px; background: rgba(0, 0, 0, 0.35); }
.status-pill { display: inline-block; max-width: 100%; box-sizing: border-box; border: 1px solid rgba(100,255,218,.45); color: #64ffda; border-radius: 999px; padding: .5rem .75rem; background: rgba(0,0,0,.25); white-space: normal; text-align: left; overflow-wrap: anywhere; }
.editor-card { display: flex; flex-direction: column; gap: 1rem; }
.field-label, .grid-title { color: #ffd700; font-weight: 700; }
.text-input { padding: .75rem 1rem; border-radius: 8px; border: 2px solid #424242; background: rgba(0,0,0,.55); color: #fff; }
.chips { display: flex; flex-wrap: wrap; gap: .5rem; }
.chip { border: 2px solid rgba(100,255,218,.2); background: rgba(0,0,0,.3); color: #fff; border-radius: 999px; padding: .55rem .75rem; cursor: pointer; }
.chip.active { border-color: #64ffda; color: #64ffda; background: rgba(100,255,218,.18); }
.color-row { display: grid; grid-template-columns: auto 1fr auto; gap: 1rem; align-items: center; color: #fff; }
.color-input { width: 64px; height: 64px; }
.alpha-field { display: flex; flex-direction: column; gap: .4rem; }
.swatch { width: 52px; height: 52px; border-radius: 8px; border: 2px solid rgba(255,255,255,.3); }
.pixel-grid { display: grid; gap: 2px; width: max-content; max-width: 100%; overflow: auto; padding: .5rem; border-radius: 10px; background: rgba(0,0,0,.35); }
.pixel { width: 14px; height: 14px; border: 1px solid rgba(255,255,255,.12); cursor: crosshair; padding: 0; }
.help-text { color: rgba(255,255,255,.75); margin: 0; }
.actions { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
.action { padding: .9rem 1rem; border-radius: 8px; border: 3px solid transparent; font-weight: 700; cursor: pointer; }
.action.primary { background: #4caf50; border-color: #2e7d32; color: #fff; }
.action.secondary { background: #ff9800; border-color: #ef6c00; color: #1a1a1a; }
.action:disabled { opacity: .5; cursor: not-allowed; }
.success { color: #7cfc9a; font-weight: 700; }
.error { color: #ff8a80; font-weight: 700; }
@media (max-width: 1100px) { .builder-layout { grid-template-columns: 1fr; } .avatar-canvas { height: 520px; } }
@media (max-width: 720px) { .avatar-builder { padding: 1rem; } .color-row, .actions { grid-template-columns: 1fr; } }
</style>
