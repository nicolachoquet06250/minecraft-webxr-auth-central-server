import type { TextureMatrix } from './types'

export const alexPalette = {
  A: [0.74, 0.33, 0.05, 1], B: [0.86, 0.43, 0.08, 1], C: [0.95, 0.55, 0.12, 1], D: [1.0, 0.66, 0.20, 1],
  E: [0.82, 0.55, 0.34, 1], F: [0.93, 0.70, 0.47, 1], G: [1.0, 0.80, 0.55, 1], H: [1.0, 0.88, 0.68, 1], I: [0.72, 0.43, 0.26, 1],
  J: [0.96, 0.94, 0.88, 1], K: [0.10, 0.35, 0.16, 1], L: [0.74, 0.42, 0.34, 1],
  M: [0.35, 0.54, 0.31, 1], N: [0.43, 0.63, 0.38, 1], O: [0.53, 0.72, 0.47, 1], P: [0.25, 0.43, 0.24, 1], Q: [0.13, 0.29, 0.16, 1],
  R: [0.92, 0.47, 0.12, 1], S: [1.0, 0.61, 0.22, 1],
  T: [0.28, 0.17, 0.10, 1], U: [0.36, 0.23, 0.13, 1], V: [0.45, 0.30, 0.18, 1], W: [0.22, 0.13, 0.08, 1],
  X: [0.22, 0.23, 0.23, 1], Y: [0.34, 0.35, 0.35, 1], Z: [0.14, 0.14, 0.14, 1],
} as const

const tx = (width: number, height: number, matrix: readonly string[]): TextureMatrix => ({ palette: alexPalette, width, height, matrix })

