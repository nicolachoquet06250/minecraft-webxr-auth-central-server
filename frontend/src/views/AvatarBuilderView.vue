<template>
  <div class="avatar-builder voxicraft-bg">
    <div class="voxicraft-container builder-layout">
      <section class="voxicraft-panel viewer-card">
        <div class="header-row">
          <div class="header-actions">
            <button class="voxicraft-button back-button" type="button" @click="router.push({ name: 'profile' })">Retour au profil</button>
            <span class="status-pill">{{ overwriteAllowed ? 'Original modifiable' : 'Original protégé' }}</span>
          </div>
          <div class="header-title">
            <h1>Éditeur d'avatar</h1>
            <p class="voxicraft-text">Édition pixel par pixel avec couleur RGBA.</p>
          </div>
        </div>
        <canvas ref="canvasRef" class="avatar-canvas" />
      </section>

      <aside v-if="currentAvatar" class="voxicraft-panel editor-card">
        <template v-if="customAvatars.length > 1">
          <label class="field-label">Avatar à modifier</label>
          <select v-model="selectedAvatarId" class="form-select" @change="loadSelectedCustomAvatar">
            <option v-for="avatar in customAvatars" :key="avatar.id" :value="avatar.id">
              {{ avatar.name }}{{ avatar.is_active ? ' — actif' : '' }}
            </option>
          </select>
        </template>

        <label class="field-label">Nom</label>
        <input v-model="avatarName" class="text-input" maxlength="80" />

        <label class="field-label">Historique</label>
        <div class="tool-row">
          <button class="tool-button" type="button" :disabled="!canUndo" @click="undo">↶ Annuler</button>
          <button class="tool-button" type="button" :disabled="!canRedo" @click="redo">↷ Rétablir</button>
        </div>

        <label class="field-label">Outil</label>
        <div class="chips">
          <button class="chip" :class="{ active: selectedTool === 'brush' }" type="button" @click="selectedTool = 'brush'">Pinceau · B</button>
          <button class="chip" :class="{ active: selectedTool === 'eyedropper' }" type="button" @click="selectedTool = 'eyedropper'">Pipette · I</button>
          <button class="chip" :class="{ active: selectedTool === 'eraser' }" type="button" @click="selectedTool = 'eraser'">Gomme · E</button>
        </div>

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
        <div class="tool-row alpha-presets">
          <button class="tool-button" type="button" @click="setAlphaPreset(1)">Opaque 100%</button>
          <button class="tool-button" type="button" @click="setAlphaPreset(0.5)">Semi 50%</button>
          <button class="tool-button" type="button" @click="setAlphaPreset(0)">Invisible 0%</button>
        </div>

        <div class="grid-title">{{ partLabels[selectedPart] }} / {{ faceLabels[selectedFace] }}</div>
        <div class="pixel-grid" :style="{ gridTemplateColumns: `repeat(${activeTexture.width}, 14px)` }">
          <button v-for="pixel in facePixels" :key="`${pixel.x}-${pixel.y}`" class="pixel" type="button" :style="{ background: pixel.cssColor }" @click="handlePixelClick(pixel.x, pixel.y)" @contextmenu.prevent="pick(pixel.x, pixel.y)" />
        </div>
        <p class="help-text">Clic gauche: applique l'outil actif. Clic droit: pipette.</p>

        <label class="field-label">Import / Export JSON</label>
        <div class="import-export-actions">
          <button class="tool-button" type="button" @click="exportTextureJson">Exporter</button>
          <button class="tool-button" type="button" :disabled="!textureJson.trim()" @click="copyTextureJson">Copier</button>
          <button class="tool-button" type="button" :disabled="!textureJson.trim()" @click="importTextureJson">Importer</button>
        </div>
        <textarea v-model="textureJson" class="json-textarea" spellcheck="false" placeholder="texture_data JSON"></textarea>

        <div class="actions">
          <button class="action primary" type="button" :disabled="saving || !avatarName.trim()" @click="saveCopy">Enregistrer comme copie</button>
          <button class="action secondary" type="button" :disabled="saving || !overwriteAllowed || !avatarName.trim()" @click="overwrite">Écraser l'original</button>
          <button class="action danger" type="button" :disabled="saving || !canDeleteAvatar" @click="deleteCurrentAvatar">Supprimer l'avatar</button>
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
import { avatarApi, type AvatarTextureData, type UserAvatar } from '@/api'
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

type EditorTool = 'brush' | 'eyedropper' | 'eraser'

