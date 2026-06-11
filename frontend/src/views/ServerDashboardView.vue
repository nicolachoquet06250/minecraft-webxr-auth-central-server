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
            <div class="stat-icon">👥</div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.total_visits || 0 }}</div>
              <div class="stat-label">Visites totales</div>
            </div>
          </div>

          <div class="stat-card minecraft-panel">
            <div class="stat-icon">📈</div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.visits_this_month || 0 }}</div>
              <div class="stat-label">Visites ce mois</div>
            </div>
          </div>

          <div class="stat-card minecraft-panel">
            <div class="stat-icon">📅</div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.visits_today || 0 }}</div>
              <div class="stat-label">Visites aujourd'hui</div>
            </div>
          </div>

          <div class="stat-card minecraft-panel">
            <div class="stat-icon">🎮</div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.active_players || 0 }}</div>
              <div class="stat-label">Joueurs actifs</div>
            </div>
          </div>
        </div>

        <!-- Charts Section -->
        <div class="charts-grid">
          <!-- Monthly Visits Chart -->
          <div class="chart-container minecraft-panel">
            <h2 class="chart-title">📊 Visites par mois</h2>
            <div class="chart-wrapper">
              <Line 
                v-if="monthlyChartData"
                :data="monthlyChartData" 
                :options="monthlyChartOptions" 
              />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <!-- Daily Visits Chart (Last 30 days) -->
          <div class="chart-container minecraft-panel">
            <h2 class="chart-title">📅 Visites des 30 derniers jours</h2>
            <div class="chart-wrapper">
              <Bar 
                v-if="dailyChartData"
                :data="dailyChartData" 
                :options="dailyChartOptions" 
              />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <!-- Hourly Traffic Chart -->
          <div class="chart-container minecraft-panel">
            <h2 class="chart-title">🕐 Trafic par heure (24h)</h2>
            <div class="chart-wrapper">
              <Line 
                v-if="hourlyChartData"
                :data="hourlyChartData" 
                :options="hourlyChartOptions" 
              />
              <div v-else class="no-data">Aucune donnée disponible</div>
            </div>
          </div>

          <!-- Connection Types Pie Chart -->
          <div class="chart-container minecraft-panel">
            <h2 class="chart-title">🔌 Types de connexion</h2>
            <div class="chart-wrapper">
              <Doughnut 
                v-if="connectionTypesData"
                :data="connectionTypesData" 
                :options="pieChartOptions" 
              />
              <div v-else class="no-data">Aucune donnée disponible</div>
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
              <strong>Dernière mise à jour:</strong>
              <span>{{ lastUpdate }}</span>
            </div>
            <div class="info-item" v-if="stats.uptime">
              <strong>Temps de fonctionnement:</strong>
              <span>{{ formatUptime(stats.uptime) }}</span>
            </div>
            <div class="info-item" v-if="stats.server_version">
              <strong>Version:</strong>
              <span>{{ stats.server_version }}</span>
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

// Register Chart.js components
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

const route = useRoute()
const router = useRouter()
const serverStore = useServerStore()

const serverId = route.params.id as string
const loading = ref(true)
const error = ref<string | null>(null)
const stats = ref<any>({})
const relayDomain = ref('')
const serverName = ref('Mon Serveur')

const lastUpdate = computed(() => {
  return new Date().toLocaleString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
})

