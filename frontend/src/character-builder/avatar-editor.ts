import type { BodyPartFaces, CharacterModel, TextureMatrix } from './types'
import { alexModelTextures } from './alex-color-matrices'
import { steveModelTextures } from './steve-color-matrices'

export type AvatarPartName = 'head' | 'torso' | 'rightArm' | 'leftArm' | 'rightLeg' | 'leftLeg'
export type AvatarFaceName = keyof BodyPartFaces
export type AvatarSource = 'steve' | 'alex' | 'custom'

export type EditableAvatar = {
  id: string
  name: string
  source: AvatarSource
  overwriteLocked: boolean
  textures: Record<AvatarPartName, BodyPartFaces>
}

const STORAGE_KEY = 'voxicraft.customAvatars.v1'
const SELECTED_KEY = 'voxicraft.selectedCustomAvatarId'

const partNames: AvatarPartName[] = ['head', 'torso', 'rightArm', 'leftArm', 'rightLeg', 'leftLeg']
const faceNames: AvatarFaceName[] = ['front', 'back', 'top', 'bottom', 'left', 'right']

export const avatarPartNames = partNames
export const avatarFaceNames = faceNames

export function createEditableAvatar(avatarName: string | undefined): EditableAvatar {
  const normalized = avatarName?.trim() || 'alex'
  const saved = normalized.startsWith('custom:') ? loadCustomAvatar(normalized.slice('custom:'.length)) : null
  if (saved) return saved

  if (normalized === 'steve') {
    return {
      id: 'steve',
      name: 'Steve',
      source: 'steve',
      overwriteLocked: true,
      textures: cloneTextures(steveModelTextures),
    }
  }

  return {
    id: normalized === 'alex' ? 'alex' : `custom-${Date.now()}`,
    name: normalized === 'alex' ? 'Alex' : normalized,
    source: normalized === 'alex' ? 'alex' : 'custom',
    overwriteLocked: normalized === 'alex',
    textures: cloneTextures(alexModelTextures),
  }
}

export function createCharacterModelFromAvatar(avatar: EditableAvatar): CharacterModel {
  const isSteve = avatar.source === 'steve'
  const armWidth = isSteve ? 0.25 : 0.1875
  const armOffset = isSteve ? 0.375 : 0.34375

  return {
    name: 'connectedAvatar',
    bodyType: isSteve ? 'masculine' : 'custom',
    bodyParts: [
      { name: 'head', dimensions: { width: 0.5, height: 0.5, depth: 0.5 }, position: { x: 0, y: 1.625, z: 0 }, textures: avatar.textures.head },
      { name: 'torso', dimensions: { width: 0.5, height: 0.75, depth: 0.25 }, position: { x: 0, y: 1, z: 0 }, textures: avatar.textures.torso },
      { name: 'rightArm', dimensions: { width: armWidth, height: 0.75, depth: 0.25 }, position: { x: -armOffset, y: 1, z: 0 }, pivot: { x: 0, y: 0.375, z: 0 }, textures: avatar.textures.rightArm },
      { name: 'leftArm', dimensions: { width: armWidth, height: 0.75, depth: 0.25 }, position: { x: armOffset, y: 1, z: 0 }, pivot: { x: 0, y: 0.375, z: 0 }, textures: avatar.textures.leftArm },
      { name: 'rightLeg', dimensions: { width: 0.25, height: 0.75, depth: 0.25 }, position: { x: -0.125, y: 0.25, z: 0 }, pivot: { x: 0, y: 0.375, z: 0 }, textures: avatar.textures.rightLeg },
      { name: 'leftLeg', dimensions: { width: 0.25, height: 0.75, depth: 0.25 }, position: { x: 0.125, y: 0.25, z: 0 }, pivot: { x: 0, y: 0.375, z: 0 }, textures: avatar.textures.leftLeg },
    ],
  }
}

export function setMatrixPixel(texture: TextureMatrix, x: number, y: number, key: string): TextureMatrix {
  const rows = [...texture.matrix]
  rows[y] = `${rows[y].slice(0, x)}${key}${rows[y].slice(x + 1)}`
  return { ...texture, matrix: rows }
}

export function saveAvatarCopy(avatar: EditableAvatar): EditableAvatar {
  const id = `custom-${Date.now()}`
  const copy: EditableAvatar = {
    ...cloneAvatar(avatar),
    id,
    name: `${avatar.name} - copie`,
    source: 'custom',
    overwriteLocked: false,
  }
  persistCustomAvatar(copy)
  localStorage.setItem(SELECTED_KEY, id)
  return copy
}

export function overwriteAvatar(avatar: EditableAvatar): EditableAvatar {
  if (avatar.overwriteLocked || avatar.source !== 'custom') {
    throw new Error('Impossible d écraser Steve ou Alex')
  }
  persistCustomAvatar(avatar)
  localStorage.setItem(SELECTED_KEY, avatar.id)
  return cloneAvatar(avatar)
}

export function getSelectedAvatarName(fallback: string | undefined): string {
  const selected = localStorage.getItem(SELECTED_KEY)
  if (selected && loadCustomAvatar(selected)) return `custom:${selected}`
  return fallback || 'alex'
}

function persistCustomAvatar(avatar: EditableAvatar) {
  const all = loadAllCustomAvatars()
  all[avatar.id] = cloneAvatar(avatar)
  localStorage.setItem(STORAGE_KEY, JSON.stringify(all))
}

function loadCustomAvatar(id: string): EditableAvatar | null {
  return loadAllCustomAvatars()[id] ?? null
}

function loadAllCustomAvatars(): Record<string, EditableAvatar> {
  const raw = localStorage.getItem(STORAGE_KEY)
  if (!raw) return {}
  try {
    return JSON.parse(raw) as Record<string, EditableAvatar>
  } catch {
    return {}
  }
}

function cloneTextures(source: Record<string, BodyPartFaces>): Record<AvatarPartName, BodyPartFaces> {
  return partNames.reduce((acc, part) => {
    acc[part] = faceNames.reduce((faces, face) => {
      faces[face] = cloneTexture(source[part][face])
      return faces
    }, {} as BodyPartFaces)
    return acc
  }, {} as Record<AvatarPartName, BodyPartFaces>)
}

function cloneAvatar(avatar: EditableAvatar): EditableAvatar {
  return {
    ...avatar,
    textures: cloneTextures(avatar.textures),
  }
}

function cloneTexture(texture: TextureMatrix): TextureMatrix {
  return {
    palette: JSON.parse(JSON.stringify(texture.palette)),
    width: texture.width,
    height: texture.height,
    matrix: [...texture.matrix],
  }
}
