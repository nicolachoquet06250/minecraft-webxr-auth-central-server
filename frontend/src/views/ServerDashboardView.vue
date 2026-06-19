<template>
  <div class="dashboard voxicraft-bg">
    <div class="voxicraft-container">
      <div class="page-header">
        <div class="header-top">
          <button @click="goBack" class="back-button voxicraft-button">← Retour aux serveurs</button>
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
          </div>
          <div v-if="connectedPlayers.length > 0" class="players-list">
            <div v-for="player in connectedPlayers" :key="playerKey(player)" class="player-item">
              <div class="player-avatar">
                <img v-if="playerAvatarUrl(player)" :src="playerAvatarUrl(player)" :alt="`Tête de ${playerLabel(player)}`" class="player-avatar-image">
                <span v-else class="player-avatar-loading" aria-label="Image de profil en chargement"></span>
              </div>
              <div class="player-main">
                <span class="player-name-row">
                  <span class="player-name">{{ playerLabel(player) }}</span>
                  <span v-if="playerIsFriend(player)" class="friend-indicator" title="Ami">🤝</span>
                </span>
              </div>
            </div>
          </div>
          <div v-else class="no-data no-players">Aucun joueur connecté</div>
        </div>

        <div class="info-section voxicraft-panel">
          <h2 class="chart-title">ℹ️ Informations du serveur</h2>
          <div class="info-grid">
            <div class="info-item">
              <strong>Serveur de jeu:</strong>
              <span>{{ gameDomain }}</span>
            </div>
            <div class="info-item">
              <strong>Endpoint statistiques:</strong>
              <span>{{ statsEndpoint }}</span>
            </div>
            <div class="info-item">
              <strong>WebSocket central:</strong>
              <span>{{ activeCentralPresenceEndpoint || 'Connexion non établie' }}</span>
            </div>
            <div class="info-item">
              <strong>État temps réel:</strong>
              <span>{{ centralPresenceStatusLabel }}</span>
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
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, BarElement, ArcElement, Title, Tooltip, Legend, Filler } from 'chart.js'
import { Line, Bar, Doughnut } from 'vue-chartjs'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, BarElement, ArcElement, Title, Tooltip, Legend, Filler)

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const AUTH_TOKEN_STORAGE_KEY = 'auth_token'

type CountByGender = { gender?: string; label?: string; name?: string; avatar?: string; avatar_kind?: string; default_avatar?: string; base_kind?: string; count?: number; total?: number; connections?: number; total_connections?: number; average_duration_seconds?: number }
type CountByMonth = { month?: string; label?: string; period?: string; date?: string; count?: number; total?: number; connections?: number; total_connections?: number }
type CountByMonthAndGender = CountByGender & CountByMonth
type AverageSessionDuration = { average_duration_seconds?: number; by_gender?: CountByGender[] }
type ConnectedPlayer = Record<string, unknown>
type ServerStats = { generated_at?: string; total_connections?: number; current_connected_players?: number; connected_players?: unknown[]; connections_by_gender?: CountByGender[]; connections_by_month?: CountByMonth[]; connections_by_month_and_gender?: CountByMonthAndGender[]; average_session_duration?: AverageSessionDuration }
type ServerPresenceSnapshotMessage = { type?: string; payload?: { players?: unknown[]; current_connected_players?: number } }

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
const playerAvatarUrls = ref<Record<string, string>>({})
const centralPresenceStatus = ref<'disconnected' | 'connecting' | 'connected' | 'error'>('disconnected')
const activeCentralPresenceEndpoint = ref('')
let centralPresenceSocket: WebSocket | null = null
let centralPresenceReconnectTimer: number | undefined
let shouldReconnectCentralPresence = true

const totalConnections = computed(() => stats.value.total_connections ?? 0)
const connectedPlayers = computed(() => liveConnectedPlayers.value)
const connectedPlayersCount = computed(() => connectedPlayers.value.length)
const currentConnectedPlayers = computed(() => connectedPlayersCount.value || stats.value.current_connected_players || 0)
const averageDurationSeconds = computed(() => stats.value.average_session_duration?.average_duration_seconds ?? 0)
const formattedAverageDuration = computed(() => formatDuration(averageDurationSeconds.value))
const statsEndpoint = computed(() => gameDomain.value ? `${gameDomain.value.replace(/\/+$/, '')}/stats` : '')
const centralPresenceStatusLabel = computed(() => {
  if (centralPresenceStatus.value === 'connected') return 'Connecté au central'
  if (centralPresenceStatus.value === 'connecting') return 'Connexion au central...'
  if (centralPresenceStatus.value === 'error') return 'Erreur websocket central'
  return 'Déconnecté'
})

