<template>
  <main class="documentation voxicraft-bg">
    <section class="voxicraft-panel documentation-panel">
      <header class="documentation-header">
        <h1 class="voxicraft-title">Documentation Voxicraft</h1>
        <p class="documentation-intro">Documentation organisée en pages imbriquées.</p>
      </header>

      <div class="documentation-layout">
        <aside class="documentation-menu">
          <button :class="tabClass('quick-start')" type="button" @click="activeTab = 'quick-start'">Quick start serveur</button>
          <button :class="tabClass('api-doc')" type="button" @click="activeTab = 'api-doc'">API centrale</button>
          <button :class="tabClass('mods-doc')" type="button" @click="activeTab = 'mods-doc'">Mods</button>
        </aside>

        <section class="documentation-content">
          <div v-if="activeTab === 'quick-start'" class="documentation-section">
            <span class="section-kicker">Quick start</span>
            <h2>Installer un serveur de jeu</h2>
            <div class="doc-grid">
              <article class="doc-card important-card">
                <h3>1. Télécharger</h3>
                <p>Récupère toujours la dernière release publiée.</p>
                <a class="download-link" href="https://github.com/nicolachoquet06250/minecraft-webxr/releases/latest" target="_blank" rel="noopener noreferrer">Télécharger la dernière version</a>
              </article>
              <article class="doc-card"><h3>2. Extraire</h3><p>Décompresse l'archive sur la machine serveur.</p><pre>tar -xzf voxicraft-server-*.tar.gz
cd voxicraft-server</pre></article>
              <article class="doc-card"><h3>3. Configurer</h3><p>Prépare le fichier d'environnement.</p><pre>cp .env.example .env
nano .env</pre></article>
              <article class="doc-card"><h3>4. Lancer</h3><p>Démarre le binaire serveur.</p><pre>chmod +x voxicraft_server
