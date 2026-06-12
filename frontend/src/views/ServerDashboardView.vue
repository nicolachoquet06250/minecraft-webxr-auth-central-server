<template>
  <div class="dashboard voxicraft-bg">
    <div class="voxicraft-container">
      <div class="page-header">
        <div class="header-top">
          <button @click="goBack" class="back-button voxicraft-button">
            ← Retour aux serveurs
          </button>
        </div>
        <h1 class="voxicraft-title">📊 Dashboard - {{ serverName }}</h1>
        <p class="voxicraft-text subtitle">Statistiques et analyses en temps réel</p>
      </div>

      <div v-if="loading" class="loading-state voxicraft-panel">
        <div class="loading-spinner">⏳</div>
        <p class="voxicraft-text">Chargement des statistiques...</p>
      </div>

      <div v-else-if="error" class="error-state voxicraft-panel">
        <div class="error-icon">❌</div>
        <h3>Erreur de chargement</h3>
        <p class="voxicraft-text">{{ error }}</p>
        <button @click="loadStats" class="voxicraft-button">🔄 Réessayer</button>
      </div>

      <div v-else class="dashboard-content">
        <div class="stats-cards">
          <div class="stat-card voxicraft-panel">
            <div class="stat-icon">🔌</div>
            <div class="stat-info">
              <div class="stat-value">{{ totalConnections }}</div>
              <div class="stat-label">Connexions totales</div>
            </div>
          </div>

          <div class="stat-card voxicraft-panel">
            <div class="stat-icon">🎮</div>
            <div class="stat-info">
              <div class="stat-value">{{ currentConnectedPlayers }}</div>
              <div class="stat-label">Joueurs connectés</div>
            </div>
          </div>

          <div class="stat-card voxicraft-panel">
            <div class="stat-icon">👥</div>
            <div class="stat-info">
              <div class="stat-value">{{ connectedPlayersCount }}</div>
              <div class="stat-label">Joueurs listés en direct</div>
            </div>
          </div>

          <div class="stat-card voxicraft-panel">
            <div class="stat-icon">⏱️</div>
            <div class="stat-info">
              <div class="stat-value">{{ formattedAverageDuration }}</div>
              <div class="stat-label">Durée moyenne</div>
            </div>
          </div>
        </div>

        <div class="charts-grid">
          <div class="chart-container voxicraft-panel">
            <h2 class="chart-title">📊 Connexions par mois</h2>
            <div class="chart-wrapper">
              <Line v-if="monthlyChartData" :data="monthlyChartData" :options="monthlyChartOptions" />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <div class="chart-container voxicraft-panel">
            <h2 class="chart-title">⚧ Connexions par genre</h2>
            <div class="chart-wrapper">
              <Doughnut v-if="genderChartData" :data="genderChartData" :options="pieChartOptions" />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <div class="chart-container voxicraft-panel">
            <h2 class="chart-title">📈 Connexions par mois et genre</h2>
            <div class="chart-wrapper">
              <Bar v-if="monthlyGenderChartData" :data="monthlyGenderChartData" :options="monthlyGenderChartOptions" />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <div class="chart-container voxicraft-panel">
            <h2 class="chart-title">⏱️ Durée moyenne par genre</h2>
            <div class="chart-wrapper">
              <Bar v-if="averageDurationByGenderChartData" :data="averageDurationByGenderChartData" :options="durationChartOptions" />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>
        </div>

        <div class="info-section voxicraft-panel">
          <div class="section-header">
            <h2>👥 Joueurs actuellement connectés</h2>
            <span :class="['websocket-status', websocketStatusClass]">
              {{ websocketStatusLabel }}
            </span>
          </div>

          <div v-if="connectedPlayers.length > 0" class="players-list">
            <div v-for="player in connectedPlayers" :key="playerKey(player)" class="player-item">
              <span>{{ playerLabel(player) }}</span>
            </div>
          </div>
          <div v-else class="no-data no-players">Aucun joueur connecté</div>
        </div>

        <div class="info-section voxicraft-panel">
          <h2>ℹ️ Informations du serveur</h2>
          <div class="info-grid">
            <div class="info-item">
              <strong>Serveur de jeu:</strong>
              <span>{{ gameDomain }}</span>
            </div>
            <div class="info-item">
              <strong>Statistiques générées le:</strong>
              <span>{{ lastUpdate }}</span>
            </div>
            <div class="info-item">
              <strong>Endpoint statistiques:</strong>
              <span>{{ statsEndpoint }}</span>
            </div>
            <div class="info-item">
              <strong>WebSocket serveur de jeu:</strong>
              <span>{{ activeWebSocketEndpoint || 'Connexion non établie' }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useServerStore } from '@/stores/server'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  ArcElement,
  Title,
  Tooltip,
  Legend,
  Filler
} from 'chart.js'
import { Line, Bar, Doughnut } from 'vue-chartjs'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  ArcElement,
  Title,
  Tooltip,
  Legend,
  Filler
)

