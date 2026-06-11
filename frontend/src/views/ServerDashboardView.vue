<template>
  <div class="dashboard minecraft-bg">
    <div class="minecraft-container">
      <div class="page-header">
        <div class="header-top">
          <button @click="goBack" class="back-button minecraft-button">
            ← Retour aux serveurs
          </button>
        </div>
        <h1 class="minecraft-title">📊 Dashboard - {{ serverName }}</h1>
        <p class="minecraft-text subtitle">
          Statistiques et analyses en temps réel
        </p>
      </div>

      <div v-if="loading" class="loading-state minecraft-panel">
        <div class="loading-spinner">⏳</div>
        <p class="minecraft-text">Chargement des statistiques...</p>
      </div>

      <div v-else-if="error" class="error-state minecraft-panel">
        <div class="error-icon">❌</div>
        <h3>Erreur de chargement</h3>
        <p class="minecraft-text">{{ error }}</p>
        <button @click="loadStats" class="minecraft-button">
          🔄 Réessayer
        </button>
      </div>

      <div v-else class="dashboard-content">
        <!-- Stats Cards -->
        <div class="stats-cards">
          <div class="stat-card minecraft-panel">
            <div class="stat-icon">🔌</div>
            <div class="stat-info">
              <div class="stat-value">{{ totalConnections }}</div>
              <div class="stat-label">Connexions totales</div>
            </div>
          </div>

          <div class="stat-card minecraft-panel">
            <div class="stat-icon">🎮</div>
            <div class="stat-info">
              <div class="stat-value">{{ currentConnectedPlayers }}</div>
              <div class="stat-label">Joueurs connectés</div>
            </div>
          </div>

          <div class="stat-card minecraft-panel">
            <div class="stat-icon">👥</div>
            <div class="stat-info">
              <div class="stat-value">{{ connectedPlayersCount }}</div>
              <div class="stat-label">Joueurs listés</div>
            </div>
          </div>

          <div class="stat-card minecraft-panel">
            <div class="stat-icon">⏱️</div>
            <div class="stat-info">
              <div class="stat-value">{{ formattedAverageDuration }}</div>
              <div class="stat-label">Durée moyenne</div>
            </div>
          </div>
        </div>

        <!-- Charts Section -->
        <div class="charts-grid">
          <!-- Monthly Connections Chart -->
          <div class="chart-container minecraft-panel">
            <h2 class="chart-title">📊 Connexions par mois</h2>
            <div class="chart-wrapper">
              <Line
                v-if="monthlyChartData"
                :data="monthlyChartData"
                :options="monthlyChartOptions"
              />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <!-- Gender Connections Chart -->
          <div class="chart-container minecraft-panel">
            <h2 class="chart-title">⚧ Connexions par genre</h2>
            <div class="chart-wrapper">
              <Doughnut
                v-if="genderChartData"
                :data="genderChartData"
                :options="pieChartOptions"
              />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <!-- Month and Gender Chart -->
          <div class="chart-container minecraft-panel">
            <h2 class="chart-title">📈 Connexions par mois et genre</h2>
            <div class="chart-wrapper">
              <Bar
                v-if="monthlyGenderChartData"
                :data="monthlyGenderChartData"
                :options="monthlyGenderChartOptions"
              />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <!-- Average Session Duration Chart -->
          <div class="chart-container minecraft-panel">
            <h2 class="chart-title">⏱️ Durée moyenne par genre</h2>
            <div class="chart-wrapper">
              <Bar
                v-if="averageDurationByGenderChartData"
                :data="averageDurationByGenderChartData"
                :options="durationChartOptions"
              />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>
        </div>

        <!-- Connected Players -->
        <div class="info-section minecraft-panel" v-if="connectedPlayers.length > 0">
          <h2>👥 Joueurs actuellement connectés</h2>
          <div class="players-list">
            <div v-for="player in connectedPlayers" :key="playerKey(player)" class="player-item">
              <span>{{ playerLabel(player) }}</span>
            </div>
          </div>
        </div>

        <!-- Additional Info -->
        <div class="info-section minecraft-panel">
          <h2>ℹ️ Informations du serveur</h2>
          <div class="info-grid">
            <div class="info-item">
              <strong>Serveur relais:</strong>
              <span>{{ relayDomain }}</span>
            </div>
            <div class="info-item">
              <strong>Statistiques générées le:</strong>
              <span>{{ lastUpdate }}</span>
            </div>
            <div class="info-item">
              <strong>Endpoint statistiques:</strong>
              <span>{{ statsEndpoint }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
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

const route = useRoute()
const router = useRouter()
const serverStore = useServerStore()

const serverId = route.params.id as string
const loading = ref(true)
const error = ref<string | null>(null)
const stats = ref<ServerStats>({})
const relayDomain = ref('')
const serverName = ref('Mon Serveur')

const totalConnections = computed(() => stats.value.total_connections ?? 0)
const currentConnectedPlayers = computed(() => stats.value.current_connected_players ?? 0)
const connectedPlayers = computed(() => stats.value.connected_players ?? [])
const connectedPlayersCount = computed(() => connectedPlayers.value.length)
const averageDurationSeconds = computed(() => stats.value.average_session_duration?.average_duration_seconds ?? 0)
const formattedAverageDuration = computed(() => formatDuration(averageDurationSeconds.value))
const statsEndpoint = computed(() => relayDomain.value ? `${relayDomain.value.replace(/\/+$/, '')}/stats` : '')

const lastUpdate = computed(() => {
  if (!stats.value.generated_at) {
    return 'Non disponible'
  }

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

  if (rows.length === 0) {
    return null
  }

  return {
    labels: rows.map((item) => monthLabel(item)),
    datasets: [
      {
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
      }
    ]
  }
})

const genderChartData = computed(() => {
  const rows = stats.value.connections_by_gender ?? []

  if (rows.length === 0) {
    return null
  }

  return {
    labels: rows.map((item) => genderLabel(item)),
    datasets: [
      {
        data: rows.map((item) => numericCount(item)),
        backgroundColor: [
          'rgba(100, 255, 218, 0.8)',
          'rgba(255, 107, 107, 0.8)',
          'rgba(255, 193, 7, 0.8)',
          'rgba(76, 175, 80, 0.8)',
          'rgba(156, 39, 176, 0.8)'
        ],
        borderColor: [
          '#64ffda',
          '#ff6b6b',
          '#ffc107',
          '#4caf50',
          '#9c27b0'
        ],
        borderWidth: 2
      }
    ]
  }
})

const monthlyGenderChartData = computed(() => {
  const rows = stats.value.connections_by_month_and_gender ?? []

  if (rows.length === 0) {
    return null
  }

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

  if (rows.length === 0) {
    return null
  }

  return {
    labels: rows.map((item) => genderLabel(item)),
    datasets: [
      {
        label: 'Durée moyenne (secondes)',
        data: rows.map((item) => durationSeconds(item)),
        backgroundColor: 'rgba(100, 255, 218, 0.6)',
        borderColor: '#64ffda',
        borderWidth: 2,
        borderRadius: 5
      }
    ]
  }
})

const chartBackgroundColors = [
  'rgba(100, 255, 218, 0.6)',
  'rgba(255, 107, 107, 0.6)',
  'rgba(255, 193, 7, 0.6)',
  'rgba(76, 175, 80, 0.6)',
  'rgba(156, 39, 176, 0.6)'
]

const chartBorderColors = [
  '#64ffda',
  '#ff6b6b',
  '#ffc107',
  '#4caf50',
  '#9c27b0'
]

const monthlyChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: true,
      labels: {
        color: '#ffffff',
        font: { size: 14 }
      }
    },
    tooltip: {
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      titleColor: '#64ffda',
      bodyColor: '#ffffff',
      borderColor: '#64ffda',
      borderWidth: 1
    }
  },
  scales: {
    y: {
      beginAtZero: true,
      ticks: { color: '#ffffff' },
      grid: { color: 'rgba(255, 255, 255, 0.1)' }
    },
    x: {
      ticks: { color: '#ffffff' },
      grid: { color: 'rgba(255, 255, 255, 0.1)' }
    }
  }
}

const monthlyGenderChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: true,
      labels: {
        color: '#ffffff',
        font: { size: 14 }
      }
    },
    tooltip: {
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      titleColor: '#64ffda',
      bodyColor: '#ffffff',
      borderColor: '#64ffda',
      borderWidth: 1
    }
  },
  scales: {
    y: {
      beginAtZero: true,
      stacked: true,
      ticks: { color: '#ffffff' },
      grid: { color: 'rgba(255, 255, 255, 0.1)' }
    },
    x: {
      stacked: true,
      ticks: { color: '#ffffff' },
      grid: { color: 'rgba(255, 255, 255, 0.1)' }
    }
  }
}

const durationChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: true,
      labels: {
        color: '#ffffff',
        font: { size: 14 }
      }
    },
    tooltip: {
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      titleColor: '#64ffda',
      bodyColor: '#ffffff',
      borderColor: '#64ffda',
      borderWidth: 1,
      callbacks: {
        label: (context: any) => `Durée moyenne: ${formatDuration(Number(context.raw) || 0)}`
      }
    }
  },
  scales: {
    y: {
      beginAtZero: true,
      ticks: {
        color: '#ffffff',
        callback: (value: any) => formatDuration(Number(value) || 0)
      },
      grid: { color: 'rgba(255, 255, 255, 0.1)' }
    },
    x: {
      ticks: { color: '#ffffff' },
      grid: { color: 'rgba(255, 255, 255, 0.1)' }
    }
  }
}

const pieChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'bottom' as const,
      labels: {
        color: '#ffffff',
        font: { size: 14 },
        padding: 15
      }
    },
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
      loading.value = false
      return
    }

    serverName.value = server.name
    relayDomain.value = server.relay_domain

    const response = await fetch(statsEndpoint.value)

    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`)
    }

    stats.value = await response.json()
  } catch (err: any) {
    console.error('Error loading stats:', err)
    error.value = err.message || 'Impossible de charger les statistiques'
    stats.value = {}
  } finally {
    loading.value = false
  }
}

const monthLabel = (item: CountByMonth) => {
  return String(item.month ?? item.label ?? item.period ?? item.date ?? 'Non renseigné')
}

const genderLabel = (item: CountByGender) => {
  return String(item.gender ?? item.label ?? item.name ?? 'Non renseigné')
}

const numericCount = (item: CountByGender | CountByMonth | CountByMonthAndGender) => {
  return Number(item.count ?? item.total ?? item.connections ?? item.total_connections ?? 0)
}

const durationSeconds = (item: CountByGender) => {
  return Number((item as any).average_duration_seconds ?? item.count ?? item.total ?? 0)
}

const uniqueValues = (values: string[]) => {
  return Array.from(new Set(values))
}

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
  if (typeof player === 'string') {
    return player
  }

  if (player && typeof player === 'object') {
    const record = player as Record<string, unknown>
    return String(record.username ?? record.name ?? record.id ?? JSON.stringify(record))
  }

  return String(player)
}

const playerKey = (player: unknown) => {
  if (player && typeof player === 'object') {
    const record = player as Record<string, unknown>
    return String(record.id ?? record.username ?? record.name ?? JSON.stringify(record))
  }

  return String(player)
}

const goBack = () => {
  router.push({ name: 'servers' })
}

onMounted(async () => {
  if (serverStore.servers.length === 0) {
    await serverStore.fetchUserServers()
  }
  await loadStats()
})
</script>

<style scoped>
.dashboard {
  min-height: calc(100vh - 80px);
  padding: 2rem 1rem;
}

.minecraft-container {
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

.back-button:hover {
  background-color: #616161;
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
  margin: 2rem auto;
}

.loading-spinner {
  font-size: 4rem;
  margin-bottom: 1rem;
  animation: spin 2s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.error-state h3 {
  font-size: 1.5rem;
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
  padding: 1.5rem;
  gap: 1.5rem;
}

.stat-icon {
  font-size: 3rem;
  line-height: 1;
}

.stat-info {
  flex: 1;
  text-align: left;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
  color: #64ffda;
  margin-bottom: 0.25rem;
}

.stat-label {
  font-size: 0.95rem;
  opacity: 0.8;
  color: #ffffff;
}

.charts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
  gap: 2rem;
  margin-bottom: 2rem;
}

.chart-container {
  padding: 1.5rem;
}

.chart-title {
  font-size: 1.3rem;
  margin-bottom: 1rem;
  color: #64ffda;
}

.chart-wrapper {
  height: 300px;
  position: relative;
}

.no-data {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
  font-size: 1.1rem;
}

.info-section {
  padding: 2rem;
  margin-bottom: 2rem;
}

.info-section h2 {
  font-size: 1.5rem;
  margin-bottom: 1.5rem;
  color: #64ffda;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.info-item strong {
  color: #64ffda;
  font-size: 0.95rem;
}

.info-item span {
  color: #ffffff;
  font-size: 1.05rem;
  word-break: break-word;
}

.players-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 1rem;
}

.player-item {
  padding: 1rem;
  background: rgba(0, 0, 0, 0.25);
  border: 1px solid rgba(100, 255, 218, 0.35);
  color: #ffffff;
}

@media (max-width: 1200px) {
  .charts-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 768px) {
  .dashboard {
    padding: 1rem;
  }

  .header-top {
    margin-bottom: 1rem;
  }

  .stats-cards {
    grid-template-columns: 1fr;
  }

  .info-grid {
    grid-template-columns: 1fr;
  }
}
</style>
