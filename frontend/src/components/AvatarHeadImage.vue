<template>
  <img :src="svgUrl" :alt="altText" class="avatar-head-image" />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { UserAvatar } from '@/api'
import { createEditableAvatarFromApi } from '@/character-builder/avatar-editor'
import { alexModelTextures } from '@/character-builder/alex-color-matrices'
import { steveModelTextures } from '@/character-builder/steve-color-matrices'
import type { TextureMatrix } from '@/character-builder/types'

const props = defineProps<{ avatar?: string; customAvatar?: UserAvatar }>()

const texture = computed<TextureMatrix>(() => {
  if (props.customAvatar) return createEditableAvatarFromApi(props.customAvatar).textures.head.front
  return props.avatar === 'steve' ? steveModelTextures.head.front : alexModelTextures.head.front
})

const altText = computed(() => props.customAvatar ? `Avatar ${props.customAvatar.name}` : props.avatar || 'avatar')

const svgUrl = computed(() => {
  const face = texture.value
  const cell = 12
  let rects = ''
  for (let y = 0; y < face.matrix.length; y += 1) {
    const row = face.matrix[y]
    for (let x = 0; x < row.length; x += 1) {
      const color = face.palette[row[x]]
      if (!color || color[3] <= 0) continue
      const fill = `rgba(${Math.round(color[0] * 255)},${Math.round(color[1] * 255)},${Math.round(color[2] * 255)},${color[3]})`
      rects += `<rect x="${x * cell}" y="${y * cell}" width="${cell}" height="${cell}" fill="${fill}"/>`
    }
  }
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${face.width * cell}" height="${face.height * cell}" viewBox="0 0 ${face.width * cell} ${face.height * cell}" shape-rendering="crispEdges">${rects}</svg>`
  return `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`
})
</script>

<style scoped>
.avatar-head-image { width: 100%; height: 100%; display: block; object-fit: cover; image-rendering: pixelated; image-rendering: crisp-edges; }
</style>