type CountByGender = {
  gender?: string
  label?: string
  name?: string
  count?: number
  total?: number
  connections?: number
  total_connections?: number
  average_duration_seconds?: number
}

type CountByMonth = {
  month?: string
  label?: string
  period?: string
  date?: string
  count?: number
  total?: number
  connections?: number
  total_connections?: number
}

type CountByMonthAndGender = CountByGender & CountByMonth

type AverageSessionDuration = {
  average_duration_seconds?: number
  by_gender?: CountByGender[]
}

type ServerStats = {
  generated_at?: string
  total_connections?: number
  current_connected_players?: number
  connected_players?: unknown[]
  connections_by_gender?: CountByGender[]
  connections_by_month?: CountByMonth[]
  connections_by_month_and_gender?: CountByMonthAndGender[]
  average_session_duration?: AverageSessionDuration
}

type GameServerWebSocketMessage = Partial<ServerStats> & {
  type?: string
  event?: string
  player?: unknown
  players?: unknown[]
  online_players?: unknown[]
  stats?: Partial<ServerStats>
  data?: Partial<ServerStats> & {
    player?: unknown
    players?: unknown[]
    online_players?: unknown[]
    connected_players?: unknown[]
  }
}

const route = useRoute()
const router = useRouter()
const serverStore = useServerStore()

const serverId = route.params.id as string
const loading = ref(true)
const error = ref<string | null>(null)
const stats = ref<ServerStats>({})
const gameDomain = ref('')
const serverName = ref('Mon Serveur')
const liveConnectedPlayers = ref<unknown[]>([])
const websocketStatus = ref<'disconnected' | 'connecting' | 'connected' | 'error'>('disconnected')
const activeWebSocketEndpoint = ref('')

let gameServerWebSocket: WebSocket | null = null
let reconnectTimeoutId: number | undefined
let currentWebSocketCandidateIndex = 0
let currentWebSocketCandidates: string[] = []
let shouldReconnectWebSocket = true

const totalConnections = computed(() => stats.value.total_connections ?? 0)
const connectedPlayers = computed(() => liveConnectedPlayers.value)
const connectedPlayersCount = computed(() => connectedPlayers.value.length)
const currentConnectedPlayers = computed(() => connectedPlayersCount.value || stats.value.current_connected_players || 0)
const averageDurationSeconds = computed(() => stats.value.average_session_duration?.average_duration_seconds ?? 0)
const formattedAverageDuration = computed(() => formatDuration(averageDurationSeconds.value))
const statsEndpoint = computed(() => gameDomain.value ? `${gameDomain.value.replace(/\/+$/, '')}/stats` : '')

const websocketStatusLabel = computed(() => {
  if (websocketStatus.value === 'connected') return '🟢 Temps réel connecté'
  if (websocketStatus.value === 'connecting') return '🟡 Connexion temps réel...'
  if (websocketStatus.value === 'error') return '🔴 Temps réel indisponible'
  return '⚪ Temps réel déconnecté'
})

