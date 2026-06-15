<template>
  <main class="api-docs voxicraft-bg">
    <section class="voxicraft-panel docs-panel">
      <header class="docs-header">
        <h1 class="voxicraft-title">Documentation Voxicraft</h1>
        <p class="voxicraft-text">Guide d'installation d'un serveur de jeu, documentation API centrale et documentation expérimentale des mods.</p>
        <nav class="docs-tabs" aria-label="Sections de documentation">
          <a href="#quick-start">Quick start serveur</a>
          <a href="#api-doc">API centrale</a>
          <a href="#mods-doc">Mods</a>
        </nav>
      </header>

      <section id="quick-start" class="docs-section highlight-section">
        <div class="section-heading">
          <span class="section-kicker">Quick start</span>
          <h2>Installer un serveur de jeu Voxicraft</h2>
        </div>
        <div class="quick-start-grid">
          <article class="doc-card primary-card">
            <h3>1. Télécharger la dernière version</h3>
            <p>Récupère toujours la dernière release publiée, sans dépendre du nom ou du numéro de version.</p>
            <a class="download-link" href="https://github.com/nicolachoquet06250/minecraft-webxr/releases/latest" target="_blank" rel="noopener noreferrer">
              Télécharger la dernière version
            </a>
          </article>
          <article class="doc-card">
            <h3>2. Extraire l'archive</h3>
            <p>Décompresse l'archive sur la machine qui hébergera le serveur de jeu.</p>
            <pre>tar -xzf voxicraft-server-*.tar.gz
cd voxicraft-server</pre>
          </article>
          <article class="doc-card">
            <h3>3. Configurer le serveur</h3>
            <p>Prépare le fichier d'environnement avec l'URL publique du serveur et les paramètres de connexion au serveur central.</p>
            <pre>cp .env.example .env
nano .env</pre>
          </article>
          <article class="doc-card">
            <h3>4. Lancer le binaire</h3>
            <p>Démarre le serveur. Le serveur central pourra ensuite le référencer via son domaine public.</p>
            <pre>chmod +x voxicraft_server
./voxicraft_server</pre>
          </article>
        </div>
      </section>

      <section id="api-doc" class="docs-section">
        <div class="section-heading">
          <span class="section-kicker">API</span>
          <h2>Documentation de l'API centrale</h2>
        </div>

        <article class="doc-card">
          <h3>Authentification</h3>
          <p>Les routes protégées utilisent un JWT envoyé dans l'en-tête HTTP suivant :</p>
          <pre>Authorization: Bearer &lt;token&gt;</pre>
          <p>Les pages Swagger et documentation sont protégées côté routeur front : sans session active, l'utilisateur est redirigé vers la page de login.</p>
        </article>

        <div class="resource-grid">
          <article v-for="resource in resources" :key="resource.name" class="doc-card">
            <h3>{{ resource.name }}</h3>
            <p>{{ resource.description }}</p>
            <ul>
              <li v-for="field in resource.fields" :key="field"><code>{{ field }}</code></li>
            </ul>
          </article>
        </div>

        <div class="schema-grid">
          <article v-for="schema in schemas" :key="schema.name" class="doc-card schema-card">
            <h3>{{ schema.name }}</h3>
            <pre>{{ schema.shape }}</pre>
          </article>
        </div>
      </section>

      <section id="mods-doc" class="docs-section">
        <div class="section-heading">
          <span class="section-kicker">Expérimental</span>
          <h2>Documentation des mods</h2>
        </div>
        <div class="resource-grid">
          <article class="doc-card">
            <h3>Manifest</h3>
            <p>Chaque mod peut déclarer un fichier <code>mod.json</code>. Le serveur expose la liste des mods via l'endpoint de manifest.</p>
            <pre>GET /api/mods/manifest</pre>
          </article>
          <article class="doc-card">
            <h3>Fichiers client</h3>
            <p>Les fichiers client d'un mod sont servis publiquement pour permettre leur chargement depuis le navigateur, y compris en solo.</p>
            <pre>GET /mods/:id/client/...</pre>
          </article>
          <article class="doc-card">
            <h3>Structure recommandée</h3>
            <p>Un mod peut fournir du JavaScript client, des assets et, plus tard, une partie serveur WebAssembly.</p>
            <pre>mods/example-mod/