./voxicraft_server</pre></article>
            </div>
          </div>

          <div v-else-if="activeTab === 'api-doc'" class="documentation-section">
            <span class="section-kicker">API</span>
            <h2>Documentation de l'API centrale</h2>
            <p>Cette section lit uniquement <code>/api/openapi.json</code>. Elle n'utilise pas Swagger UI.</p>
            <article class="doc-card auth-card"><h3>Authentification</h3><pre>Authorization: Bearer &lt;token&gt;</pre></article>
            <article v-if="apiLoading" class="doc-card">Chargement de /api/openapi.json...</article>
            <article v-else-if="apiError" class="doc-card warning-card">{{ apiError }}</article>
            <div v-else class="api-route-list">
              <article v-for="route in apiRoutes" :key="route.method + route.path" class="doc-card route-card">
                <div class="route-heading"><span class="method-badge" :class="route.method.toLowerCase()">{{ route.method }}</span><code>{{ route.path }}</code></div>
                <p v-if="route.operation.summary">{{ route.operation.summary }}</p>
                <div class="route-block"><h4>Paramètres d'URL</h4><p v-if="!urlParameters(route.operation).length" class="empty-note">Aucun paramètre d'URL documenté.</p><div v-else class="table-scroll"><table class="param-table"><thead><tr><th>Nom</th><th>Position</th><th>Type</th><th>Obligatoire</th><th>Regex / contrainte</th></tr></thead><tbody><tr v-for="parameter in urlParameters(route.operation)" :key="parameter.name + parameter.in"><td><code>{{ parameter.name }}</code></td><td>{{ parameter.in }}</td><td><code>{{ schemaType(parameter.schema) }}</code></td><td>{{ parameter.required ? 'Oui' : 'Non' }}</td><td><code>{{ schemaConstraint(parameter.schema) || '—' }}</code></td></tr></tbody></table></div></div>
                <div class="route-block"><h4>Format de réponse</h4><pre>{{ responseFormat(route.operation) }}</pre></div>
              </article>
            </div>
          </div>

          <div v-else class="documentation-section">
            <span class="section-kicker">Mods</span>
            <h2>Créer un mod Voxicraft</h2>
            <p>Un mod Voxicraft est une extension placée dans le dossier <code>mods/</code> d'un serveur de jeu. Cette documentation décrit ce qu'il faut créer, comment déclarer le mod, et quelles APIs tu peux utiliser côté navigateur et côté serveur.</p>
            <p><strong>Statut :</strong> le système de mods est encore expérimental. Le format peut évoluer, mais la structure ci-dessous est celle à utiliser pour créer un mod aujourd'hui.</p>

            <div class="mods-list">
              <article class="doc-card">
                <h3>1. Créer le dossier du mod</h3>
                <p>Crée un dossier par mod. L'identifiant du dossier doit être stable, sans espace, sans slash et sans <code>..</code>.</p>
                <pre>mods/
  ruby-tools/
    mod.json
    client/
      mod.js
      mod.d.ts
      assets/
        ruby_ore.png
    server/
      mod.wasm</pre>
              </article>

              <article class="doc-card">
                <h3>2. Déclarer <code>mod.json</code></h3>
                <p>Le manifest décrit le nom du mod, sa version, les fichiers à charger et les permissions demandées. Un mod peut être <code>client</code>, <code>server</code> ou <code>both</code>.</p>
                <pre>{
  "id": "ruby-tools",
  "name": "Ruby Tools",
  "version": "0.1.0",
  "side": "both",
  "client": {
    "runtime": "javascript",
    "entry": "client/mod.js",
    "types": "client/mod.d.ts",
    "assets": "client/assets"
  },
  "server": {
    "runtime": "wasm",
    "entry": "server/mod.wasm"
  },
  "permissions": {
    "client": [
      "babylon.scene",
      "game.player.read",
      "game.events",
      "ui.notify"
    ],
    "server": [
      "world.read",
      "world.write"
    ]
  }
}</pre>
              </article>

              <article class="doc-card">
                <h3>Champs du manifest</h3>
                <div class="table-scroll">
                  <table class="param-table">
                    <thead><tr><th>Champ</th><th>Obligatoire</th><th>Description</th></tr></thead>
                    <tbody>
                      <tr><td><code>id</code></td><td>Oui</td><td>Identifiant unique du mod.</td></tr>
                      <tr><td><code>name</code></td><td>Oui</td><td>Nom affiché dans les outils et messages.</td></tr>
                      <tr><td><code>version</code></td><td>Oui</td><td>Version du mod, idéalement en SemVer.</td></tr>
                      <tr><td><code>side</code></td><td>Oui</td><td><code>client</code>, <code>server</code> ou <code>both</code>.</td></tr>
                      <tr><td><code>client.entry</code></td><td>Pour un mod client</td><td>Fichier JavaScript chargé par le navigateur.</td></tr>
                      <tr><td><code>client.types</code></td><td>Non</td><td>Fichier de types TypeScript pour l'autocomplétion.</td></tr>
                      <tr><td><code>client.assets</code></td><td>Non</td><td>Dossier public des images, sons, modèles ou textures du mod.</td></tr>
                      <tr><td><code>server.entry</code></td><td>Pour un mod serveur</td><td>Fichier WebAssembly serveur.</td></tr>
                      <tr><td><code>permissions</code></td><td>Non</td><td>Liste les APIs demandées par le mod.</td></tr>
                    </tbody>
                  </table>
                </div>
              </article>

              <article class="doc-card">
                <h3>3. Écrire un mod client</h3>
                <p>Le fichier déclaré dans <code>client.entry</code> doit exporter <code>activate(ctx)</code>. Il peut aussi exporter <code>deactivate(ctx)</code> pour nettoyer les ressources du mod.</p>
                <pre>export async function activate(ctx) {
  ctx.ui.notify('Ruby Tools chargé')

  const observer = ctx.scene.onBeforeRenderObservable.add(() =&gt; {
    // logique exécutée à chaque frame
  })

  ctx.addDisposable({
    dispose: () =&gt; ctx.scene.onBeforeRenderObservable.remove(observer)
  })
}

export async function deactivate(ctx) {
  ctx.ui.notify('Ruby Tools déchargé')
}</pre>
              </article>

              <article class="doc-card">
                <h3>API front disponible</h3>
                <p>Dans <code>activate(ctx)</code>, le paramètre <code>ctx</code> expose les APIs utilisables côté navigateur.</p>
                <pre>ctx.BABYLON