export const alexModelTextures = {
  head: {
    top: tx(8, 8, ['BBBBBBBB', 'BCCDDCBB', 'BCDDDDCB', 'BCDDCDCB', 'BCDCDDDB', 'BCCDDCCB', 'BBCDCBBB', 'BBBBBBBB']),
    bottom: tx(8, 8, ['BBBBBBBB', 'BCCDDCCB', 'BCDCCDDB', 'BCDCDDDB', 'BCCDDCCB', 'BCDCCDDB', 'BBCDCCBB', 'BBBBBBBB']),
    front: tx(8, 8, ['BBBBBBBB', 'BCCDDCBB', 'BCCHHCCB', 'BGGHHGGB', 'GJKHHKJG', 'GGGHGGGG', 'GGGLLGGG', 'GGGGGGGG']),
    back: tx(8, 8, ['BBBBBBBB', 'BCCDDCCB', 'BCDDDCCB', 'BCDCCDCB', 'BCCDDCCB', 'BCDDDDCB', 'BBCDCCBB', 'BBBBBBBB']),
    left: tx(8, 8, ['BBBBBBBB', 'BCCDDCBB', 'BCDDDCCB', 'BCDDDCBB', 'BCCDGGGG', 'BCDGGGGG', 'BCGGGGGG', 'BBGGGGGG']),
    right: tx(8, 8, ['BBBBBBBB', 'BBCDDCCB', 'BCCDDDDB', 'BBDDDCCB', 'GGGGDCCB', 'GGGGGDDB', 'GGGGGGCB', 'GGGGGGBB']),
  },
  torso: {
    front: tx(8, 12, ['NNNNNNNN', 'NOOONNNN', 'NOOONRSN', 'NOONRSNN', 'NONRSNNN', 'NNRSNNNN', 'NNRNNNNN', 'PNNNNNNP', 'QQQQQQQQ', 'MTTTTTTM', 'TUVUUVUT', 'TUUTTUUT']),
    back: tx(8, 12, ['NNNNNNNN', 'NOOONNON', 'NONOOONN', 'NNOONNON', 'NNONNOON', 'NNNOONNN', 'PNNNNNNP', 'PNNNNNNP', 'QQQQQQQQ', 'MTTTTTTM', 'TUUVUUTT', 'TTUUTTUT']),
    left: tx(4, 12, ['NNNN', 'NOON', 'NNON', 'NNON', 'NNNN', 'NNNN', 'PNNP', 'PNNP', 'QQQQ', 'MTTM', 'TUUT', 'TTUT']),
    right: tx(4, 12, ['NNNN', 'NOON', 'NONN', 'NONN', 'NNNN', 'NNNN', 'PNNP', 'PNNP', 'QQQQ', 'MTTM', 'TUUT', 'TUTT']),
    top: tx(8, 4, ['HHHHHHHH', 'GGGGGGGG', 'NNNNNNNN', 'NOONNOON']),
    bottom: tx(8, 4, ['TTTTTTTT', 'TUUVUUTT', 'TTUUTTUT', 'WWTTTTWW']),
  },
  rightArm: {
    front: tx(3, 12, ['HHH', 'GGG', 'NNN', 'NOO', 'NNN', 'NNN', 'NNN', 'PNP', 'GGG', 'HHG', 'GGG', 'YYZ']),
    back: tx(3, 12, ['HHH', 'GGG', 'NNN', 'ONO', 'NNN', 'NNN', 'NNN', 'PNP', 'GGG', 'GHH', 'GGG', 'ZYY']),
    left: tx(4, 12, ['HHHH', 'GGGG', 'NNNN', 'NOON', 'NNNN', 'NNNN', 'NNNN', 'PNNP', 'GGGG', 'HHGG', 'GGGG', 'YYZZ']),
    right: tx(4, 12, ['HHHH', 'GGGG', 'NNNN', 'NOON', 'NNNN', 'NNNN', 'NNNN', 'PNNP', 'GGGG', 'GGHH', 'GGGG', 'ZZYY']),
    top: tx(3, 4, ['HHH', 'GGG', 'GGG', 'HHG']),
    bottom: tx(3, 4, ['GGG', 'HHG', 'GGG', 'YYZ']),
  },
  leftArm: {
    front: tx(3, 12, ['HHH', 'GGG', 'NNN', 'OON', 'NNN', 'NNN', 'NNN', 'PNP', 'GGG', 'GHH', 'GGG', 'ZYY']),
    back: tx(3, 12, ['HHH', 'GGG', 'NNN', 'ONO', 'NNN', 'NNN', 'NNN', 'PNP', 'GGG', 'HHG', 'GGG', 'YYZ']),
    left: tx(4, 12, ['HHHH', 'GGGG', 'NNNN', 'NOON', 'NNNN', 'NNNN', 'NNNN', 'PNNP', 'GGGG', 'GGHH', 'GGGG', 'ZZYY']),
    right: tx(4, 12, ['HHHH', 'GGGG', 'NNNN', 'NOON', 'NNNN', 'NNNN', 'NNNN', 'PNNP', 'GGGG', 'HHGG', 'GGGG', 'YYZZ']),
    top: tx(3, 4, ['HHH', 'GGG', 'GGG', 'GHH']),
    bottom: tx(3, 4, ['GGG', 'GHH', 'GGG', 'ZYY']),
  },
  rightLeg: {
    front: tx(4, 12, ['TUVT', 'TUUT', 'TUTT', 'UTTT', 'TTUT', 'TUTT', 'TTTU', 'TUTT', 'XXXX', 'XYYX', 'YXXY', 'ZZZZ']),
    back: tx(4, 12, ['TTUT', 'TUTT', 'TUUT', 'TUTT', 'UTTT', 'TTUT', 'TUTT', 'TTTU', 'XXXX', 'XYXX', 'XYYX', 'ZZZZ']),
    left: tx(4, 12, ['TTTU', 'TUTT', 'TTUT', 'TUTT', 'TUUT', 'TUTT', 'UTTT', 'TTUT', 'XXXX', 'XXYX', 'XYYX', 'ZZZZ']),
    right: tx(4, 12, ['UTTT', 'TTUT', 'TUTT', 'TUUT', 'TUTT', 'UTTT', 'TTUT', 'TUTT', 'XXXX', 'XYYX', 'XYXX', 'ZZZZ']),
    top: tx(4, 4, ['TUVT', 'TUUT', 'TUTT', 'WTWW']),
    bottom: tx(4, 4, ['XXXX', 'XYYX', 'YXXY', 'ZZZZ']),
  },
  leftLeg: {
    front: tx(4, 12, ['TVUT', 'UUTT', 'TTUT', 'TTTU', 'TUTT', 'TTUT', 'UTTT', 'TTUT', 'XXXX', 'XYYX', 'XYXX', 'ZZZZ']),
    back: tx(4, 12, ['TUTT', 'TTUT', 'UUTT', 'TTUT', 'TTTU', 'TUTT', 'TTUT', 'UTTT', 'XXXX', 'XXYX', 'XYYX', 'ZZZZ']),
    left: tx(4, 12, ['TTTU', 'TUTT', 'TTUT', 'UUTT', 'TTUT', 'TTTU', 'TUTT', 'TTUT', 'XXXX', 'XYXX', 'XYYX', 'ZZZZ']),
    right: tx(4, 12, ['UTTT', 'TTTU', 'TUTT', 'TTUT', 'UUTT', 'TTUT', 'TTTU', 'TUTT', 'XXXX', 'XYYX', 'XXYX', 'ZZZZ']),
    top: tx(4, 4, ['TVUT', 'UUTT', 'TTUT', 'WWTW']),
    bottom: tx(4, 4, ['XXXX', 'XYXX', 'XYYX', 'ZZZZ']),
  },
} as const
