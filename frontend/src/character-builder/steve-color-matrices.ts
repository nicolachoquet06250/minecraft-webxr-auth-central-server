import type { TextureMatrix } from './types'

export const stevePalette = {
  A: [0.12, 0.07, 0.03, 1], B: [0.19, 0.10, 0.04, 1], C: [0.27, 0.15, 0.06, 1], D: [0.34, 0.20, 0.09, 1],
  E: [0.46, 0.26, 0.16, 1], F: [0.60, 0.36, 0.22, 1], G: [0.72, 0.46, 0.30, 1], H: [0.82, 0.56, 0.38, 1], I: [0.90, 0.66, 0.46, 1],
  J: [0.96, 0.94, 0.90, 1], K: [0.19, 0.12, 0.68, 1], L: [0.28, 0.12, 0.07, 1], M: [0.45, 0.20, 0.12, 1],
  N: [0.00, 0.50, 0.58, 1], O: [0.00, 0.62, 0.68, 1], P: [0.05, 0.72, 0.76, 1], Q: [0.12, 0.78, 0.80, 1], R: [0.00, 0.42, 0.48, 1],
  S: [0.12, 0.12, 0.45, 1], T: [0.16, 0.16, 0.58, 1], U: [0.20, 0.20, 0.68, 1], V: [0.10, 0.10, 0.35, 1],
  W: [0.18, 0.18, 0.18, 1], X: [0.28, 0.28, 0.28, 1], Y: [0.38, 0.38, 0.36, 1], Z: [0.08, 0.08, 0.08, 1],
} as const

const tx = (width: number, height: number, matrix: readonly string[]): TextureMatrix => ({ palette: stevePalette, width, height, matrix })

