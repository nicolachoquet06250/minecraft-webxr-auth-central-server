<template>
  <div class="servers voxicraft-bg">
    <div class="voxicraft-container">
      <div class="page-header">
        <h1 class="voxicraft-title">🖥️ Mes Serveurs</h1>
        <p class="voxicraft-text subtitle">
          Gérez vos serveurs de jeu
        </p>
      </div>

      <div class="servers-header">
        <button @click="showCreateForm = !showCreateForm" class="voxicraft-button create-btn">
          {{ showCreateForm ? '❌ Annuler' : '➕ Créer un serveur' }}
        </button>
        <router-link to="/servers/favorites" class="voxicraft-button nav-btn">⭐ Serveurs favoris</router-link>
        <router-link to="/servers/recent" class="voxicraft-button nav-btn">🕘 Récemment visités</router-link>
      </div>

      <div v-if="showCreateForm" class="create-form voxicraft-panel">
        <h2>➕ Nouveau Serveur</h2>
        <p class="form-description voxicraft-text">
          Enregistrez un nouveau serveur de jeu
        </p>
        <form @submit.prevent="handleCreate">
          <div class="form-group">
            <label class="voxicraft-label">Nom du serveur</label>
            <input
              v-model="createData.name"
              type="text"
              class="voxicraft-input"
              required
              minlength="3"
              maxlength="50"
            />
          </div>

          <div class="form-group">
            <label class="voxicraft-label">Domaine du serveur de jeu</label>
            <input
              v-model="createData.game_domain"
              type="url"
              class="voxicraft-input"
              required
              placeholder="https://game.example.com"
            />
            <small class="voxicraft-text">URL du serveur de jeu exposant le jeu, les statistiques et le WebSocket</small>
          </div>

          <div class="form-group">
            <label class="voxicraft-label">Description</label>
            <textarea
              v-model="createData.description"
              class="voxicraft-input"
              rows="3"
            ></textarea>
          </div>

          <button type="submit" class="voxicraft-button" :disabled="serverStore.loading">
            Créer
          </button>

          <div v-if="serverStore.error" class="error-message">
            {{ serverStore.error }}
          </div>
        </form>
      </div>

      <div class="servers-list">
        <div v-if="serverStore.loading" class="voxicraft-text loading-message">
          ⏳ Chargement...
        </div>

        <div v-else-if="serverStore.servers.length === 0" class="empty-state voxicraft-panel">
          <div class="empty-icon">🏗️</div>
          <h3>Aucun serveur enregistré</h3>
          <p class="voxicraft-text">
            Commencez par créer votre premier serveur pour gérer votre expérience Minecraft WebXR
          </p>
          <button @click="showCreateForm = true" class="voxicraft-button">
            ➕ Créer mon premier serveur
          </button>
        </div>

        <div v-else class="server-grid">
          <div
            v-for="server in serverStore.servers"
            :key="server.id"
            class="server-card voxicraft-panel"
            @click="goToDashboard(server.id)"
          >
            <h3>{{ server.name }}</h3>
            <p class="server-url-row"><strong>Serveur de jeu:</strong> <a :href="server.game_domain" target="_blank" class="server-url" :title="server.game_domain" @click.stop="openGameServer(server)">{{ server.game_domain }}</a></p>
            <p v-if="server.description"><strong>Description:</strong> {{ server.description }}</p>
            <p><strong>Status:</strong> {{ server.is_active ? '✅ Actif' : '🔴 Inactif' }}</p>

            <div class="button-container">
              <button
                @click.stop="goToDashboard(server.id)"
                class="voxicraft-button small primary"
              >
                📊 Dashboard
              </button>
              <button
                @click.stop="toggleServerStatus(server)"
                class="voxicraft-button small"
              >
                {{ server.is_active ? 'Désactiver' : 'Activer' }}
              </button>
              <button
                @click.stop="deleteServerConfirm(server.id)"
                class="voxicraft-button small danger"
              >
                Supprimer
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import type { Server } from '@/api'
import { useServerStore } from '@/stores/server'

const router = useRouter()
const serverStore = useServerStore()
const showCreateForm = ref(false)
const createData = ref({
  name: '',
  game_domain: '',
  description: '',
})

onMounted(async () => {
  await serverStore.fetchUserServers()
})

const handleCreate = async () => {
  const success = await serverStore.createServer(createData.value)
  if (success) {
    const newServer = serverStore.servers[serverStore.servers.length - 1]
    createData.value = {
      name: '',
      game_domain: '',
      description: '',
    }
    showCreateForm.value = false

    router.push({ name: 'server-dashboard', params: { id: newServer.id } })
  }
}

const toggleServerStatus = async (server: Server) => {
  await serverStore.updateServer(server.id, {
    name: server.name,
    description: server.description,
    is_active: !server.is_active,
  } as any)
}

const deleteServerConfirm = async (id: string) => {
  if (confirm('Êtes-vous sûr de vouloir supprimer ce serveur ?')) {
    await serverStore.deleteServer(id)
  }
}