const websocketStatusClass = computed(() => `status-${websocketStatus.value}`)

const lastUpdate = computed(() => {
  if (!stats.value.generated_at) return 'Non disponible'

  return new Date(stats.value.generated_at).toLocaleString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
})

const monthlyChartData = computed(() => {
  const rows = stats.value.connections_by_month ?? []
  if (rows.length === 0) return null

  return {
    labels: rows.map((item) => monthLabel(item)),
    datasets: [{
      label: 'Connexions',
      data: rows.map((item) => numericCount(item)),
      borderColor: '#64ffda',
      backgroundColor: 'rgba(100, 255, 218, 0.1)',
      fill: true,
      tension: 0.4,
      borderWidth: 3,
      pointRadius: 5,
      pointHoverRadius: 7,
      pointBackgroundColor: '#64ffda',
      pointBorderColor: '#1a1a1a',
      pointBorderWidth: 2
    }]
  }
})

const genderChartData = computed(() => {
  const rows = stats.value.connections_by_gender ?? []
  if (rows.length === 0) return null

  return {
    labels: rows.map((item) => genderLabel(item)),
    datasets: [{
      data: rows.map((item) => numericCount(item)),
      backgroundColor: chartBackgroundColors,
      borderColor: chartBorderColors,
      borderWidth: 2
    }]
  }
})

const monthlyGenderChartData = computed(() => {
  const rows = stats.value.connections_by_month_and_gender ?? []
  if (rows.length === 0) return null

  const months = uniqueValues(rows.map((item) => monthLabel(item)))
  const genders = uniqueValues(rows.map((item) => genderLabel(item)))

  return {
    labels: months,
    datasets: genders.map((gender, index) => ({
      label: gender,
      data: months.map((month) => {
        const row = rows.find((item) => monthLabel(item) === month && genderLabel(item) === gender)
        return row ? numericCount(row) : 0
      }),
      backgroundColor: chartBackgroundColors[index % chartBackgroundColors.length],
      borderColor: chartBorderColors[index % chartBorderColors.length],
      borderWidth: 2,
      borderRadius: 5
    }))
  }
})

const averageDurationByGenderChartData = computed(() => {
  const rows = stats.value.average_session_duration?.by_gender ?? []
  if (rows.length === 0) return null

  return {
    labels: rows.map((item) => genderLabel(item)),
    datasets: [{
      label: 'Durée moyenne (secondes)',
      data: rows.map((item) => durationSeconds(item)),
      backgroundColor: 'rgba(100, 255, 218, 0.6)',
      borderColor: '#64ffda',
      borderWidth: 2,
      borderRadius: 5
    }]
  }
})

const chartBackgroundColors = [
  'rgba(100, 255, 218, 0.6)',
  'rgba(255, 107, 107, 0.6)',
  'rgba(255, 193, 7, 0.6)',
  'rgba(76, 175, 80, 0.6)',
  'rgba(156, 39, 176, 0.6)'
]

const chartBorderColors = ['#64ffda', '#ff6b6b', '#ffc107', '#4caf50', '#9c27b0']

const baseChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { labels: { color: '#ffffff', font: { size: 14 } } },
    tooltip: {
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      titleColor: '#64ffda',
      bodyColor: '#ffffff',
      borderColor: '#64ffda',
      borderWidth: 1
    }
  },
  scales: {
    y: { beginAtZero: true, ticks: { color: '#ffffff' }, grid: { color: 'rgba(255, 255, 255, 0.1)' } },
    x: { ticks: { color: '#ffffff' }, grid: { color: 'rgba(255, 255, 255, 0.1)' } }
  }
}

