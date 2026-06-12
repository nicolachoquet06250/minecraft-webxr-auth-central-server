<template>
  <div class="servers voxicraft-bg">
    <div class="voxicraft-container">
      <div class="page-header">
        <h1 class="voxicraft-title">🖥️ Mes Serveurs</h1>
        <p class="voxicraft-text subtitle">
          Gérez vos serveurs de jeu et serveurs relais
        </p>
      </div>
      
      <div class="servers-header">
        <button @click="showCreateForm = !showCreateForm" class="voxicraft-button create-btn">
          {{ showCreateForm ? '❌ Annuler' : '➕ Créer un serveur' }}
        </button>
      </div>
      
      <div v-if="showCreateForm" class="create-form voxicraft-panel">
        <h2>➕ Nouveau Serveur</h2>
        <p class="form-description voxicraft-text">
          Enregistrez un nouveau serveur avec ses domaines de relais et de jeu
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
            <label class="voxicraft-label">Domaine du serveur relais (WebSocket)</label>
            <input 
              v-model="createData.relay_domain" 
              type="url" 
              class="voxicraft-input" 
              required
              placeholder="wss://relay.example.com"
            />
            <small class="voxicraft-text">URL du serveur de relais pour les connexions WebSocket</small>
          </div>
          
          <div class="form-group">
            <label class="voxicraft-label">Domaine du jeu (Frontend)</label>
            <input 
              v-model="createData.game_domain" 
              type="url" 
              class="voxicraft-input" 
              required
              placeholder="https://game.example.com"
            />
            <small class="voxicraft-text">URL où le jeu WebXR est hébergé</small>
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
            <p><strong>Serveur relais:</strong> <a :href="server.relay_domain" target="_blank" @click.stop>{{ server.relay_domain }}</a></p>
            <p><strong>Jeu (Frontend):</strong> <a :href="server.game_domain" target="_blank" @click.stop>{{ server.game_domain }}</a></p>
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
import { useServerStore } from '@/stores/server'
import type { Server } from '@/api'

const router = useRouter()
const serverStore = useServerStore()
const showCreateForm = ref(false)
const createData = ref({
  name: '',
  relay_domain: '',
  game_domain: '',
  description: '',
})

onMounted(() => {
  serverStore.fetchUserServers()
})

const handleCreate = async () => {
  const success = await serverStore.createServer(createData.value)
  if (success) {
    const newServer = serverStore.servers[serverStore.servers.length - 1]
    createData.value = {
      name: '',
      relay_domain: '',
      game_domain: '',
      description: '',
    }
    showCreateForm.value = false
    
    // Redirect to dashboard after creation
    router.push({ name: 'server-dashboard', params: { id: newServer.id } })
  }
}

const toggleServerStatus = async (server: Server) => {
  await serverStore.updateServer(server.id, {
    name: server.name,
    relay_domain: server.relay_domain,
    game_domain: server.game_domain,
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
</script>

<style scoped>
.servers {
  min-height: calc(100vh - 80px);
  padding: 2rem 1rem;
}

.voxicraft-container {
  max-width: 1200px;
  margin: 0 auto;
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

.create-form {
  margin: 0 auto 2rem;
  max-width: 700px;
  padding: 2rem;
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
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.empty-state h3 {
  font-size: 1.5rem;
  margin-bottom: 1rem;
}

.empty-state p {
  margin-bottom: 2rem;
  line-height: 1.6;
}

.server-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(450px, 1fr));
  gap: 2rem;
}

.server-card {
  padding: 2rem;
  cursor: pointer;
}

.server-card h3 {
  margin-bottom: 1rem;
  font-size: 1.3rem;
  color: #64ffda;
}

.server-card p {
  margin: 0.75rem 0;
  word-break: break-word;
  line-height: 1.5;
}

.server-card strong {
  color: #64ffda;
}

.server-card a {
  color: #fff;
  text-decoration: none;
}

.button-container {
  display: flex;
  gap: 0.75rem;
  margin-top: 1.5rem;
  flex-wrap: wrap;
}

.voxicraft-button.small {
  padding: 0.6rem 1rem;
  font-size: 0.9rem;
}

.voxicraft-button.small.primary {
  background-color: #64ffda;
  color: #1a1a1a;
  border-color: #4dd0ba;
  font-weight: bold;
}

.voxicraft-button.small.primary:hover {
  background-color: #7fffeb;
}

.voxicraft-button.danger {
  background-color: #d32f2f;
  border-color: #b71c1c;
}

.voxicraft-button.danger:hover {
  background-color: #f44336;
}

.error-message {
  color: #ff4444;
  margin-top: 1rem;
  background-color: rgba(255, 68, 68, 0.1);
  padding: 0.75rem;
  border-radius: 4px;
  border: 2px solid #ff4444;
}

@media (max-width: 768px) {
  .servers {
    padding: 1rem;
  }
  
  .server-grid {
    grid-template-columns: 1fr;
  }
  
  .create-form {
    padding: 1.5rem;
  }
}
</style>