const monthlyChartData = computed(() => {
  const rows = stats.value.connections_by_month ?? []
  if (rows.length === 0) return null
  return { labels: rows.map(monthLabel), datasets: [{ label: 'Connexions', data: rows.map(numericCount), borderColor: '#64ffda', backgroundColor: 'rgba(100, 255, 218, 0.1)', fill: true, tension: 0.4, borderWidth: 3, pointRadius: 5, pointHoverRadius: 7, pointBackgroundColor: '#64ffda', pointBorderColor: '#1a1a1a', pointBorderWidth: 2 }] }
})

const genderChartData = computed(() => {
  const rows = stats.value.connections_by_gender ?? []
  if (rows.length === 0) return null
  return { labels: rows.map(genderLabel), datasets: [{ data: rows.map(numericCount), backgroundColor: chartBackgroundColors, borderColor: chartBorderColors, borderWidth: 2 }] }
})

const monthlyGenderChartData = computed(() => {
  const rows = stats.value.connections_by_month_and_gender ?? []
  if (rows.length === 0) return null
  const months = uniqueValues(rows.map(monthLabel))
  const genders = uniqueValues(rows.map(genderLabel))
  return { labels: months, datasets: genders.map((gender, index) => ({ label: gender, data: months.map((month) => numericCount(rows.find((item) => monthLabel(item) === month && genderLabel(item) === gender) ?? {})), backgroundColor: chartBackgroundColors[index % chartBackgroundColors.length], borderColor: chartBorderColors[index % chartBorderColors.length], borderWidth: 2, borderRadius: 5 })) }
})

const averageDurationByGenderChartData = computed(() => {
  const rows = stats.value.average_session_duration?.by_gender ?? []
  if (rows.length === 0) return null
  return { labels: rows.map(genderLabel), datasets: [{ label: 'Durée moyenne (secondes)', data: rows.map(durationSeconds), backgroundColor: 'rgba(100, 255, 218, 0.6)', borderColor: '#64ffda', borderWidth: 2, borderRadius: 5 }] }
})

const chartBackgroundColors = ['rgba(100, 255, 218, 0.6)', 'rgba(255, 107, 107, 0.6)', 'rgba(255, 193, 7, 0.6)', 'rgba(76, 175, 80, 0.6)', 'rgba(156, 39, 176, 0.6)']
const chartBorderColors = ['#64ffda', '#ff6b6b', '#ffc107', '#4caf50', '#9c27b0']
const baseChartOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { labels: { color: '#ffffff', font: { size: 14 } } }, tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.8)', titleColor: '#64ffda', bodyColor: '#ffffff', borderColor: '#64ffda', borderWidth: 1 } }, scales: { y: { beginAtZero: true, ticks: { color: '#ffffff' }, grid: { color: 'rgba(255, 255, 255, 0.1)' } }, x: { ticks: { color: '#ffffff' }, grid: { color: 'rgba(255, 255, 255, 0.1)' } } } }
const monthlyChartOptions = baseChartOptions
const monthlyGenderChartOptions = { ...baseChartOptions, scales: { y: { beginAtZero: true, stacked: true, ticks: { color: '#ffffff' }, grid: { color: 'rgba(255, 255, 255, 0.1)' } }, x: { stacked: true, ticks: { color: '#ffffff' }, grid: { color: 'rgba(255, 255, 255, 0.1)' } } } }
const durationChartOptions = baseChartOptions
const pieChartOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'bottom' as const, labels: { color: '#ffffff', font: { size: 14 }, padding: 15 } }, tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.8)', titleColor: '#64ffda', bodyColor: '#ffffff', borderColor: '#64ffda', borderWidth: 1 } } }

const loadStats = async () => {
  loading.value = true
  error.value = null
  try {
    const server = serverStore.servers.find((s: any) => s.id === serverId)
    if (!server) { error.value = 'Serveur non trouvé'; return }
    serverName.value = server.name
    gameDomain.value = server.game_domain
    const response = await fetch(statsEndpoint.value)
    if (!response.ok) throw new Error(`Erreur HTTP: ${response.status}`)
    const data = await response.json()
    stats.value = data
    updateConnectedPlayers(normalizePlayers(data.connected_players))
    connectCentralPresenceSocket()
  } catch (err: any) {
    console.error('Error loading stats:', err)
    error.value = err.message || 'Impossible de charger les statistiques'
    stats.value = {}
    updateConnectedPlayers([])
    closeCentralPresenceSocket(false)
  } finally {
    loading.value = false
  }
}