const monthlyChartOptions = baseChartOptions
const monthlyGenderChartOptions = {
  ...baseChartOptions,
  scales: {
    y: { beginAtZero: true, stacked: true, ticks: { color: '#ffffff' }, grid: { color: 'rgba(255, 255, 255, 0.1)' } },
    x: { stacked: true, ticks: { color: '#ffffff' }, grid: { color: 'rgba(255, 255, 255, 0.1)' } }
  }
}
const durationChartOptions = baseChartOptions
const pieChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { position: 'bottom' as const, labels: { color: '#ffffff', font: { size: 14 }, padding: 15 } },
    tooltip: {
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      titleColor: '#64ffda',
      bodyColor: '#ffffff',
      borderColor: '#64ffda',
      borderWidth: 1
    }
  }
}

const loadStats = async () => {
  loading.value = true
  error.value = null

  try {
    const server = serverStore.servers.find((s: any) => s.id === serverId)
    if (!server) {
      error.value = 'Serveur non trouvé'
      return
    }

    serverName.value = server.name
    gameDomain.value = server.game_domain

    const response = await fetch(statsEndpoint.value)
    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`)
    }

    const data = await response.json()
    stats.value = data
    liveConnectedPlayers.value = normalizePlayers(data.connected_players)
    connectGameServerWebSocket()
  } catch (err: any) {
    console.error('Error loading stats:', err)
    error.value = err.message || 'Impossible de charger les statistiques'
    stats.value = {}
    liveConnectedPlayers.value = []
    closeGameServerWebSocket(false)
  } finally {
    loading.value = false
  }
}

const connectGameServerWebSocket = () => {
  closeGameServerWebSocket(false)
  currentWebSocketCandidates = buildWebSocketCandidates(gameDomain.value)
  currentWebSocketCandidateIndex = 0
  shouldReconnectWebSocket = true
  openCurrentGameServerWebSocket()
}

const openCurrentGameServerWebSocket = () => {
  if (!shouldReconnectWebSocket || currentWebSocketCandidates.length === 0) {
    websocketStatus.value = 'error'
    return
  }

  const endpoint = currentWebSocketCandidates[currentWebSocketCandidateIndex]
  activeWebSocketEndpoint.value = endpoint
  websocketStatus.value = 'connecting'

  try {
    gameServerWebSocket = new WebSocket(endpoint)
    gameServerWebSocket.onopen = () => {
      websocketStatus.value = 'connected'
      currentWebSocketCandidateIndex = 0
    }
    gameServerWebSocket.onmessage = (event) => handleGameServerWebSocketMessage(event.data)
    gameServerWebSocket.onerror = () => {
      websocketStatus.value = 'error'
    }
    gameServerWebSocket.onclose = () => {
      gameServerWebSocket = null
      if (!shouldReconnectWebSocket) {
        websocketStatus.value = 'disconnected'
        return
      }
      currentWebSocketCandidateIndex = (currentWebSocketCandidateIndex + 1) % currentWebSocketCandidates.length
      websocketStatus.value = 'connecting'
      reconnectTimeoutId = window.setTimeout(openCurrentGameServerWebSocket, 1500)
    }
  } catch (err) {
    console.error('Unable to open game server websocket:', err)
    websocketStatus.value = 'error'
    currentWebSocketCandidateIndex = (currentWebSocketCandidateIndex + 1) % currentWebSocketCandidates.length
    reconnectTimeoutId = window.setTimeout(openCurrentGameServerWebSocket, 1500)
  }
}

const closeGameServerWebSocket = (disableReconnect = true) => {
  shouldReconnectWebSocket = !disableReconnect
  if (reconnectTimeoutId !== undefined) {
    window.clearTimeout(reconnectTimeoutId)
    reconnectTimeoutId = undefined
  }
  if (gameServerWebSocket) {
    gameServerWebSocket.close()
    gameServerWebSocket = null
  }
  if (disableReconnect) {
    websocketStatus.value = 'disconnected'
    activeWebSocketEndpoint.value = ''
  }
}

const handleGameServerWebSocketMessage = (rawMessage: unknown) => {
  const message = parseMessage(rawMessage)
  if (Array.isArray(message)) {
    updateConnectedPlayers(message)
    return
  }
  if (!message || typeof message !== 'object') return

  const typedMessage = message as GameServerWebSocketMessage
  const nestedData = typedMessage.data ?? {}
  const statsPayload = typedMessage.stats ?? (hasStatsShape(typedMessage) ? typedMessage : undefined)
  if (statsPayload) mergeStats(statsPayload)

  const playerList = typedMessage.connected_players
    ?? typedMessage.players
    ?? typedMessage.online_players
    ?? nestedData.connected_players
    ?? nestedData.players
    ?? nestedData.online_players

  if (Array.isArray(playerList)) {
    updateConnectedPlayers(playerList)
    return
  }

  const eventType = String(typedMessage.type ?? typedMessage.event ?? '').toLowerCase()
  const player = typedMessage.player ?? nestedData.player
  if (eventType.includes('connect') && !eventType.includes('disconnect') && player) addConnectedPlayer(player)
  if ((eventType.includes('disconnect') || eventType.includes('leave')) && player) removeConnectedPlayer(player)
  if (typeof typedMessage.current_connected_players === 'number') {
    stats.value = { ...stats.value, current_connected_players: typedMessage.current_connected_players }
  }
}

const parseMessage = (rawMessage: unknown) => {
  if (typeof rawMessage !== 'string') return rawMessage
  try {
    return JSON.parse(rawMessage)
  } catch {
    return rawMessage
  }
}

const hasStatsShape = (value: Partial<ServerStats>) => {
  return value.total_connections !== undefined
    || value.current_connected_players !== undefined
    || value.connected_players !== undefined
    || value.connections_by_gender !== undefined
    || value.connections_by_month !== undefined
    || value.connections_by_month_and_gender !== undefined
    || value.average_session_duration !== undefined
}

const mergeStats = (partialStats: Partial<ServerStats>) => {
  stats.value = {
    ...stats.value,
    ...partialStats,
    average_session_duration: {
      ...stats.value.average_session_duration,
      ...partialStats.average_session_duration
    }
  }
  if (Array.isArray(partialStats.connected_players)) updateConnectedPlayers(partialStats.connected_players)
}

const updateConnectedPlayers = (players: unknown[]) => {
  liveConnectedPlayers.value = normalizePlayers(players)
  stats.value = {
    ...stats.value,
    current_connected_players: liveConnectedPlayers.value.length,
    connected_players: liveConnectedPlayers.value
  }
}

const addConnectedPlayer = (player: unknown) => {
  const nextPlayers = [...liveConnectedPlayers.value]
  const key = playerKey(player)
  if (!nextPlayers.some((currentPlayer) => playerKey(currentPlayer) === key)) nextPlayers.push(player)
  updateConnectedPlayers(nextPlayers)
}

const removeConnectedPlayer = (player: unknown) => {
  const key = playerKey(player)
  updateConnectedPlayers(liveConnectedPlayers.value.filter((currentPlayer) => playerKey(currentPlayer) !== key))
}

const normalizePlayers = (players: unknown) => Array.isArray(players) ? players.filter((player) => player !== null && player !== undefined) : []

const buildWebSocketCandidates = (domain: string) => {
  if (!domain) return []
  try {
    const url = new URL(domain)
    const protocol = url.protocol === 'https:' ? 'wss:' : 'ws:'
    const basePath = url.pathname.replace(/\/+$/, '')
    const origin = `${protocol}//${url.host}`
    return uniqueValues([
      `${origin}${basePath}/ws`,
      `${origin}${basePath}/api/ws`,
      `${origin}/ws`,
      `${origin}/api/ws`,
      `${origin}/stats/ws`
    ])
  } catch {
    return []
  }
}

