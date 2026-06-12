import { Animation, AnimationGroup as BabylonAnimationGroup, Mesh, Scene, Vector3 } from '@babylonjs/core'
import type { CharacterAnimations } from './types'
import { getAllBodyParts } from './character-builder'

export type Axis = 'x' | 'y' | 'z'

export type BodyPartVectorUpdate = {
  x?: number
  y?: number
  z?: number
}

export class CharacterAnimator {
  private scene: Scene
  private bodyParts: Map<string, Mesh>
  private animationGroups: Map<string, BabylonAnimationGroup> = new Map()
  private currentAnimation: BabylonAnimationGroup | null = null

  constructor(characterMesh: Mesh, scene: Scene) {
    this.scene = scene
    this.bodyParts = getAllBodyParts(characterMesh)
  }

  getBodyPartNames(): string[] {
    return [...this.bodyParts.keys()]
  }

  getBodyPart(partName: string): Mesh | null {
    return this.bodyParts.get(partName) ?? null
  }

  loadAnimations(animations: CharacterAnimations): void {
    for (const [groupName, animGroup] of Object.entries(animations)) {
      const babylonAnimGroup = new BabylonAnimationGroup(groupName, this.scene)
      for (const animDef of animGroup.animations) {
        const targetPart = this.bodyParts.get(animDef.targetPart)
        if (!targetPart) {
          console.warn(`Animation target "${animDef.targetPart}" not found in character body parts`)
          continue
        }

        const animation = new Animation(
          `${groupName}_${animDef.targetPart}_${animDef.property}`,
          animDef.property,
          animGroup.fps,
          Animation.ANIMATIONTYPE_FLOAT,
          animGroup.loop ? Animation.ANIMATIONLOOPMODE_CYCLE : Animation.ANIMATIONLOOPMODE_CONSTANT,
        )

        animation.setKeys(animDef.keyframes.map((keyframe) => ({ frame: keyframe.frame, value: keyframe.value })))
        targetPart.animations.push(animation)
        babylonAnimGroup.addTargetedAnimation(animation, targetPart)
      }

      this.animationGroups.set(groupName, babylonAnimGroup)
    }
  }

  play(animationName: string, loop?: boolean, speed: number = 1.0): void {
    const animGroup = this.animationGroups.get(animationName)

    if (!animGroup) {
      console.warn(`Animation "${animationName}" not found`)
      return
    }

    if (this.currentAnimation && this.currentAnimation !== animGroup) {
      this.currentAnimation.stop()
      this.resetPose()
    }

    animGroup.play(loop ?? true)
    animGroup.speedRatio = speed
    this.currentAnimation = animGroup
  }

  stop(): void {
    if (this.currentAnimation) {
      this.currentAnimation.stop()
      this.currentAnimation = null
      this.resetPose()
    }
  }

  setPartRotation(partName: string, update: BodyPartVectorUpdate): boolean {
    const part = this.bodyParts.get(partName)
    if (!part) {
      console.warn(`Body part "${partName}" not found`)
      return false
    }

    if (update.x !== undefined) part.rotation.x = update.x
    if (update.y !== undefined) part.rotation.y = update.y
    if (update.z !== undefined) part.rotation.z = update.z
    return true
  }

  setPartPosition(partName: string, update: BodyPartVectorUpdate): boolean {
    const part = this.bodyParts.get(partName)
    if (!part) {
      console.warn(`Body part "${partName}" not found`)
      return false
    }

    if (update.x !== undefined) part.position.x = update.x
    if (update.y !== undefined) part.position.y = update.y
    if (update.z !== undefined) part.position.z = update.z
    return true
  }

  setPartRotationAxis(partName: string, axis: Axis, value: number): boolean {
    const part = this.bodyParts.get(partName)
    if (!part) {
      console.warn(`Body part "${partName}" not found`)
      return false
    }

    part.rotation[axis] = value
    return true
  }

  setHeadYaw(yaw: number, headPartName: string = 'head'): boolean {
    const head = this.bodyParts.get(headPartName)
    if (!head) {
      console.warn(`Head part "${headPartName}" not found`)
      return false
    }

    head.rotation.x = 0
    head.rotation.y = yaw
    head.rotation.z = 0
    return true
  }

  private resetPose(): void {
    for (const part of this.bodyParts.values()) {
      part.rotation = Vector3.Zero()
    }
  }

  getCurrentAnimation(): string | null {
    if (!this.currentAnimation) return null

    for (const [name, group] of this.animationGroups.entries()) {
      if (group === this.currentAnimation) return name
    }

    return null
  }

  dispose(): void {
    this.animationGroups.forEach((group) => group.dispose())
    this.animationGroups.clear()
  }
}
