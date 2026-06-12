import type { AvatarTextureData } from '@/api'
import type { BodyPartFaces, CharacterModel, TextureMatrix } from './types'
import { alexModelTextures } from './alex-color-matrices'
import { steveModelTextures } from './steve-color-matrices'

export type AvatarPartName = 'head' | 'torso' | 'rightArm' | 'leftArm' | 'rightLeg' | 'leftLeg'
export type AvatarFaceName = keyof BodyPartFaces
export type AvatarSource = 'steve' | 'alex' | 'custom'
type MutableBodyPartFaces = { -readonly [K in keyof BodyPartFaces]: TextureMatrix }
type RgbaColor = [number, number, number, number]

export type EditableAvatar = {
  id: string
  name: string
  source: AvatarSource
  baseKind: 'steve' | 'alex' | 'custom'
  overwriteLocked: boolean
  textures: Record<AvatarPartName, BodyPartFaces>
}

const partNames: AvatarPartName[] = ['head', 'torso', 'rightArm', 'leftArm', 'rightLeg', 'leftLeg']
const faceNames: AvatarFaceName[] = ['front', 'back', 'top', 'bottom', 'left', 'right']
const keyPool = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'

export const avatarPartNames = partNames
export const avatarFaceNames = faceNames

export function getSelectedAvatarName(fallback: string | undefined): string {
  return fallback?.trim() || 'alex'
}

export function createEditableAvatar(avatarName: string | undefined): EditableAvatar {
  const normalized = avatarName?.trim() || 'alex'

  if (normalized === 'steve') {
    return {
      id: 'steve',
      name: 'Steve',
      source: 'steve',
      baseKind: 'steve',
      overwriteLocked: true,
      textures: cloneTextures(steveModelTextures),
    }
  }

  return {
    id: 'alex',
    name: 'Alex',
    source: 'alex',
    baseKind: 'alex',
    overwriteLocked: true,
    textures: cloneTextures(alexModelTextures),
  }
}

export function createEditableAvatarFromApi(input: {
  id: string
  name: string
  base_kind: string
  texture_data: AvatarTextureData
}): EditableAvatar {
  const baseKind = normalizeBaseKind(input.base_kind)

  return {
    id: input.id,
    name: input.name,
    source: 'custom',
    baseKind,
    overwriteLocked: false,
    textures: textureDataToTextures(input.texture_data),
  }
}

export function createCharacterModelFromAvatar(avatar: EditableAvatar): CharacterModel {
  const isSteve = avatar.baseKind === 'steve'
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

export function getTexturePixelColor(texture: TextureMatrix, x: number, y: number): RgbaColor {
  const key = texture.matrix[y]?.[x]
  const color = key ? texture.palette[key] : undefined
  return color ? [color[0], color[1], color[2], color[3]] : [0, 0, 0, 0]
}

export function paintAvatarPixel(
  avatar: EditableAvatar,
  part: AvatarPartName,
  face: AvatarFaceName,
  x: number,
  y: number,
  color: RgbaColor,
): EditableAvatar {
  const next = cloneAvatar(avatar)
  const texture = next.textures[part][face]
  const palette = { ...texture.palette }
  const key = findOrCreatePaletteKey(palette, color)
  const rows = [...texture.matrix]
  rows[y] = `${rows[y].slice(0, x)}${key}${rows[y].slice(x + 1)}`

  ;(next.textures[part] as MutableBodyPartFaces)[face] = {
    palette,
    width: texture.width,
    height: texture.height,
    matrix: rows,
  }

  return next
}

export function texturesToTextureData(avatar: EditableAvatar): AvatarTextureData {
  const globalPalette: Record<string, readonly [number, number, number, number]> = {}
  const reverse = new Map<string, string>()
  let keyIndex = 0

  const getGlobalKey = (rgba: readonly [number, number, number, number]) => {
    const signature = rgba.join(',')
    const existing = reverse.get(signature)
    if (existing) return existing

    const key = keyPool[keyIndex]
    if (!key) {
      throw new Error('Palette globale trop grande pour être sérialisée')
    }

    keyIndex += 1
    reverse.set(signature, key)
    globalPalette[key] = rgba
    return key
  }

  const parts = Object.fromEntries(
    partNames.map((partName) => {
      const partFaces = Object.fromEntries(
        faceNames.map((faceName) => {
          const texture = avatar.textures[partName][faceName]
          const matrix = texture.matrix.map((row) =>
            row.split('').map((cellKey) => getGlobalKey(texture.palette[cellKey])).join(''),
          )

          return [
            faceName,
            {
              width: texture.width,
              height: texture.height,
              matrix,
            },
          ]
        }),
      )

      return [partName, partFaces]
    }),
  ) as AvatarTextureData['parts']

  return {
    version: 1,
    palette: globalPalette,
    parts,
  }
}

export function rgbaToHex([r, g, b]: RgbaColor) {
  return `#${toHex(r)}${toHex(g)}${toHex(b)}`
}

export function hexToRgba(hex: string, alpha = 1): RgbaColor {
  const cleaned = hex.replace('#', '')
  const normalized = cleaned.length === 3
    ? cleaned.split('').map((c) => c + c).join('')
    : cleaned

  const r = parseInt(normalized.slice(0, 2), 16) / 255
  const g = parseInt(normalized.slice(2, 4), 16) / 255
  const b = parseInt(normalized.slice(4, 6), 16) / 255

  return [r, g, b, alpha]
}

export function colorToCss([r, g, b, a]: RgbaColor) {
  return `rgba(${Math.round(r * 255)}, ${Math.round(g * 255)}, ${Math.round(b * 255)}, ${a})`
}

function normalizeBaseKind(value: string): 'steve' | 'alex' | 'custom' {
  if (value === 'steve') return 'steve'
  if (value === 'alex') return 'alex'
  return 'custom'
}

function textureDataToTextures(data: AvatarTextureData): Record<AvatarPartName, BodyPartFaces> {
  const palette = data.palette

  return Object.fromEntries(
    partNames.map((partName) => {
      const faces = faceNames.reduce((acc, faceName) => {
        const face = data.parts[partName][faceName]
        acc[faceName] = {
          palette: { ...palette },
          width: face.width,
          height: face.height,
          matrix: [...face.matrix],
        }
        return acc
      }, {} as MutableBodyPartFaces)

      return [partName, faces as BodyPartFaces]
    }),
  ) as Record<AvatarPartName, BodyPartFaces>
}

function findOrCreatePaletteKey(
  palette: Record<string, readonly [number, number, number, number]>,
  rgba: RgbaColor,
) {
  const signature = rgba.join(',')

  for (const [key, value] of Object.entries(palette)) {
    if (value.join(',') === signature) {
      return key
    }
  }

  const used = new Set(Object.keys(palette))
  const nextKey = keyPool.split('').find((key) => !used.has(key))

  if (!nextKey) {
    throw new Error('Palette de texture saturée')
  }

  palette[nextKey] = rgba
  return nextKey
}

function cloneTextures(source: Record<string, BodyPartFaces>): Record<AvatarPartName, BodyPartFaces> {
  return partNames.reduce((acc, part) => {
    const faces = faceNames.reduce((facesAcc, face) => {
      facesAcc[face] = cloneTexture(source[part][face])
      return facesAcc
    }, {} as MutableBodyPartFaces)
    acc[part] = faces
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

function toHex(value: number) {
  return Math.round(value * 255).toString(16).padStart(2, '0')
}
