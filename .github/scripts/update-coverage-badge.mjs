import { mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'

const root = process.cwd()
const frontendSummaryPath = resolve(root, 'frontend/coverage/coverage-summary.json')
const backendLcovPath = resolve(root, 'backend/coverage/lcov.info')
const badgePath = resolve(root, '.github/badges/coverage.json')

const readFrontendLines = () => {
  try {
    const summary = JSON.parse(readFileSync(frontendSummaryPath, 'utf8'))
    const lines = summary.total?.lines
    if (!lines) return { covered: 0, total: 0 }
    return { covered: Number(lines.covered || 0), total: Number(lines.total || 0) }
  } catch {
    return { covered: 0, total: 0 }
  }
}

const readBackendLines = () => {
  try {
    const lcov = readFileSync(backendLcovPath, 'utf8')
    let covered = 0
    let total = 0
    for (const line of lcov.split('\n')) {
      if (line.startsWith('LH:')) covered += Number(line.slice(3)) || 0
      if (line.startsWith('LF:')) total += Number(line.slice(3)) || 0
    }
    return { covered, total }
  } catch {
    return { covered: 0, total: 0 }
  }
}

const colorFor = (percent) => {
  if (percent >= 90) return 'brightgreen'
  if (percent >= 75) return 'green'
  if (percent >= 60) return 'yellowgreen'
  if (percent >= 45) return 'yellow'
  if (percent >= 30) return 'orange'
  return 'red'
}

const frontend = readFrontendLines()
const backend = readBackendLines()
const covered = frontend.covered + backend.covered
const total = frontend.total + backend.total
const percent = total > 0 ? Math.round((covered / total) * 10000) / 100 : 0

mkdirSync(dirname(badgePath), { recursive: true })
writeFileSync(
  badgePath,
  `${JSON.stringify({ schemaVersion: 1, label: 'coverage', message: `${percent}%`, color: colorFor(percent) }, null, 2)}\n`,
)

console.log(`coverage=${percent}% (${covered}/${total} lines)`)
