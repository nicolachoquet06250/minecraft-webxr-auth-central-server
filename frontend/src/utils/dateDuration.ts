const DAY_MS = 24 * 60 * 60 * 1000

type DurationUnit = {
  value: number
  singular: string
  plural: string
}

const formatUnit = ({ value, singular, plural }: DurationUnit) => `${value} ${value > 1 ? plural : singular}`

const joinUnits = (units: DurationUnit[]) => {
  const visibleUnits = units.filter((unit) => unit.value > 0).slice(0, 2)
  if (visibleUnits.length === 0) return 'moins d’un jour'
  if (visibleUnits.length === 1) return formatUnit(visibleUnits[0])
  return `${formatUnit(visibleUnits[0])} et ${formatUnit(visibleUnits[1])}`
}

export const formatApproxDurationSince = (value: string, now = new Date()) => {
  const start = new Date(value)
  const diffMs = now.getTime() - start.getTime()

  if (Number.isNaN(start.getTime()) || diffMs <= 0) return 'moins d’un jour'

  const totalDays = Math.floor(diffMs / DAY_MS)
  const years = Math.floor(totalDays / 365)
  const months = Math.floor((totalDays % 365) / 30)
  const days = totalDays % 30

  if (years > 0) {
    return joinUnits([
      { value: years, singular: 'an', plural: 'ans' },
      { value: months, singular: 'mois', plural: 'mois' },
    ])
  }

  if (months > 0) {
    return joinUnits([
      { value: months, singular: 'mois', plural: 'mois' },
      { value: days, singular: 'jour', plural: 'jours' },
    ])
  }

  return joinUnits([{ value: totalDays, singular: 'jour', plural: 'jours' }])
}