const connectCentralPresenceSocket = () => {
  closeCentralPresenceSocket(false)
  const endpoint = resolveCentralPresenceSocketUrl()
  if (!endpoint) {
    centralPresenceStatus.value = 'error'
    activeCentralPresenceEndpoint.value = ''
    return
  }

  shouldReconnectCentralPresence = true
  activeCentralPresenceEndpoint.value = redactAuthQuery(endpoint)
  centralPresenceStatus.value = 'connecting'

  try {
    const socket = new WebSocket(endpoint)
    centralPresenceSocket = socket
    socket.onopen = () => { centralPresenceStatus.value = 'connected' }
    socket.onmessage = (event) => handleCentralPresenceMessage(event.data)
    socket.onerror = () => { centralPresenceStatus.value = 'error' }
    socket.onclose = () => {
      if (centralPresenceSocket === socket) centralPresenceSocket = null
      if (!shouldReconnectCentralPresence) {
        centralPresenceStatus.value = 'disconnected'
        return
      }
      centralPresenceStatus.value = 'connecting'
      centralPresenceReconnectTimer = window.setTimeout(connectCentralPresenceSocket, 1500)
    }
  } catch (err) {
    console.error('Unable to open central presence websocket:', err)
    centralPresenceStatus.value = 'error'
    centralPresenceReconnectTimer = window.setTimeout(connectCentralPresenceSocket, 1500)
  }
}

const closeCentralPresenceSocket = (disableReconnect = true) => {
  shouldReconnectCentralPresence = !disableReconnect
  if (centralPresenceReconnectTimer !== undefined) {
    window.clearTimeout(centralPresenceReconnectTimer)
    centralPresenceReconnectTimer = undefined
  }
  if (centralPresenceSocket) {
    centralPresenceSocket.close()
    centralPresenceSocket = null
  }
  if (disableReconnect) {
    centralPresenceStatus.value = 'disconnected'
    activeCentralPresenceEndpoint.value = ''
  }
}

const handleCentralPresenceMessage = (rawMessage: unknown) => {
  const message = parseMessage(rawMessage) as ServerPresenceSnapshotMessage | null
  if (!message || typeof message !== 'object') return
  if (message.type !== 'server_presence_snapshot') return

  const players = message.payload?.players
  if (Array.isArray(players)) updateConnectedPlayers(players)
  if (typeof message.payload?.current_connected_players === 'number') {
    stats.value = { ...stats.value, current_connected_players: message.payload.current_connected_players }
  }
}

const resolveCentralPresenceSocketUrl = () => {
  const token = localStorage.getItem(AUTH_TOKEN_STORAGE_KEY)
  if (!token) return ''
  const url = new URL(API_BASE_URL)
  url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:'
  url.pathname = `${url.pathname.replace(/\/+$/, '')}/friends/presence/realtime`
  url.searchParams.set('auth', token)
  url.searchParams.set('server_id', serverId)
  url.searchParams.set('include_all_players', 'true')
  return url.toString()
}

