import { NullEngine, Scene, Mesh, Vector3 } from '@babylonjs/core'
import type { UserAvatar } from '@/api'
import { buildCharacter } from '@/character-builder/character-builder'
import { createCharacterModelFromAvatar, createEditableAvatarFromApi } from '@/character-builder/avatar-editor'
import { generateCharacterPerspectiveSvg } from '@/character-builder/svg-export'

const PREVIEW_WIDTH = 180
const PREVIEW_HEIGHT = 280

export const generateProfileInfoAvatarMailPreview = (avatar: UserAvatar): string | undefined => {
  let engine: NullEngine | null = null
  let scene: Scene | null = null
  let mesh: Mesh | null = null

  try {
    engine = new NullEngine()
    scene = new Scene(engine)
    mesh = buildCharacter(scene, createCharacterModelFromAvatar(createEditableAvatarFromApi(avatar)), Vector3.Zero(), { physics: false })
    applyWalkingPose(mesh)
    const svg = generateCharacterPerspectiveSvg(mesh, {
      width: PREVIEW_WIDTH,
      height: PREVIEW_HEIGHT,
      padding: 14,
      background: 'rgba(3, 4, 8, 0)',
    })
    return `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`
  } catch {
    return undefined
  } finally {
    mesh = null
    scene?.dispose()
    engine?.dispose()
  }
}

const applyWalkingPose = (root: Mesh) => {
  const parts = root.getChildMeshes().filter((child): child is Mesh => child instanceof Mesh)
  const rightArm = parts.find((part) => part.name.endsWith('_rightArm'))
  const leftArm = parts.find((part) => part.name.endsWith('_leftArm'))
  const rightLeg = parts.find((part) => part.name.endsWith('_rightLeg'))
  const leftLeg = parts.find((part) => part.name.endsWith('_leftLeg'))

  if (rightArm) rightArm.rotation.x = -0.55
  if (leftArm) leftArm.rotation.x = 0.55
  if (rightLeg) rightLeg.rotation.x = 0.45
  if (leftLeg) leftLeg.rotation.x = -0.45
}
