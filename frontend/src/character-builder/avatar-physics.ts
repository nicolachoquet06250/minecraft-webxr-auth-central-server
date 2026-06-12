import { Mesh, Vector3 } from '@babylonjs/core'

export type CharacterPhysicsOptions = {
  collisionRadius?: number
  collisionHeight?: number
  gravityEnabled?: boolean
  collisionsEnabled?: boolean
  externalControl?: boolean
}

export type CharacterPhysicsUpdateParams = {
  worldChunks?: unknown
  sizeX?: number
  sizeY?: number
  sizeZ?: number
  deltaTime: number
}

export class CharacterPhysicsController {
  readonly mesh: Mesh
  readonly collisionRadius: number
  readonly collisionHeight: number

  private velocity: Vector3 = Vector3.Zero()
  private grounded = false
  private gravityEnabled: boolean
  private collisionsEnabled: boolean
  private externalControl: boolean

  constructor(mesh: Mesh, options: CharacterPhysicsOptions = {}) {
    this.mesh = mesh
    this.collisionRadius = options.collisionRadius ?? 0.35
    this.collisionHeight = options.collisionHeight ?? 1.8
    this.gravityEnabled = options.gravityEnabled ?? false
    this.collisionsEnabled = options.collisionsEnabled ?? false
    this.externalControl = options.externalControl ?? true
  }

  setExternalControl(enabled: boolean): void {
    this.externalControl = enabled
    if (enabled) {
      this.velocity.setAll(0)
      this.grounded = false
    }
  }

  setGravityEnabled(enabled: boolean): void {
    this.gravityEnabled = enabled
  }

  setCollisionsEnabled(enabled: boolean): void {
    this.collisionsEnabled = enabled
  }

  setVelocity(velocity: Vector3): void {
    this.velocity.copyFrom(velocity)
  }

  isGrounded(): boolean {
    return this.grounded
  }

  teleport(position: Vector3): void {
    this.mesh.position.copyFrom(position)
    this.velocity.setAll(0)
    this.grounded = false
  }

  update(params: CharacterPhysicsUpdateParams): void {
    if (this.externalControl || !this.gravityEnabled) {
      return
    }

    const deltaTime = params.deltaTime
    this.mesh.position.addInPlace(this.velocity.scale(deltaTime))

    if (!this.collisionsEnabled && this.mesh.position.y <= 0) {
      this.mesh.position.y = 0
      this.velocity.y = 0
      this.grounded = true
    }
  }
}

export function resolvePlayerCharacterCollision(): boolean {
  return false
}

export function syncCameraToPlayerPosition(_player: unknown, _cameraPosition: Vector3): void {}