const router = useRouter()
const authStore = useAuthStore()
const canvasRef = ref<HTMLCanvasElement | null>(null)
const currentAvatar = ref<EditableAvatar | null>(null)
const customAvatars = ref<UserAvatar[]>([])
const selectedAvatarId = ref('')
const avatarName = ref('')
const selectedPart = ref<AvatarPartName>('head')
const selectedFace = ref<AvatarFaceName>('front')
const selectedTool = ref<EditorTool>('brush')
const selectedHex = ref('#000000')
const selectedAlpha = ref(1)
const textureJson = ref('')
const undoStack = ref<EditableAvatar[]>([])
const redoStack = ref<EditableAvatar[]>([])
const saving = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

// Les noms techniques restent anatomiques côté modèle (`rightArm` = bras droit du personnage).
// L'éditeur est affiché face au personnage : son bras droit apparaît donc à gauche de l'écran.
// Les libellés ci-dessous sont volontairement exprimés du point de vue de l'utilisateur qui regarde l'avatar.
const partLabels: Record<AvatarPartName, string> = {
  head: 'Tête',
  torso: 'Torse',
  rightArm: 'Bras gauche',
  leftArm: 'Bras droit',
  rightLeg: 'Jambe gauche',
  leftLeg: 'Jambe droite',
}
const faceLabels: Record<AvatarFaceName, string> = { front: 'Avant', back: 'Arrière', top: 'Dessus', bottom: 'Dessous', left: 'Gauche', right: 'Droite' }

let engine: Engine | null = null
let scene: Scene | null = null
let avatarRoot: Mesh | null = null

const overwriteAllowed = computed(() => !!currentAvatar.value && !currentAvatar.value.overwriteLocked)
const canDeleteAvatar = computed(() => !!currentAvatar.value && currentAvatar.value.source === 'custom' && !currentAvatar.value.overwriteLocked)
const canUndo = computed(() => undoStack.value.length > 0)
const canRedo = computed(() => redoStack.value.length > 0)
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
  window.addEventListener('keydown', handleKeyboardShortcut)
})

onBeforeUnmount(() => {
  canvasRef.value?.removeEventListener('wheel', preventCanvasPageScroll)
  window.removeEventListener('resize', resize)
  window.removeEventListener('keydown', handleKeyboardShortcut)
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
  await refreshCustomAvatars()

  const activeCustom = customAvatars.value.find((avatar) => avatar.is_active)
  if (activeCustom) {
    setCurrentCustomAvatar(activeCustom)
    return
  }

  const active = await avatarApi.getActive().then((response) => response.data).catch(() => null)
  if (active?.avatar) {
    setCurrentCustomAvatar(active.avatar)
    return
  }

  currentAvatar.value = createEditableAvatar(authStore.user?.avatar || 'alex')
  selectedAvatarId.value = ''
  avatarName.value = currentAvatar.value.name
  resetHistory()
}

async function refreshCustomAvatars(preferredAvatarId?: string) {
  customAvatars.value = await avatarApi.list().then((response) => response.data).catch(() => [])
  if (preferredAvatarId) selectedAvatarId.value = preferredAvatarId
}

function setCurrentCustomAvatar(avatar: UserAvatar) {
  currentAvatar.value = createEditableAvatarFromApi(avatar)
  selectedAvatarId.value = avatar.id
  avatarName.value = avatar.name
  resetHistory()
}

function loadSelectedCustomAvatar() {
  const avatar = customAvatars.value.find((item) => item.id === selectedAvatarId.value)
  if (avatar) setCurrentCustomAvatar(avatar)
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

function handlePixelClick(x: number, y: number) {
  if (selectedTool.value === 'eyedropper') {
    pick(x, y)
    return
  }
  if (selectedTool.value === 'eraser') {
    erase(x, y)
    return
  }
  paint(x, y)
}

function paint(x: number, y: number) {
  if (!currentAvatar.value) return
  pushHistory()
  currentAvatar.value = paintAvatarPixel(currentAvatar.value, selectedPart.value, selectedFace.value, x, y, selectedColor.value)
}

function erase(x: number, y: number) {
  if (!currentAvatar.value) return
  pushHistory()
  currentAvatar.value = paintAvatarPixel(currentAvatar.value, selectedPart.value, selectedFace.value, x, y, [0, 0, 0, 0])
}

function pick(x: number, y: number) {
  const color = getTexturePixelColor(activeTexture.value, x, y)
  selectedHex.value = rgbaToHex(color)
  selectedAlpha.value = color[3]
}

function setAlphaPreset(alpha: number) {
  selectedAlpha.value = alpha
}

function pushHistory() {
  if (!currentAvatar.value) return
  undoStack.value.push(cloneAvatar(currentAvatar.value))
  if (undoStack.value.length > 100) undoStack.value.shift()
  redoStack.value = []
}

function undo() {
  if (!currentAvatar.value || undoStack.value.length === 0) return
  redoStack.value.push(cloneAvatar(currentAvatar.value))
  currentAvatar.value = undoStack.value.pop()!
}

function redo() {
  if (!currentAvatar.value || redoStack.value.length === 0) return
  undoStack.value.push(cloneAvatar(currentAvatar.value))
  currentAvatar.value = redoStack.value.pop()!
}

function resetHistory() {
  undoStack.value = []
  redoStack.value = []
  textureJson.value = ''
}

function cloneAvatar(avatar: EditableAvatar): EditableAvatar {
  return JSON.parse(JSON.stringify(avatar)) as EditableAvatar
}

function handleKeyboardShortcut(event: KeyboardEvent) {
  const target = event.target as HTMLElement | null
  const tagName = target?.tagName.toLowerCase()
  const isTextInput = tagName === 'input' || tagName === 'textarea' || tagName === 'select' || target?.isContentEditable
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'z') {
    event.preventDefault()
    if (event.shiftKey) redo()
    else undo()
    return
  }
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'y') {
    event.preventDefault()
    redo()
    return
  }
  if (isTextInput || event.ctrlKey || event.metaKey || event.altKey) return
  const key = event.key.toLowerCase()
  if (key === 'b') selectedTool.value = 'brush'
  if (key === 'i') selectedTool.value = 'eyedropper'
  if (key === 'e') selectedTool.value = 'eraser'
}

