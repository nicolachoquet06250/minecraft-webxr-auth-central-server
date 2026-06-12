import {
  Color3,
  DynamicTexture,
  Mesh,
  MultiMaterial,
  Scene,
  StandardMaterial,
  SubMesh,
  VertexData,
} from '@babylonjs/core'

export function createCuboidMesh(
  scene: Scene,
  name: string,
  width: number,
  height: number,
  depth: number,
  textures: {
    front: DynamicTexture
    back: DynamicTexture
    top: DynamicTexture
    bottom: DynamicTexture
    right: DynamicTexture
    left: DynamicTexture
  },
): Mesh {
  const positions: number[] = []
  const indices: number[] = []
  const normals: number[] = []
  const uvs: number[] = []

  const w = width / 2
  const h = height / 2
  const d = depth / 2

  positions.push(-w, -h, d, w, -h, d, w, h, d, -w, h, d)
  uvs.push(1, 0, 0, 0, 0, 1, 1, 1)
  indices.push(0, 2, 1, 0, 3, 2)

  positions.push(w, -h, -d, -w, -h, -d, -w, h, -d, w, h, -d)
  uvs.push(1, 0, 0, 0, 0, 1, 1, 1)
  indices.push(4, 6, 5, 4, 7, 6)

  positions.push(-w, h, -d, w, h, -d, w, h, d, -w, h, d)
  uvs.push(1, 1, 0, 1, 0, 0, 1, 0)
  indices.push(8, 10, 9, 8, 11, 10)

  positions.push(-w, -h, d, w, -h, d, w, -h, -d, -w, -h, -d)
  uvs.push(1, 1, 0, 1, 0, 0, 1, 0)
  indices.push(12, 13, 14, 12, 14, 15)

  positions.push(w, -h, d, w, -h, -d, w, h, -d, w, h, d)
  uvs.push(1, 0, 0, 0, 0, 1, 1, 1)
  indices.push(16, 18, 17, 16, 19, 18)

  positions.push(-w, -h, -d, -w, -h, d, -w, h, d, -w, h, -d)
  uvs.push(1, 0, 0, 0, 0, 1, 1, 1)
  indices.push(20, 22, 21, 20, 23, 22)

  const mesh = new Mesh(name, scene)
  const vertexData = new VertexData()
  vertexData.positions = positions
  vertexData.indices = indices
  vertexData.uvs = uvs

  VertexData.ComputeNormals(positions, indices, normals)
  vertexData.normals = normals
  vertexData.applyToMesh(mesh)

  const multiMat = new MultiMaterial(name + '_multiMat', scene)
  const faceTextures = [
    textures.front,
    textures.back,
    textures.top,
    textures.bottom,
    textures.right,
    textures.left,
  ]

  for (let index = 0; index < 6; index++) {
    const mat = new StandardMaterial(`${name}_mat_${index}`, scene)
    mat.diffuseTexture = faceTextures[index]
    mat.specularColor = new Color3(0, 0, 0)
    mat.backFaceCulling = false
    multiMat.subMaterials.push(mat)
  }

  mesh.material = multiMat
  mesh.subMeshes = []
  for (let index = 0; index < 6; index++) {
    new SubMesh(index, 0, 24, index * 6, 6, mesh)
  }

  return mesh
}