const goToDashboard = (serverId: string) => {
  router.push({ name: 'server-dashboard', params: { id: serverId } })
}

const openGameServer = async (server: Server) => {
  await serverStore.recordServerVisit(server.game_domain)
}
</script>

<style scoped>
.servers {
  min-height: calc(100vh - 80px);
  padding: 2rem 1rem;
}

.voxicraft-container {
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
  min-width: 0;
  box-sizing: border-box;
}

.page-header {
  text-align: center;
  margin-bottom: 2rem;
}

.subtitle {
  font-size: 1.1rem;
  opacity: 0.9;
  margin-top: 0.5rem;
}

.servers-header {
  margin: 2rem 0;
  display: flex;
  justify-content: center;
  align-items: center;
  gap: .75rem;
  flex-wrap: wrap;
  max-width: 100%;
}

.create-btn {
  background-color: #4caf50;
  border-color: #2e7d32;
  font-size: 1.1rem;
  padding: 0.75rem 1.5rem;
}

.create-btn:hover {
  background-color: #66bb6a;
}

.nav-btn {
  background-color: #6d4c41;
  border-color: #4e342e;
  font-size: .95rem;
  padding: .75rem 1.1rem;
}

.create-form {
  margin: 0 auto 2rem;
  max-width: 700px;
  padding: 2rem;
  box-sizing: border-box;
}

.create-form h2 {
  margin-bottom: 0.5rem;
}

.form-description {
  margin-bottom: 1.5rem;
  opacity: 0.8;
}

.form-group {
  display: flex;
  flex-direction: column;
  text-align: left;
  margin-bottom: 1.5rem;
}

.form-group small {
  margin-top: 0.5rem;
  font-size: 0.9rem;
  opacity: 0.7;
  line-height: 1.4;
}

.servers-list {
  margin-top: 2rem;
  width: 100%;
  min-width: 0;
  max-width: 100%;
}

.loading-message {
  text-align: center;
  font-size: 1.2rem;
  padding: 2rem;
}

.empty-state {
  text-align: center;
  padding: 3rem 2rem;
  max-width: 600px;
  margin: 0 auto;
  box-sizing: border-box;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.server-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(380px, 100%), 1fr));
  gap: 2rem;
  width: 100%;
  max-width: 100%;
  min-width: 0;
  box-sizing: border-box;
}

.server-card {
  padding: 2rem;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: left;
  min-width: 0;
  max-width: 100%;
  width: 100%;
  box-sizing: border-box;
  overflow: hidden;
}

.server-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
}

.server-card h3 {
  color: #64ffda;
  margin-bottom: 1rem;
  font-size: 1.4rem;
  overflow-wrap: anywhere;
}

.server-card p {
  margin-bottom: 0.5rem;
  color: #d7ccc8;
  overflow-wrap: anywhere;
}

.server-url-row {
  display: flex;
  align-items: baseline;
  gap: .4rem;
  min-width: 0;
  max-width: 100%;
}

.server-url-row strong {
  flex: 0 0 auto;
}

.server-url {
  display: inline-block;
  min-width: 0;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  vertical-align: bottom;
}

.server-card a {
  color: #64ffda;
  text-decoration: none;
}

.server-card a:hover {
  text-decoration: underline;
}

.button-container {
  display: flex;
  gap: 0.5rem;
  margin-top: 1.5rem;
  flex-wrap: wrap;
  max-width: 100%;
}

.voxicraft-button.small {
  padding: 0.5rem 1rem;
  font-size: 0.8rem;
}

.voxicraft-button.primary {
  background-color: #2196f3;
  border-color: #1565c0;
}

.voxicraft-button.danger {
  background-color: #f44336;
  border-color: #c62828;
}

.error-message {
  color: #ff6b6b;
  background: rgba(255, 107, 107, 0.1);
  border: 1px solid #ff6b6b;
  padding: 0.5rem;
  margin-top: 1rem;
  border-radius: 4px;
}

@media (max-width: 768px) {
  .servers {
    padding: 1rem .55rem;
    max-width: 100vw;
    overflow-x: hidden;
  }

  .voxicraft-container,
  .servers-list,
  .server-grid,
  .server-card,
  .create-form,
  .empty-state {
    width: 100%;
    max-width: 100%;
    min-width: 0;
    box-sizing: border-box;
  }

  .server-grid {
    grid-template-columns: minmax(0, 1fr);
    gap: 1rem;
  }

  .server-card {
    padding: 1rem;
  }

  .server-card:hover {
    transform: none;
  }

  .server-url-row {
    display: block;
  }

  .server-url {
    display: block;
  }

  .button-container,
  .servers-header {
    flex-direction: column;
    align-items: stretch;
  }

  .button-container .voxicraft-button,
  .servers-header .voxicraft-button {
    width: 100%;
    max-width: 100%;
    box-sizing: border-box;
  }
}
</style>