const parseMessage = (rawMessage: unknown) => {
  if (typeof rawMessage !== 'string') return rawMessage
  try { return JSON.parse(rawMessage) } catch { return null }
}
const updateConnectedPlayers = (players: unknown[]) => { const normalizedPlayers = normalizePlayers(players); liveConnectedPlayers.value = normalizedPlayers; stats.value = { ...stats.value, current_connected_players: normalizedPlayers.length, connected_players: normalizedPlayers }; refreshPlayerAvatars(normalizedPlayers) }
const normalizePlayers = (players: unknown) => Array.isArray(players) ? players.filter((player) => player !== null && player !== undefined) : []
const refreshPlayerAvatars = (players: unknown[]) => {
  const nextKeys = new Set(players.map(playerKey))
  for (const [key, url] of Object.entries(playerAvatarUrls.value)) if (!nextKeys.has(key)) { URL.revokeObjectURL(url); const nextUrls = { ...playerAvatarUrls.value }; delete nextUrls[key]; playerAvatarUrls.value = nextUrls }
  for (const player of players) { const key = playerKey(player); if (playerAvatarUrls.value[key]) continue; const source = playerProfilePictureSource(player); if (source) void loadPlayerAvatar(key, source) }
}
const loadPlayerAvatar = async (key: string, source: string) => { const token = localStorage.getItem(AUTH_TOKEN_STORAGE_KEY); const headers = new Headers(); if (token) headers.set('Authorization', `Bearer ${token}`); try { const response = await fetch(source, { headers, credentials: 'include' }); if (!response.ok) return; const blob = await response.blob(); const url = URL.createObjectURL(blob); if (playerAvatarUrls.value[key]) URL.revokeObjectURL(playerAvatarUrls.value[key]); playerAvatarUrls.value = { ...playerAvatarUrls.value, [key]: url } } catch (err) { console.warn('Unable to load player profile picture:', err) } }
const playerProfilePictureSource = (player: unknown) => { const centralUserId = playerCentralUserId(player); if (!centralUserId) return ''; return `${API_BASE_URL}/users/${encodeURIComponent(centralUserId)}/profile-pic.svg` }
const playerCentralUserId = (player: unknown) => { if (!player || typeof player !== 'object') return ''; const record = player as ConnectedPlayer; const value = record.user_id ?? record.userId ?? record.central_user_id ?? record.centralUserId ?? record.auth_user_id ?? record.authUserId; if (value === undefined || value === null) return ''; return String(value).trim() }
const monthLabel = (item: CountByMonth) => String(item.month ?? item.label ?? item.period ?? item.date ?? 'Non renseigné')
const genderValue = (item: CountByGender) => String(item.gender ?? item.label ?? item.name ?? item.avatar ?? item.avatar_kind ?? item.default_avatar ?? item.base_kind ?? 'Non renseigné')
const genderLabel = (item: CountByGender) => {
  const raw = genderValue(item)
  const normalized = raw.trim().toLowerCase()
  if (normalized === 'steve' || normalized === 'male' || normalized === 'man' || normalized === 'homme' || normalized === 'masculin') return 'Homme'
  if (normalized === 'alex' || normalized === 'female' || normalized === 'woman' || normalized === 'femme' || normalized === 'féminin' || normalized === 'feminin') return 'Femme'
  return raw
}
const numericCount = (item: CountByGender | CountByMonth | CountByMonthAndGender) => Number(item.count ?? item.total ?? item.connections ?? item.total_connections ?? 0)
const durationSeconds = (item: CountByGender) => Number(item.average_duration_seconds ?? item.count ?? item.total ?? 0)
const uniqueValues = (values: string[]) => Array.from(new Set(values))
const formatDuration = (seconds: number) => { const safeSeconds = Math.max(0, Math.floor(seconds)); const hours = Math.floor(safeSeconds / 3600); const minutes = Math.floor((safeSeconds % 3600) / 60); const remainingSeconds = safeSeconds % 60; if (hours > 0) return `${hours}h ${minutes}m`; if (minutes > 0) return `${minutes}m ${remainingSeconds}s`; return `${remainingSeconds}s` }
const playerLabel = (player: unknown) => { if (typeof player === 'string') return player; if (player && typeof player === 'object') { const record = player as ConnectedPlayer; return String(record.nickname ?? record.username ?? record.name ?? record.display_name ?? record.player_name ?? record.player_id ?? record.id ?? 'Joueur connecté') } return String(player) }
const playerKey = (player: unknown) => { const centralUserId = playerCentralUserId(player); if (centralUserId) return `user:${centralUserId}`; if (player && typeof player === 'object') { const record = player as ConnectedPlayer; return String(record.player_id ?? record.id ?? record.uuid ?? record.nickname ?? record.username ?? record.name ?? record.display_name ?? record.player_name ?? JSON.stringify(record)) } return String(player) }
const playerAvatarUrl = (player: unknown) => playerAvatarUrls.value[playerKey(player)] || ''
const playerIsFriend = (player: unknown) => player !== null && typeof player === 'object' && ((player as ConnectedPlayer).is_friend === true || (player as ConnectedPlayer).isFriend === true)
const redactAuthQuery = (endpoint: string) => endpoint.replace(/([?&]auth=)[^&]+/, '$1***')
const goBack = () => router.push({ name: 'servers' })
onMounted(async () => { if (serverStore.servers.length === 0) await serverStore.fetchUserServers(); await loadStats() })
onBeforeUnmount(() => { closeCentralPresenceSocket(true); for (const url of Object.values(playerAvatarUrls.value)) URL.revokeObjectURL(url); playerAvatarUrls.value = {} })
</script>