ctx.scene
ctx.engine
ctx.player
ctx.worldChunks
ctx.droppedItems
ctx.events
ctx.ui.notify(message)
ctx.resolveAssetUrl(path)
ctx.addDisposable(disposable)</pre>
              </article>

              <article class="doc-card">
                <h3>API front — Babylon.js</h3>
                <p>Utilise <code>ctx.BABYLON</code>, <code>ctx.scene</code> et <code>ctx.engine</code> pour créer des meshes, matériaux, particules ou observers Babylon.js.</p>
                <pre>export async function activate(ctx) {
  const mesh = ctx.BABYLON.MeshBuilder.CreateBox(
    'ruby-tools-marker',
    { size: 0.25 },
    ctx.scene
  )

  mesh.position.set(0, 2, 0)
  ctx.addDisposable(mesh)
}</pre>
              </article>

              <article class="doc-card">
                <h3>API front — assets du mod</h3>
                <p><code>ctx.resolveAssetUrl(path)</code> retourne l'URL publique d'un fichier situé dans <code>client.assets</code>.</p>
                <pre>export async function activate(ctx) {
  const textureUrl = ctx.resolveAssetUrl('ruby_ore.png')
  const texture = new ctx.BABYLON.Texture(textureUrl, ctx.scene)

  ctx.addDisposable(texture)
}</pre>
              </article>

              <article class="doc-card">
                <h3>API front — interface</h3>
                <p><code>ctx.ui.notify(message)</code> affiche une notification simple dans le jeu.</p>
                <pre>export async function activate(ctx) {
  ctx.ui.notify('Nouveau minerai disponible : rubis')
}</pre>
              </article>

              <article class="doc-card">
                <h3>API front — évènements et nettoyage</h3>
                <p><code>ctx.events</code> sert à brancher la logique du mod sur les évènements exposés par le jeu. Tout listener, mesh, observer ou texture créé par le mod doit être libéré via <code>ctx.addDisposable</code> ou <code>deactivate</code>.</p>
                <pre>export async function activate(ctx) {
  const unsubscribe = ctx.events.on('ready', () =&gt; {
    ctx.ui.notify('Le monde est prêt')
  })

  ctx.addDisposable({ dispose: unsubscribe })
}</pre>
              </article>

              <article class="doc-card">
                <h3>API back disponible</h3>
                <p>Un mod serveur se déclare avec <code>server.runtime = "wasm"</code> et <code>server.entry</code>. Les permissions serveur actuellement prévues sont :</p>
                <pre>world.read
world.write</pre>
                <p>Ces permissions servent à documenter ce que le mod serveur veut faire côté monde. L'exécution complète des mods serveur WebAssembly reste expérimentale : ne pars pas du principe qu'un mod serveur dispose déjà des mêmes capacités qu'un mod client.</p>
              </article>

              <article class="doc-card">
                <h3>Mod client uniquement</h3>
                <p>Pour créer un mod qui ne tourne que dans le navigateur, utilise <code>side: "client"</code> et retire le bloc <code>server</code>.</p>
                <pre>{
  "id": "hud-helper",
  "name": "HUD Helper",
  "version": "0.1.0",
  "side": "client",
  "client": {
    "runtime": "javascript",
    "entry": "client/mod.js",
    "assets": "client/assets"
  }
}</pre>
              </article>

              <article class="doc-card">
                <h3>Tester rapidement le mod</h3>
                <p>Lance le serveur avec le dossier <code>mods/</code>, puis vérifie que le manifest et le fichier client sont accessibles.</p>
                <pre>MODS_DIR=mods ./voxicraft_server

curl -i http://localhost:8080/api/mods/manifest
curl -i http://localhost:8080/mods/ruby-tools/client/mod.js</pre>
              </article>

              <article class="doc-card">
                <h3>Bonnes pratiques</h3>
                <p>Préfixe les IDs avec l'ID du mod, garde un manifest simple, libère toutes les ressources créées, évite les traitements lourds à chaque frame, et déclare uniquement les permissions réellement nécessaires.</p>
              </article>
            </div>
          </div>
        </section>
      </div>
    </section>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'

type JsonMap = Record<string, unknown>
type OpenApiParameter = { name: string; in: string; required?: boolean; schema?: JsonMap }
type ApiRoute = { path: string; method: string; operation: JsonMap }

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'
const activeTab = ref<'quick-start' | 'api-doc' | 'mods-doc'>('quick-start')
const apiSpec = ref<JsonMap | null>(null)
const apiLoading = ref(true)
const apiError = ref('')