function exportTextureJson() {
  if (!currentAvatar.value) return
  textureJson.value = JSON.stringify(texturesToTextureData({ ...currentAvatar.value, name: avatarName.value.trim() || currentAvatar.value.name }), null, 2)
  successMessage.value = 'Texture exportée dans la zone JSON.'
  errorMessage.value = ''
}

async function copyTextureJson() {
  if (!textureJson.value.trim()) return
  await navigator.clipboard.writeText(textureJson.value)
  successMessage.value = 'Texture JSON copiée.'
  errorMessage.value = ''
}

function importTextureJson() {
  if (!currentAvatar.value || !textureJson.value.trim()) return
  try {
    const textureData = JSON.parse(textureJson.value) as AvatarTextureData
    validateTextureData(textureData)
    pushHistory()
    const importedAvatar: UserAvatar = {
      id: currentAvatar.value.id,
      name: avatarName.value.trim() || currentAvatar.value.name,
      base_kind: currentAvatar.value.baseKind,
      is_active: false,
      texture_data: textureData,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    }
    currentAvatar.value = createEditableAvatarFromApi(importedAvatar)
    successMessage.value = 'Texture JSON importée.'
    errorMessage.value = ''
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'JSON de texture invalide.'
  }
}

function validateTextureData(textureData: AvatarTextureData) {
  if (textureData.version !== 1) throw new Error('Version texture_data invalide.')
  if (!textureData.palette || typeof textureData.palette !== 'object') throw new Error('Palette manquante.')
  if (!textureData.parts || typeof textureData.parts !== 'object') throw new Error('Parts manquant.')
  for (const part of avatarPartNames) {
    if (!textureData.parts[part]) throw new Error(`Partie manquante: ${part}`)
    for (const face of avatarFaceNames) {
      const texture = textureData.parts[part][face]
      if (!texture || typeof texture.width !== 'number' || typeof texture.height !== 'number' || !Array.isArray(texture.matrix)) {
        throw new Error(`Face invalide: ${part}.${face}`)
      }
    }
  }
}

async function saveCopy() {
  if (!currentAvatar.value || !avatarName.value.trim()) return
  saving.value = true
  successMessage.value = ''
  errorMessage.value = ''
  try {
    const created = await avatarApi.createCopy({ name: avatarName.value.trim(), base_kind: currentAvatar.value.baseKind, texture_data: texturesToTextureData({ ...currentAvatar.value, name: avatarName.value.trim() }) }).then((response) => response.data)
    await avatarApi.select(created.id)
    await refreshCustomAvatars(created.id)
    setCurrentCustomAvatar({ ...created, is_active: true })
    successMessage.value = 'Copie enregistrée.'
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
    customAvatars.value = customAvatars.value.map((avatar) => avatar.id === updated.id ? { ...updated, is_active: avatar.is_active } : avatar)
    setCurrentCustomAvatar({ ...updated, is_active: customAvatars.value.find((avatar) => avatar.id === updated.id)?.is_active ?? false })
    successMessage.value = 'Avatar écrasé.'
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Erreur lors de la sauvegarde.'
  } finally {
    saving.value = false
  }
}

