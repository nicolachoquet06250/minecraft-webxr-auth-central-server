<template>
  <div class="avatar-svg-preview">
    <div v-if="svgUrl" class="svg-preview-box">
      <img :src="svgUrl" :alt="altText" class="svg-image" />
    </div>
    <div v-else class="svg-empty-state">Génération SVG en attente.</div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { NullEngine, Scene, Mesh, Vector3 } from '@babylonjs/core'
import type { UserAvatar } from '@/api'
import { buildCharacter } from '@/character-builder/character-builder'
import { generateCharacterPerspectiveSvg } from '@/character-builder/svg-export'
import { createCharacterModelFromAvatar, createEditableAvatar, createEditableAvatarFromApi, getSelectedAvatarName } from '@/character-builder/avatar-editor'

const props = defineProps<{ avatar?: string; customAvatar?: UserAvatar }>()
const svgUrl = ref('')
let engine: NullEngine | null = null
let scene: Scene | null = null
let mesh: Mesh | null = null

const altText = computed(() => props.customAvatar ? `Avatar ${props.customAvatar.name}` : 'Avatar SVG')

const renderSvg = () => {
  disposeScene()
  engine = new NullEngine()
  scene = new Scene(engine)
  const avatar = props.customAvatar
    ? createEditableAvatarFromApi(props.customAvatar)
    : createEditableAvatar(getSelectedAvatarName(props.avatar))
  mesh = buildCharacter(scene, createCharacterModelFromAvatar(avatar), Vector3.Zero(), { physics: false })
  const svg = generateCharacterPerspectiveSvg(mesh, { width: 280, height: 280, padding: 14, background: 'rgba(3, 4, 8, 1)' })
  svgUrl.value = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`
}

const disposeScene = () => {
  mesh = null
  scene?.dispose()
  engine?.dispose()
  scene = null
  engine = null
}

watch(() => props.avatar, renderSvg)
watch(() => props.customAvatar, renderSvg, { deep: true })
onMounted(renderSvg)
onBeforeUnmount(disposeScene)
</script>

<style scoped>
.avatar-svg-preview { width: 100%; }
.svg-preview-box { display: flex; align-items: center; justify-content: center; min-height: 300px; border: 3px solid rgba(100, 255, 218, 0.35); border-radius: 12px; background: rgba(0, 0, 0, 0.35); overflow: hidden; }
.svg-image { width: min(280px, 100%); height: auto; display: block; image-rendering: crisp-edges; }
.svg-empty-state { padding: 2rem; border: 2px dashed rgba(255, 255, 255, 0.25); border-radius: 12px; color: rgba(255, 255, 255, 0.75); background: rgba(0, 0, 0, 0.25); line-height: 1.7; }
</style>