├── mod.json
├── client/mod.js
└── assets/...</pre>
          </article>
          <article class="doc-card">
            <h3>Statut</h3>
            <p>Le système de mods est expérimental. Les clés optionnelles comme <code>assets</code> doivent être tolérées lorsqu'elles sont absentes.</p>
          </article>
        </div>
      </section>
    </section>
  </main>
</template>

<script setup lang="ts">
const resources = [
  {
    name: 'User',
    description: 'Compte utilisateur central utilisé pour l’authentification, le profil et les avatars.',
    fields: ['id', 'username', 'email', 'avatar', 'bio?', 'birthdate', 'age_verified', 'discord_username?', 'created_at'],
  },
  {
    name: 'Server',
    description: 'Serveur de jeu enregistré par un utilisateur.',
    fields: ['id', 'owner_id', 'name', 'game_domain', 'description?', 'is_active', 'created_at', 'updated_at'],
  },
  {
    name: 'UserAvatar',
    description: 'Avatar personnalisé stocké sous forme de matrices de couleurs.',
    fields: ['id', 'name', 'base_kind', 'is_active', 'texture_data', 'created_at', 'updated_at'],
  },
  {
    name: 'AvatarTextureData',
    description: 'Structure sérialisée des textures pixel art de chaque partie du corps.',
    fields: ['version', 'palette', 'parts.head', 'parts.torso', 'parts.rightArm', 'parts.leftArm', 'parts.rightLeg', 'parts.leftLeg'],
  },
]

const schemas = [
  {
    name: 'RegisterData',
    shape: `{
  username: string
  email: string
  password: string
  avatar: 'steve' | 'alex'
  birthdate: string
  bio?: string
}`,
  },
  {
    name: 'SaveAvatarData',
    shape: `{
  name: string
  base_kind: 'steve' | 'alex' | 'custom'
  texture_data: AvatarTextureData
}`,
  },
  {
    name: 'AvatarTextureData',
    shape: `{
  version: 1
  palette: Record<string, [number, number, number, number]>
  parts: {
    [partName]: {
      [faceName]: {
        width: number
        height: number
        matrix: string[]
      }
    }
  }
}`,
  },
  {
    name: 'CreateServerData',
    shape: `{
  name: string
  game_domain: string
  description?: string
}`,
  },
]
</script>

<style scoped>
.api-docs { min-height: calc(100vh - 80px); padding: 2rem 1rem; }
.docs-panel { max-width: 1400px; margin: 0 auto; padding: 2rem; }
.docs-header { text-align: center; margin-bottom: 2rem; }
.docs-tabs { display: flex; flex-wrap: wrap; justify-content: center; gap: .75rem; margin-top: 1.2rem; }
.docs-tabs a, .download-link { color: #1a1a1a; background: #64ffda; border: 2px solid #1a1a1a; border-radius: 8px; padding: .75rem 1rem; text-decoration: none; font-weight: 700; box-shadow: 3px 3px 0 rgba(0,0,0,.35); }
.docs-section { margin-top: 2.5rem; scroll-margin-top: 90px; }
.highlight-section { border-top: 2px solid rgba(100, 255, 218, .3); padding-top: 2rem; }
.section-heading { margin-bottom: 1rem; }
.section-kicker { color: #ffd700; text-transform: uppercase; font-size: .75rem; letter-spacing: .08em; }
.docs-section h2 { color: #64ffda; margin: .25rem 0 1rem; }
.quick-start-grid, .resource-grid, .schema-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1rem; }
.doc-card { background: rgba(0, 0, 0, .28); border: 1px solid rgba(100, 255, 218, .18); border-radius: 12px; padding: 1rem; min-width: 0; }
.primary-card { border-color: rgba(255, 215, 0, .55); }
h3 { color: #ffd700; margin-bottom: .75rem; }
p, li { color: rgba(255, 255, 255, .82); line-height: 1.6; }
ul, ol { padding-left: 1.25rem; }
code { color: #64ffda; }
pre { overflow: auto; background: rgba(0,0,0,.55); color: #d8fff6; border-radius: 10px; padding: 1rem; font-size: .85rem; }
.download-link { display: inline-flex; margin-top: .75rem; }
@media (max-width: 768px) { .api-docs { padding: 1rem .55rem; } .docs-panel { padding: 1rem; } .quick-start-grid, .resource-grid, .schema-grid { grid-template-columns: 1fr; } .docs-tabs { flex-direction: column; } }
</style>