<style scoped>
.dashboard { min-height: calc(100vh - 80px); padding: 1.25rem; }
.voxicraft-container { max-width: 1680px; margin: 0 auto; }
.page-header { text-align: center; margin-bottom: 1.25rem; }
.header-top { display: flex; justify-content: flex-start; margin-bottom: 1rem; }
.back-button { background-color: #424242; border-color: #212121; padding: 0.5rem 1rem; font-size: 0.95rem; }
.subtitle { font-size: 1rem; opacity: 0.9; margin-top: 0.5rem; }
.loading-state, .error-state { text-align: center; padding: 3rem 2rem; max-width: 600px; margin: 0 auto; }
.loading-spinner, .error-icon { font-size: 3rem; margin-bottom: 1rem; }
.dashboard-content { display: grid; grid-template-columns: repeat(auto-fit, minmax(215px, 1fr)); gap: 1rem; align-items: stretch; }
.stats-cards { grid-column: 1 / -1; display: grid; grid-template-columns: repeat(auto-fit, minmax(210px, 1fr)); gap: 1rem; }
.stat-card { display: flex; align-items: center; gap: 0.75rem; padding: 1rem; max-width: none; margin: 0; }
.stat-icon { font-size: 2rem; }
.stat-value { font-size: 1.45rem; font-weight: bold; color: #64ffda; }
.stat-label { opacity: 0.85; margin-top: 0.2rem; font-size: 0.72rem; }
.charts-grid { display: contents; }
.chart-container, .info-section { width: 100%; max-width: none; min-width: 0; margin: 0; padding: 1rem; }
.chart-title, .info-section > h2 { margin-bottom: 0.8rem; color: #64ffda; font-size: 0.78rem; line-height: 1.35; }
.chart-wrapper { height: 205px; min-height: 0; }
.info-section { min-height: 205px; }
.section-header { display: flex; justify-content: space-between; gap: 0.65rem; align-items: flex-start; margin-bottom: 0.8rem; }
.section-header h2 { color: #64ffda; font-size: 0.78rem; line-height: 1.35; }
.players-list { display: flex; flex-direction: column; gap: 0.55rem; }
.player-item { display: flex; align-items: center; gap: 0.75rem; padding: 0.65rem; background: linear-gradient(135deg, rgba(100, 255, 218, 0.12), rgba(255, 255, 255, 0.06)); border: 1px solid rgba(100, 255, 218, 0.22); border-radius: 10px; box-shadow: 0 8px 18px rgba(0, 0, 0, 0.18); }
.player-avatar { width: 44px; height: 44px; flex: 0 0 44px; border-radius: 10px; overflow: hidden; border: 2px solid rgba(100, 255, 218, 0.55); background: rgba(0, 0, 0, 0.35); display: flex; align-items: center; justify-content: center; }
.player-avatar-image { width: 100%; height: 100%; object-fit: cover; image-rendering: pixelated; image-rendering: crisp-edges; }
.player-avatar-loading { width: 24px; height: 24px; border-radius: 6px; background: linear-gradient(135deg, rgba(100, 255, 218, 0.22), rgba(255, 255, 255, 0.08)); box-shadow: inset 0 0 0 2px rgba(100, 255, 218, 0.3); }
.player-main { min-width: 0; display: flex; flex-direction: column; gap: 0.25rem; }
.player-name-row { min-width: 0; display: inline-flex; align-items: center; gap: 0.4rem; }
.player-name { color: #ffffff; font-size: 0.72rem; font-weight: 700; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.friend-indicator { flex: 0 0 auto; font-size: 0.78rem; filter: drop-shadow(0 0 4px rgba(100, 255, 218, 0.45)); }
.info-grid { display: grid; grid-template-columns: 1fr; gap: 0.75rem; }
.info-item { display: flex; flex-direction: column; gap: 0.35rem; word-break: break-word; font-size: 0.68rem; line-height: 1.45; }
.info-item strong { color: #64ffda; }
.no-data { display: flex; align-items: center; justify-content: center; height: 100%; opacity: 0.75; font-size: 0.72rem; text-align: center; }
.no-players { min-height: 120px; }
@media (min-width: 1440px) { .dashboard-content { grid-template-columns: repeat(6, minmax(0, 1fr)); } }
@media (max-width: 1200px) { .dashboard-content { grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); } .chart-wrapper, .info-section { height: auto; min-height: 240px; } }
@media (max-width: 768px) { .dashboard { padding: 1rem; } .voxicraft-container { max-width: 100%; } .dashboard-content, .stats-cards { grid-template-columns: 1fr; } .chart-container, .info-section, .stat-card { padding: 1rem; } .chart-wrapper { height: 250px; } .section-header { flex-direction: column; align-items: flex-start; } }
</style>