const apiRoutes = computed<ApiRoute[]>(() => {
  const methods = ['get', 'post', 'put', 'patch', 'delete']
  return Object.entries(asMap(apiSpec.value?.paths)).flatMap(([path, operations]) => Object.entries(asMap(operations)).filter(([method]) => methods.includes(method.toLowerCase())).map(([method, operation]) => ({ path, method: method.toUpperCase(), operation: asMap(operation) })))
})

onMounted(async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/openapi.json`)
    if (!response.ok) throw new Error(`HTTP ${response.status}`)
    apiSpec.value = await response.json() as JsonMap
  } catch (error) {
    apiError.value = error instanceof Error ? error.message : 'Impossible de charger /api/openapi.json'
  } finally {
    apiLoading.value = false
  }
})

function tabClass(tab: 'quick-start' | 'api-doc' | 'mods-doc') { return ['menu-tab', { active: activeTab.value === tab }] }
function asMap(value: unknown): JsonMap { return value && typeof value === 'object' && !Array.isArray(value) ? value as JsonMap : {} }
function asArray(value: unknown): unknown[] { return Array.isArray(value) ? value : [] }
function urlParameters(operation: JsonMap): OpenApiParameter[] { return asArray(operation.parameters).map((parameter) => asMap(parameter) as OpenApiParameter).filter((parameter) => parameter.in === 'path' || parameter.in === 'query') }
function responseFormat(operation: JsonMap) { const responses = asMap(operation.responses); const response = asMap(responses['200'] || responses['201'] || responses['204'] || responses.default || Object.values(responses)[0]); const content = asMap(response.content); const jsonContent = asMap(content['application/json']); const firstContent = asMap(Object.values(content)[0]); const schema = asMap(jsonContent.schema || firstContent.schema); return Object.keys(schema).length ? JSON.stringify(schemaExample(resolveSchema(schema)), null, 2) : 'Aucun body JSON documenté pour cette réponse.' }
function resolveSchema(schema: JsonMap): JsonMap { const ref = typeof schema.$ref === 'string' ? schema.$ref : ''; if (!ref) return schema; const name = ref.split('/').pop() || ''; return (asMap(asMap(apiSpec.value?.components).schemas)[name] as JsonMap) || schema }
function schemaType(schema?: JsonMap): string { const resolved = resolveSchema(asMap(schema)); if (typeof resolved.$ref === 'string') return resolved.$ref.split('/').pop() || resolved.$ref; if (Array.isArray(resolved.enum)) return resolved.enum.map(String).join(' | '); if (typeof resolved.format === 'string') return `${String(resolved.type || 'string')}:${resolved.format}`; if (resolved.type === 'array') return `${schemaType(asMap(resolved.items))}[]`; return String(resolved.type || 'object') }
function schemaConstraint(schema?: JsonMap): string { const resolved = resolveSchema(asMap(schema)); if (typeof resolved.pattern === 'string') return resolved.pattern; if (Array.isArray(resolved.enum)) return `enum: ${resolved.enum.map(String).join(', ')}`; if (typeof resolved.format === 'string') return `format: ${resolved.format}`; return '' }
function schemaExample(schema?: JsonMap): unknown { const resolved = resolveSchema(asMap(schema)); if (Array.isArray(resolved.oneOf)) return schemaExample(asMap(resolved.oneOf[0])); if (Array.isArray(resolved.enum) && resolved.enum.length) return resolved.enum[0]; if (resolved.type === 'array') return [schemaExample(asMap(resolved.items))]; if (resolved.type === 'boolean') return true; if (resolved.type === 'integer' || resolved.type === 'number') return 0; if (resolved.type === 'string') return resolved.format ? `<${String(resolved.format)}>` : 'string'; const output: Record<string, unknown> = {}; for (const [key, value] of Object.entries(asMap(resolved.properties))) output[key] = schemaExample(asMap(value)); return Object.keys(output).length ? output : {} }
</script>

<style scoped>
.documentation { min-height: calc(100vh - 80px); padding: 2.2rem 1rem; }
.documentation-panel { max-width: 1480px; margin: 0 auto; padding: 2.25rem; }
.documentation-header { text-align: center; margin-bottom: 2.5rem; }
.documentation-intro { max-width: 920px; margin: 1rem auto 0; line-height: 1.9; color: rgba(255,255,255,.82); }
.documentation-layout { display: grid; grid-template-columns: 240px minmax(0, 1fr); gap: 2.5rem; align-items: start; }
.documentation-menu { position: sticky; top: 100px; display: flex; flex-direction: column; gap: .9rem; padding: 1rem; background: rgba(0,0,0,.28); border: 1px solid rgba(100,255,218,.22); border-radius: 12px; box-shadow: 0 12px 26px rgba(0,0,0,.28); backdrop-filter: blur(8px); }
.menu-tab { color: rgba(255,255,255,.78); background: rgba(0,0,0,.35); border: 2px solid rgba(100,255,218,.18); border-radius: 8px; padding: .9rem 1rem; text-align: left; font-weight: 700; cursor: pointer; line-height: 1.45; box-shadow: 3px 3px 0 rgba(0,0,0,.28); }
.menu-tab:hover { color: #fff; border-color: rgba(100,255,218,.5); background: rgba(100,255,218,.08); }
.menu-tab.active { color: #1a1a1a; background: #64ffda; border-color: #1a1a1a; box-shadow: 3px 3px 0 rgba(0,0,0,.35); }
.documentation-content { min-width: 0; }
.section-kicker { color: #ffd700; text-transform: uppercase; font-size: .75rem; letter-spacing: .08em; }
h2 { color: #64ffda; margin: .55rem 0 1.2rem; line-height: 1.35; }
h3, h4 { color: #ffd700; margin-bottom: 1rem; line-height: 1.45; }
p, li { color: rgba(255,255,255,.82); line-height: 1.95; }
.doc-grid, .mods-list, .api-route-list { display: grid; gap: 1.35rem; }
.doc-grid { grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); }
.doc-card { background: rgba(0,0,0,.28); border: 1px solid rgba(100,255,218,.18); border-radius: 12px; padding: 1.45rem; min-width: 0; }
.important-card { border-color: rgba(255,215,0,.55); }
.warning-card { border-color: rgba(255,193,7,.55); }
.download-link { display: flex; align-items: center; justify-content: center; width: 100%; max-width: 100%; min-height: 3.25rem; box-sizing: border-box; white-space: normal; overflow-wrap: anywhere; word-break: break-word; text-align: center; color: #1a1a1a; background: #64ffda; border: 2px solid #1a1a1a; border-radius: 8px; padding: .85rem 1rem; text-decoration: none; font-weight: 800; line-height: 1.35; box-shadow: 3px 3px 0 rgba(0,0,0,.35); }
pre { overflow: auto; background: rgba(0,0,0,.58); color: #d8fff6; border-radius: 10px; padding: 1rem; font-size: .85rem; line-height: 1.7; }
code { color: #64ffda; overflow-wrap: anywhere; }
.route-heading { display: flex; flex-wrap: wrap; gap: .9rem; align-items: center; margin-bottom: 1rem; }
.method-badge { display: inline-flex; justify-content: center; min-width: 72px; padding: .45rem .65rem; border-radius: 8px; color: #111; font-weight: 800; background: #64ffda; }
.method-badge.post { background: #ffd166; }
.method-badge.put, .method-badge.patch { background: #f4a261; }
.method-badge.delete { background: #ef476f; color: #fff; }
.route-block { margin-top: 1.45rem; }
.table-scroll { overflow-x: auto; }
.param-table { width: 100%; border-collapse: collapse; color: rgba(255,255,255,.84); }
.param-table th, .param-table td { padding: .8rem; border: 1px solid rgba(100,255,218,.18); text-align: left; line-height: 1.6; }
.param-table th { color: #ffd700; background: rgba(0,0,0,.28); }
.empty-note { color: rgba(255,255,255,.62); font-style: italic; }
@media (max-width: 900px) { .documentation-layout { grid-template-columns: 1fr; gap: 1.25rem; } .documentation-menu { position: static; } }
@media (max-width: 768px) { .documentation { padding: 1rem .55rem; } .documentation-panel { padding: 1rem; } .doc-grid { grid-template-columns: 1fr; } .doc-card { padding: 1rem; } .menu-tab { text-align: center; } }
</style>