export const steveModelTextures = {
  head: {
    top: tx(8, 8, ['ABBCBBAA', 'BCCDCBBA', 'BCDDDCBA', 'ACDCCDCA', 'BCCDDCBA', 'ABCDCBBA', 'AACBCBAA', 'AABBAAAA']),
    bottom: tx(8, 8, ['FGGGHGFF', 'GGHHIHGF', 'GHHIHHGG', 'FHHHHHGF', 'GGHHHGFF', 'FGGHHGGF', 'FFGGGFFF', 'EFFFGFFE']),
    front: tx(8, 8, ['BBBBBBBB', 'BCDDCCBB', 'BGGHHGGB', 'GJKKJJHG', 'GGHLLHGG', 'GGMMMMGG', 'FGGGGGGF', 'FGHHGGFF']),
    back: tx(8, 8, ['ABBCCBBA', 'BBCDDCBA', 'BCDCDDCA', 'ACDDDCBA', 'BCDCCDBA', 'BBCDCBBA', 'ABBCCBAA', 'AABBBBAA']),
    left: tx(8, 8, ['ABBBCCBA', 'BBCDCCBA', 'BCDDDCBA', 'BGGGGGGB', 'GGGHHHGF', 'GGHHGGGF', 'FGGGGGFF', 'FFGGGFFF']),
    right: tx(8, 8, ['ABCCBBBA', 'ABCCDDBB', 'ABCDDDCB', 'BGGGGGGB', 'FGHHHGGG', 'FGGGHHGG', 'FFGGGGGF', 'FFFGGGFF']),
  },
  torso: {
    front: tx(8, 12, ['POOQPOOP', 'OPPPQOPO', 'POOQPOOP', 'OOPPRPOO', 'OOPRRPOO', 'PONNNNOP', 'OPNNNNPO', 'OONNNNOO', 'TTTTTTTT', 'TUTTUUTT', 'TTUUUTTT', 'TUTTTUTT']),
    back: tx(8, 12, ['OPPOQOOP', 'POOOPQPO', 'OPOQPOOP', 'OPPRPOOP', 'OOPRROPO', 'OONNNNOO', 'PNNNNNOP', 'OONNNNOO', 'TTTTTTTT', 'TTUUTUTT', 'TUTTTUTT', 'TTUTTUTT']),
    left: tx(4, 12, ['OPOP', 'PPQO', 'OPPO', 'OORO', 'OPRO', 'ONNO', 'PNNO', 'ONNO', 'TTTT', 'TUTT', 'TTUT', 'TUTT']),
    right: tx(4, 12, ['POPO', 'OQPP', 'OPPO', 'OROO', 'ORPO', 'ONNO', 'ONNP', 'ONNO', 'TTTT', 'TTUT', 'TUTT', 'TTUT']),
    top: tx(8, 4, ['OPPOQOOP', 'POOQPOOP', 'OPPPQOPO', 'POOPOOQP']),
    bottom: tx(8, 4, ['STTUTTTS', 'TTUUUTTT', 'TUTTTUTT', 'STTTTUTS']),
  },
  rightArm: {
    front: tx(4, 12, ['OQPO', 'OPPO', 'OPRO', 'FHHG', 'GGHG', 'FGGF', 'GGHF', 'FGGF', 'GGHG', 'FHHG', 'FGGF', 'EFFG']),
    back: tx(4, 12, ['OPPO', 'POQO', 'OPRO', 'GHGF', 'GGFG', 'GHHG', 'FGGF', 'GGHF', 'FGGF', 'GHHG', 'FGGF', 'FEFG']),
    left: tx(4, 12, ['OOPO', 'PQOP', 'OPRO', 'GGHF', 'FGGF', 'GGHG', 'FHHG', 'GGFG', 'FGGF', 'GGHF', 'FGGF', 'EFFG']),
    right: tx(4, 12, ['POOO', 'POQP', 'ORPO', 'FHHG', 'GGFG', 'FGGF', 'GHHG', 'FGGF', 'GGHF', 'FHHG', 'GFGF', 'FEFG']),
    top: tx(4, 4, ['OPQO', 'POOP', 'OPPO', 'ROOR']),
    bottom: tx(4, 4, ['FGGF', 'GHHG', 'FGGF', 'EFFG']),
  },
  leftArm: {
    front: tx(4, 12, ['OPOQ', 'OPPO', 'ORPO', 'GHHF', 'GHGG', 'FGGF', 'FHGG', 'FGGF', 'GHGG', 'GHHF', 'FGGF', 'GFFE']),
    back: tx(4, 12, ['OPPO', 'OQOP', 'ORPO', 'FHGG', 'GFGG', 'GHHG', 'FGGF', 'FHGG', 'FGGF', 'GHHG', 'FGGF', 'GFEF']),
    left: tx(4, 12, ['OOOP', 'PQOP', 'ORPO', 'GHHF', 'GFGG', 'FGGF', 'GHHG', 'FGGF', 'FHGG', 'GHHF', 'FGFG', 'GFEF']),
    right: tx(4, 12, ['OPOO', 'POQP', 'OPRO', 'FHGG', 'FGGF', 'GHGG', 'GHHF', 'GGFG', 'FGGF', 'FHGG', 'FGGF', 'GFFE']),
    top: tx(4, 4, ['OQPO', 'POOP', 'OPPO', 'ROOR']),
    bottom: tx(4, 4, ['GFGF', 'GHHG', 'FGGF', 'GFFE']),
  },
  rightLeg: {
    front: tx(4, 12, ['TUTT', 'TTUU', 'TUTT', 'UTTT', 'TTUT', 'TUTT', 'TTTU', 'TUTT', 'XXXX', 'WXXW', 'WWXW', 'ZZZZ']),
    back: tx(4, 12, ['TTUT', 'TUTT', 'TTUU', 'TUTT', 'UTTT', 'TTUT', 'TUTT', 'TTTU', 'XXXX', 'WXWW', 'WXXW', 'ZZZZ']),
    left: tx(4, 12, ['TTTU', 'TUTT', 'TTUT', 'TUTT', 'TTUU', 'TUTT', 'UTTT', 'TTUT', 'XXXX', 'WWXW', 'WXXW', 'ZZZZ']),
    right: tx(4, 12, ['UTTT', 'TTUT', 'TUTT', 'TTUU', 'TUTT', 'UTTT', 'TTUT', 'TUTT', 'XXXX', 'WXXW', 'WXWW', 'ZZZZ']),
    top: tx(4, 4, ['STTS', 'TTUT', 'TUTT', 'STTS']),
    bottom: tx(4, 4, ['WWWW', 'WXXW', 'WXWW', 'ZZZZ']),
  },
  leftLeg: {
    front: tx(4, 12, ['TTUT', 'UUTT', 'TTUT', 'TTTU', 'TUTT', 'TTUT', 'UTTT', 'TTUT', 'XXXX', 'WXXW', 'WXWW', 'ZZZZ']),
    back: tx(4, 12, ['TUTT', 'TTUT', 'UUTT', 'TTUT', 'TTTU', 'TUTT', 'TTUT', 'UTTT', 'XXXX', 'WWXW', 'WXXW', 'ZZZZ']),
    left: tx(4, 12, ['TTTU', 'TUTT', 'TTUT', 'UUTT', 'TTUT', 'TTTU', 'TUTT', 'TTUT', 'XXXX', 'WXWW', 'WXXW', 'ZZZZ']),
    right: tx(4, 12, ['UTTT', 'TTTU', 'TUTT', 'TTUT', 'UUTT', 'TTUT', 'TTTU', 'TUTT', 'XXXX', 'WXXW', 'WWXW', 'ZZZZ']),
    top: tx(4, 4, ['STTS', 'TUTT', 'TTUT', 'STTS']),
    bottom: tx(4, 4, ['WWWW', 'WXWW', 'WXXW', 'ZZZZ']),
  },
} as const