async function deleteCurrentAvatar() {
  if (!currentAvatar.value || !canDeleteAvatar.value) return
  const deletedId = currentAvatar.value.id
  const deletedAvatar = customAvatars.value.find((avatar) => avatar.id === deletedId)
  const deletedIndex = customAvatars.value.findIndex((avatar) => avatar.id === deletedId)
  const nextAvatarId = findPreviousInactiveAvatarId(deletedIndex, deletedId)

  saving.value = true
  successMessage.value = ''
  errorMessage.value = ''

  try {
    await avatarApi.delete(deletedId)
    await refreshCustomAvatars(nextAvatarId)

    const nextAvatar = nextAvatarId
      ? customAvatars.value.find((avatar) => avatar.id === nextAvatarId)
      : customAvatars.value.find((avatar) => !avatar.is_active) ?? customAvatars.value[0]

    if (nextAvatar) {
      if (deletedAvatar?.is_active) {
        await avatarApi.select(nextAvatar.id)
        await refreshCustomAvatars(nextAvatar.id)
        setCurrentCustomAvatar({ ...nextAvatar, is_active: true })
      } else {
        setCurrentCustomAvatar(nextAvatar)
      }
    } else {
      currentAvatar.value = createEditableAvatar(authStore.user?.avatar || 'alex')
      selectedAvatarId.value = ''
      avatarName.value = currentAvatar.value.name
      resetHistory()
    }

    successMessage.value = 'Avatar supprimé.'
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Erreur lors de la suppression.'
  } finally {
    saving.value = false
  }
}

function findPreviousInactiveAvatarId(deletedIndex: number, deletedId: string): string | undefined {
  if (deletedIndex <= 0) return undefined
  for (let index = deletedIndex - 1; index >= 0; index -= 1) {
    const avatar = customAvatars.value[index]
    if (avatar.id !== deletedId && !avatar.is_active) return avatar.id
  }
  return undefined
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
.text-input, .form-select { padding: .75rem 1rem; border-radius: 8px; border: 2px solid #424242; background: rgba(0,0,0,.55); color: #fff; width: 100%; min-width: 0; }
.chips, .tool-row, .import-export-actions { display: flex; flex-wrap: wrap; gap: .5rem; }
.chip, .tool-button { border: 2px solid rgba(100,255,218,.2); background: rgba(0,0,0,.3); color: #fff; border-radius: 999px; padding: .55rem .75rem; cursor: pointer; }
.chip.active { border-color: #64ffda; color: #64ffda; background: rgba(100,255,218,.18); }
.tool-button:disabled { opacity: .45; cursor: not-allowed; }
.color-row { display: grid; grid-template-columns: auto 1fr auto; gap: 1rem; align-items: center; color: #fff; }
.color-input { width: 64px; height: 64px; }
.alpha-field { display: flex; flex-direction: column; gap: .4rem; }
.swatch { width: 52px; height: 52px; border-radius: 8px; border: 2px solid rgba(255,255,255,.3); }
.pixel-grid { display: grid; gap: 2px; width: max-content; max-width: 100%; overflow: auto; padding: .5rem; border-radius: 10px; background: rgba(0,0,0,.35); }
.pixel { width: 14px; height: 14px; border: 1px solid rgba(255,255,255,.12); cursor: crosshair; padding: 0; }
.help-text { color: rgba(255,255,255,.75); margin: 0; }
.json-textarea { min-height: 160px; resize: vertical; border-radius: 10px; border: 2px solid #424242; background: rgba(0,0,0,.55); color: #d8fff6; padding: .75rem; font-family: monospace; font-size: .8rem; }
.actions { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
.action { padding: .9rem 1rem; border-radius: 8px; border: 3px solid transparent; font-weight: 700; cursor: pointer; }
.action.primary { background: #4caf50; border-color: #2e7d32; color: #fff; }
.action.secondary { background: #ff9800; border-color: #ef6c00; color: #1a1a1a; }
.action.danger { grid-column: 1 / -1; background: #d32f2f; border-color: #8b1a1a; color: #fff; }
.action:disabled { opacity: .5; cursor: not-allowed; }
.success { color: #7cfc9a; font-weight: 700; }
.error { color: #ff8a80; font-weight: 700; }
@media (max-width: 1100px) { .builder-layout { grid-template-columns: 1fr; } .avatar-canvas { height: 520px; } }
@media (max-width: 720px) { .avatar-builder { padding: 1rem; } .color-row, .actions { grid-template-columns: 1fr; } .action.danger { grid-column: auto; } }
</style>