const monthLabel = (item: CountByMonth) => String(item.month ?? item.label ?? item.period ?? item.date ?? 'Non renseigné')
const genderLabel = (item: CountByGender) => String(item.gender ?? item.label ?? item.name ?? 'Non renseigné')
const numericCount = (item: CountByGender | CountByMonth | CountByMonthAndGender) => Number(item.count ?? item.total ?? item.connections ?? item.total_connections ?? 0)
const durationSeconds = (item: CountByGender) => Number(item.average_duration_seconds ?? item.count ?? item.total ?? 0)
const uniqueValues = (values: string[]) => Array.from(new Set(values))

const formatDuration = (seconds: number) => {
  const safeSeconds = Math.max(0, Math.floor(seconds))
  const hours = Math.floor(safeSeconds / 3600)
  const minutes = Math.floor((safeSeconds % 3600) / 60)
  const remainingSeconds = safeSeconds % 60
  if (hours > 0) return `${hours}h ${minutes}m`
  if (minutes > 0) return `${minutes}m ${remainingSeconds}s`
  return `${remainingSeconds}s`
}

const playerLabel = (player: unknown) => {
  if (typeof player === 'string') return player
  if (player && typeof player === 'object') {
    const record = player as Record<string, unknown>
    return String(record.username ?? record.name ?? record.display_name ?? record.player_name ?? record.id ?? JSON.stringify(record))
  }
  return String(player)
}