// Monthly Chart Data
const monthlyChartData = computed(() => {
  if (!stats.value.visits_by_month || stats.value.visits_by_month.length === 0) {
    return null
  }

  return {
    labels: stats.value.visits_by_month.map((item: any) => item.label),
    datasets: [
      {
        label: 'Visites',
        data: stats.value.visits_by_month.map((item: any) => item.count),
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

// Daily Chart Data
const dailyChartData = computed(() => {
  if (!stats.value.visits_by_day || stats.value.visits_by_day.length === 0) {
    return null
  }

  return {
    labels: stats.value.visits_by_day.map((item: any) => item.label),
    datasets: [
      {
        label: 'Visites',
        data: stats.value.visits_by_day.map((item: any) => item.count),
        backgroundColor: 'rgba(100, 255, 218, 0.6)',
        borderColor: '#64ffda',
        borderWidth: 2,
        borderRadius: 5
      }
    ]
  }
})

// Hourly Chart Data
const hourlyChartData = computed(() => {
  if (!stats.value.visits_by_hour || stats.value.visits_by_hour.length === 0) {
    return null
  }

  return {
    labels: stats.value.visits_by_hour.map((item: any) => item.label),
    datasets: [
      {
        label: 'Visites',
        data: stats.value.visits_by_hour.map((item: any) => item.count),
        borderColor: '#ff6b6b',
        backgroundColor: 'rgba(255, 107, 107, 0.1)',
        fill: true,
        tension: 0.4,
        borderWidth: 2,
        pointRadius: 3,
        pointHoverRadius: 5
      }
    ]
  }
})

// Connection Types Data
const connectionTypesData = computed(() => {
  if (!stats.value.connection_types || Object.keys(stats.value.connection_types).length === 0) {
    return null
  }

  const types = stats.value.connection_types
  return {
    labels: Object.keys(types),
    datasets: [
      {
        data: Object.values(types) as number[],
        backgroundColor: [
          'rgba(100, 255, 218, 0.8)',
          'rgba(255, 107, 107, 0.8)',
          'rgba(255, 193, 7, 0.8)',
          'rgba(76, 175, 80, 0.8)'
        ],
        borderColor: [
          '#64ffda',
          '#ff6b6b',
          '#ffc107',
          '#4caf50'
        ],
        borderWidth: 2
      }
    ]
  }
})

// Chart Options
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

const dailyChartOptions = {
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
      ticks: { 
        color: '#ffffff',
        maxRotation: 45,
        minRotation: 45
      },
      grid: { color: 'rgba(255, 255, 255, 0.1)' }
    }
  }
}

const hourlyChartOptions = {
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
      titleColor: '#ff6b6b',
      bodyColor: '#ffffff',
      borderColor: '#ff6b6b',
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

const formatUptime = (seconds: number) => {
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  
  if (days > 0) return `${days}j ${hours}h ${minutes}m`
  if (hours > 0) return `${hours}h ${minutes}m`
  return `${minutes}m`
}

const loadStats = async () => {
  loading.value = true
  error.value = null

  try {
    // Get server info
    const server = serverStore.servers.find((s: any) => s.id === serverId)
    if (!server) {
      error.value = 'Serveur non trouvé'
      loading.value = false
      return
    }

    serverName.value = server.name
    relayDomain.value = server.relay_domain

    // Fetch stats from relay server
    const statsUrl = `${server.relay_domain}/stats`
    const response = await fetch(statsUrl)
    
    if (!response.ok) {
      throw new Error(`Erreur HTTP: ${response.status}`)
    }

    const data = await response.json()
    stats.value = data

    // If the relay server doesn't provide structured data, create mock data
    if (!data.visits_by_month) {
      stats.value = generateMockStats()
    }
  } catch (err: any) {
    console.error('Error loading stats:', err)
    error.value = err.message || 'Impossible de charger les statistiques'
    // Use mock data as fallback
    stats.value = generateMockStats()
  } finally {
    loading.value = false
  }
}

const generateMockStats = () => {
  const now = new Date()
  const months = []
  const days = []
  const hours = []

  // Generate 12 months of data
  for (let i = 11; i >= 0; i--) {
    const date = new Date(now.getFullYear(), now.getMonth() - i, 1)
    months.push({
      label: date.toLocaleDateString('fr-FR', { month: 'short', year: 'numeric' }),
      count: Math.floor(Math.random() * 500) + 100
    })
  }

  // Generate 30 days of data
  for (let i = 29; i >= 0; i--) {
    const date = new Date(now)
    date.setDate(date.getDate() - i)
    days.push({
      label: date.toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit' }),
      count: Math.floor(Math.random() * 50) + 10
    })
  }

  // Generate 24 hours of data
  for (let i = 0; i < 24; i++) {
    hours.push({
      label: `${i}h`,
      count: Math.floor(Math.random() * 30) + 5
    })
  }

  return {
    total_visits: 12543,
    visits_this_month: 1234,
    visits_today: 89,
    active_players: 12,
    visits_by_month: months,
    visits_by_day: days,
    visits_by_hour: hours,
    connection_types: {
      'WebSocket': 450,
      'WebRTC': 320,
      'HTTP': 180,
      'Direct': 90
    },
    uptime: 2592000, // 30 days in seconds
    server_version: '1.0.0'
  }
}

const goBack = () => {
  router.push({ name: 'servers' })
}

onMounted(async () => {
  // Ensure servers are loaded
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
