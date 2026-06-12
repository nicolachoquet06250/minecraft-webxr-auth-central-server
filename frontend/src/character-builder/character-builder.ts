import { Mesh, Ray, Scene, Vector3 } from '@babylonjs/core'
import type { CharacterModel } from './types'
import { createTextureFromMatrix } from './texture-builder'
import { createCuboidMesh } from './mesh-builder'
import {
  CharacterPhysicsController,
  type CharacterPhysicsOptions,
} from './avatar-physics'

export type BuildCharacterOptions = {
  physics?: false | CharacterPhysicsOptions
}

const characterPhysicsControllers = new WeakMap<Mesh, CharacterPhysicsController>()
const CHARACTER_MESH_METADATA_KEY = 'isCharacterMesh'

export function buildCharacter(
  scene: Scene,
  model: CharacterModel,
  position: Vector3,
  options: BuildCharacterOptions = {},
): Mesh {
  const rootMesh = new Mesh(model.name, scene)
  rootMesh.position = position
  rootMesh.metadata = {
    ...(rootMesh.metadata ?? {}),
    [CHARACTER_MESH_METADATA_KEY]: true,
  }

  const meshMap = new Map<string, Mesh>()
  meshMap.set(model.name, rootMesh)

  for (const bodyPart of model.bodyParts) {
    const textures = {
      front: createTextureFromMatrix(scene, `${model.name}_${bodyPart.name}_front`, bodyPart.textures.front),
      back: createTextureFromMatrix(scene, `${model.name}_${bodyPart.name}_back`, bodyPart.textures.back),
      top: createTextureFromMatrix(scene, `${model.name}_${bodyPart.name}_top`, bodyPart.textures.top),
      bottom: createTextureFromMatrix(scene, `${model.name}_${bodyPart.name}_bottom`, bodyPart.textures.bottom),
      right: createTextureFromMatrix(scene, `${model.name}_${bodyPart.name}_right`, bodyPart.textures.right),
      left: createTextureFromMatrix(scene, `${model.name}_${bodyPart.name}_left`, bodyPart.textures.left),
    }

    const partMesh = createCuboidMesh(
      scene,
      `${model.name}_${bodyPart.name}`,
      bodyPart.dimensions.width,
      bodyPart.dimensions.height,
      bodyPart.dimensions.depth,
      textures,
    )
    partMesh.metadata = {
      ...(partMesh.metadata ?? {}),
      [CHARACTER_MESH_METADATA_KEY]: true,
    }

    if (bodyPart.pivot) {
      partMesh.setPivotPoint(new Vector3(bodyPart.pivot.x, bodyPart.pivot.y, bodyPart.pivot.z))
    }

    partMesh.position = new Vector3(bodyPart.position.x, bodyPart.position.y, bodyPart.position.z)

    const parent = bodyPart.parent ? meshMap.get(bodyPart.parent) : rootMesh
    partMesh.parent = parent ?? rootMesh

    meshMap.set(bodyPart.name, partMesh)
  }

  if (options.physics !== false) {
    const physicsOptions = options.physics ?? {}
    const controller = new CharacterPhysicsController(rootMesh, physicsOptions)
    characterPhysicsControllers.set(rootMesh, controller)
  }

  return rootMesh
}

export function getCharacterPhysics(characterMesh: Mesh): CharacterPhysicsController | null {
  return characterPhysicsControllers.get(characterMesh) ?? null
}

export function getCharacterHitDistance(scene: Scene, ray: Ray, maxDistance: number): number | null {
  const pick = scene.pickWithRay(
    ray,
    (mesh) => Boolean(mesh.metadata?.[CHARACTER_MESH_METADATA_KEY]),
    true,
  )

  if (!pick?.hit || pick.distance === undefined) {
    return null
  }

  return pick.distance <= maxDistance ? pick.distance : null
}

export function getBodyPart(characterMesh: Mesh, partName: string): Mesh | undefined {
  return characterMesh.getChildMeshes().find((mesh) => mesh.name.endsWith(`_${partName}`)) as Mesh | undefined
}

export function getAllBodyParts(characterMesh: Mesh): Map<string, Mesh> {
  const parts = new Map<string, Mesh>()
  const namePrefix = characterMesh.name + '_'

  for (const child of characterMesh.getChildMeshes()) {
    if (child.name.startsWith(namePrefix)) {
      const partName = child.name.substring(namePrefix.length)
      parts.set(partName, child as Mesh)
    }
  }

  return parts
}