const playerKey = (player: unknown) => {
  if (player && typeof player === 'object') {
    const record = player as Record<string, unknown>
    return String(record.id ?? record.uuid ?? record.username ?? record.name ?? record.display_name ?? record.player_name ?? JSON.stringify(record))
  }
  return String(player)
}

const goBack = () => {
  router.push({ name: 'servers' })
}

onMounted(async () => {
  if (serverStore.servers.length === 0) await serverStore.fetchUserServers()
  await loadStats()
})

onBeforeUnmount(() => {
  closeGameServerWebSocket(true)
})
</script>

<style scoped>
.dashboard {
  min-height: calc(100vh - 80px);
  padding: 2rem 1rem;
}

.voxicraft-container {
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  text-align: center;
  margin-bottom: 2rem;
}

.header-top {
  display: flex;
  justify-content: flex-start;
  margin-bottom: 1.5rem;
}

.back-button {
  background-color: #424242;
  border-color: #212121;
  padding: 0.5rem 1rem;
  font-size: 0.95rem;
}

.subtitle {
  font-size: 1.1rem;
  opacity: 0.9;
  margin-top: 0.5rem;
}

.loading-state,
.error-state {
  text-align: center;
  padding: 3rem 2rem;
  max-width: 600px;
  margin: 0 auto;
}

.loading-spinner,
.error-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
}

.stat-icon {
  font-size: 2.5rem;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
  color: #64ffda;
}

.stat-label {
  opacity: 0.85;
  margin-top: 0.25rem;
}

.charts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(420px, 1fr));
  gap: 2rem;
  margin-bottom: 2rem;
}

.chart-container {
  padding: 1.5rem;
}

.chart-title {
  margin-bottom: 1rem;
  color: #64ffda;
}

.chart-wrapper {
  height: 320px;
}

.info-section {
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: center;
  margin-bottom: 1rem;
}

.websocket-status {
  padding: 0.35rem 0.7rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.08);
  font-size: 0.9rem;
}

.status-connected {
  color: #64ffda;
}

.status-error {
  color: #ff6b6b;
}

.players-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 0.75rem;
}

.player-item {
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 6px;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
  word-break: break-word;
}

.info-item strong {
  color: #64ffda;
}

.no-data {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  opacity: 0.75;
}

.no-players {
  min-height: 80px;
}

@media (max-width: 768px) {
  .dashboard {
    padding: 1rem;
  }

  .charts-grid {
    grid-template-columns: 1fr;
  }

  .chart-wrapper {
    height: 260px;
  }

  .section-header {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
