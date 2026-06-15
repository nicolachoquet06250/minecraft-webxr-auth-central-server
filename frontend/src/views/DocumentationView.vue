<template>
  <main class="documentation voxicraft-bg">
    <section class="voxicraft-panel documentation-panel">
      <header class="documentation-header">
        <h1 class="voxicraft-title">Documentation Voxicraft</h1>
        <p class="documentation-intro">Quick start serveur, API centrale et documentation détaillée des mods.</p>
      </header>

      <div class="documentation-layout">
        <aside class="documentation-menu">
          <button :class="tabClass('quick-start')" type="button" @click="activeTab = 'quick-start'">Quick start serveur</button>
          <button :class="tabClass('api-doc')" type="button" @click="activeTab = 'api-doc'">API centrale</button>
          <button :class="tabClass('mods-doc')" type="button" @click="activeTab = 'mods-doc'">Mods</button>
        </aside>

        <div class="documentation-content">
          <section v-if="activeTab === 'quick-start'" class="documentation-section">
            <span class="section-kicker">Quick start</span>
            <h2>Installer un serveur de jeu</h2>
            <p>Ces étapes permettent de récupérer, configurer puis lancer un serveur de jeu auto-hébergé.</p>
            <div class="doc-grid">
              <article class="doc-card important-card">
                <h3>1. Télécharger</h3>
                <p>Récupère toujours la dernière release publiée, sans dépendre du nom ou du numéro de version.</p>
                <a class="download-link" href="https://github.com/nicolachoquet06250/minecraft-webxr/releases/latest" target="_blank" rel="noopener noreferrer">Télécharger la dernière version</a>
              </article>
              <article class="doc-card"><h3>2. Extraire</h3><p>Décompresse l'archive sur la machine serveur.</p><pre>tar -xzf voxicraft-server-*.tar.gz
cd voxicraft-server</pre></article>
              <article class="doc-card"><h3>3. Configurer</h3><p>Prépare le fichier d'environnement du serveur.</p><pre>cp .env.example .env
nano .env</pre></article>
              <article class="doc-card"><h3>4. Lancer</h3><p>Démarre le binaire du serveur de jeu.</p><pre>chmod +x voxicraft_server
./voxicraft_server</pre></article>
            </div>
          </section>

          <section v-else-if="activeTab === 'api-doc'" class="documentation-section">
            <span class="section-kicker">API</span>
            <h2>Documentation de l'API centrale</h2>
            <p>Cette section lit le JSON OpenAPI/Swagger et affiche les routes, les paramètres d'URL et le format JSON de réponse.</p>

            <article class="doc-card auth-card">
              <h3>Authentification</h3>
              <p>Les routes protégées utilisent un JWT dans l'en-tête HTTP.</p>
              <pre>Authorization: Bearer &lt;token&gt;</pre>
            </article>

            <article v-if="swaggerLoading" class="doc-card">Chargement du Swagger JSON...</article>
            <article v-else-if="swaggerError" class="doc-card warning-card">{{ swaggerError }}</article>

            <div v-else class="api-route-list">
              <article v-for="route in apiRoutes" :key="route.method + route.path" class="doc-card route-card">
                <div class="route-heading">
                  <span class="method-badge" :class="route.method.toLowerCase()">{{ route.method }}</span>
                  <code>{{ route.path }}</code>
                </div>
                <p v-if="route.operation.summary">{{ route.operation.summary }}</p>
                <p v-if="route.operation.description">{{ route.operation.description }}</p>

                <div class="route-block">
                  <h4>Paramètres d'URL</h4>
                  <div v-if="urlParameters(route.operation).length" class="table-scroll">
                    <table class="param-table">
                      <thead><tr><th>Nom</th><th>Position</th><th>Type</th><th>Obligatoire</th><th>Regex / contrainte</th></tr></thead>
                      <tbody>
                        <tr v-for="parameter in urlParameters(route.operation)" :key="parameter.name + parameter.in">
                          <td><code>{{ parameter.name }}</code></td>
                          <td>{{ parameter.in }}</td>
                          <td><code>{{ schemaType(parameter.schema) }}</code></td>
                          <td>{{ parameter.required ? 'Oui' : 'Non' }}</td>
                          <td><code>{{ schemaConstraint(parameter.schema) || '—' }}</code></td>
                        </tr>
                      </tbody>
                    </table>
                  </div>
                  <p v-else class="empty-note">Aucun paramètre d'URL documenté.</p>
                </div>

                <div class="route-block">
                  <h4>Format de réponse</h4>
                  <pre>{{ responseFormat(route.operation) }}</pre>
                </div>
              </article>
            </div>
          </section>

          <section v-else class="documentation-section">
            <span class="section-kicker">Expérimental</span>
            <h2>Documentation des mods</h2>
            <p>Le système de mods permet d'exposer des extensions côté client et, à terme, côté serveur. Les fichiers client restent publics pour être chargés en solo comme en multi.</p>

            <div class="mods-list">
              <article class="doc-card">
                <h3>1. Objectif</h3>
                <p>Un mod peut ajouter du comportement navigateur, des assets, des écrans ou des intégrations avec l'API du jeu. Le chargement client doit rester découplé de l'authentification pour fonctionner aussi en solo.</p>
              </article>
              <article class="doc-card">
                <h3>2. Manifest <code>mod.json</code></h3>
                <p>Le manifest décrit le mod. Les champs strictement nécessaires doivent rester minimaux, et les champs optionnels comme <code>assets</code> doivent être acceptés même lorsqu'ils sont absents.</p>
                <pre>{
  "id": "example-mod",
  "name": "Example Mod",
  "version": "0.1.0",
  "client": "client/mod.js",
  "assets": ["assets/icon.png"]
}</pre>
              </article>
              <article class="doc-card">
                <h3>3. Structure recommandée</h3>
                <pre>mods/example-mod/
├── mod.json
├── client/
│   └── mod.js
└── assets/
    └── icon.png</pre>
              </article>
              <article class="doc-card">
                <h3>4. Routes publiques</h3>
                <p>Le serveur expose le manifest global et les fichiers client des mods. Ces routes doivent rester publiques.</p>
                <pre>GET /api/mods/manifest
GET /mods/:id/client/...
GET /mods/:id/assets/...</pre>
              </article>
              <article class="doc-card">
                <h3>5. Chargement côté navigateur</h3>
                <p>Le client récupère le manifest, filtre les mods activés, puis charge le fichier JavaScript client. Le fichier chargé peut ensuite s'enregistrer dans l'API du jeu.</p>
                <pre>export function register(game) {
  game.events.on('ready', () => {
    console.log('mod prêt')
  })
}</pre>
              </article>
              <article class="doc-card">
                <h3>6. Compatibilité et sécurité</h3>
                <p>Les mods client s'exécutent dans le navigateur. Un serveur public ne doit servir que des fichiers de mods qu'il contrôle. Les APIs exposées aux mods doivent être limitées et documentées.</p>
              </article>
            </div>
          </section>
        </div>
      </div>
    </section>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'

type JsonMap = Record<string, unknown>

type OpenApiParameter = {
  name: string
  in: string
  required?: boolean
  schema?: JsonMap
}

type ApiRoute = {
  path: string
  method: string
  operation: JsonMap
}

const activeTab = ref<'quick-start' | 'api-doc' | 'mods-doc'>('quick-start')
const swagger = ref<JsonMap | null>(null)
const swaggerLoading = ref(true)
const swaggerError = ref('')

const swaggerUrls = ['/api-docs/openapi.json', '/api-docs/swagger.json', '/openapi.json', '/swagger.json', '/api/openapi.json', '/api/swagger.json']

const apiRoutes = computed<ApiRoute[]>(() => {
  const methods = ['get', 'post', 'put', 'patch', 'delete']
  const paths = asMap(swagger.value?.paths)

  return Object.entries(paths).flatMap(([path, operations]) =>
    Object.entries(asMap(operations))
      .filter(([method]) => methods.includes(method.toLowerCase()))
      .map(([method, operation]) => ({ path, method: method.toUpperCase(), operation: asMap(operation) })),
  )
})

onMounted(async () => {
  for (const url of swaggerUrls) {
    try {
      const response = await fetch(url)
      if (!response.ok) continue
      swagger.value = await response.json() as JsonMap
      swaggerLoading.value = false
      return
    } catch {
      // Try next OpenAPI endpoint.
    }
  }
  swaggerLoading.value = false
  swaggerError.value = `Aucun JSON Swagger/OpenAPI trouvé : ${swaggerUrls.join(', ')}`
})

function tabClass(tab: 'quick-start' | 'api-doc' | 'mods-doc') {
  return ['menu-tab', { active: activeTab.value === tab }]
}

function asMap(value: unknown): JsonMap {
  return value && typeof value === 'object' && !Array.isArray(value) ? value as JsonMap : {}
}

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : []
}

function urlParameters(operation: JsonMap): OpenApiParameter[] {
  return asArray(operation.parameters)
    .map((parameter) => asMap(parameter) as OpenApiParameter)
    .filter((parameter) => parameter.in === 'path' || parameter.in === 'query')
}

function responseFormat(operation: JsonMap) {
  const responses = asMap(operation.responses)
  const response = asMap(responses['200'] || responses['201'] || responses['204'] || responses.default || Object.values(responses)[0])
  const content = asMap(response.content)
  const jsonContent = asMap(content['application/json'])
  const firstContent = asMap(Object.values(content)[0])
  const schema = asMap(jsonContent.schema || firstContent.schema)

  if (Object.keys(schema).length === 0) return 'Aucun body JSON documenté pour cette réponse.'
  return JSON.stringify(schemaExample(resolveSchema(schema)), null, 2)
}

function resolveSchema(schema: JsonMap): JsonMap {
  const ref = typeof schema.$ref === 'string' ? schema.$ref : ''
  if (!ref) return schema
  const name = ref.split('/').pop() || ''
  return asMap(asMap(swagger.value?.components).schemas)[name] as JsonMap || asMap(swagger.value?.definitions)[name] as JsonMap || schema
}

function schemaType(schema?: JsonMap): string {
  const resolved = resolveSchema(asMap(schema))
  if (typeof resolved.$ref === 'string') return resolved.$ref.split('/').pop() || resolved.$ref
  if (Array.isArray(resolved.enum)) return resolved.enum.map(String).join(' | ')
  if (typeof resolved.format === 'string') return `${String(resolved.type || 'string')}:${resolved.format}`
  if (resolved.type === 'array') return `${schemaType(asMap(resolved.items))}[]`
  return String(resolved.type || 'object')
}

function schemaConstraint(schema?: JsonMap): string {
  const resolved = resolveSchema(asMap(schema))
  if (typeof resolved.pattern === 'string') return resolved.pattern
  if (Array.isArray(resolved.enum)) return `enum: ${resolved.enum.map(String).join(', ')}`
  if (typeof resolved.format === 'string') return `format: ${resolved.format}`
  return ''
}

function schemaExample(schema?: JsonMap): unknown {
  const resolved = resolveSchema(asMap(schema))
  if (Array.isArray(resolved.allOf)) return Object.assign({}, ...resolved.allOf.map((item) => schemaExample(asMap(item))))
  if (Array.isArray(resolved.oneOf)) return schemaExample(asMap(resolved.oneOf[0]))
  if (Array.isArray(resolved.anyOf)) return schemaExample(asMap(resolved.anyOf[0]))
  if (Array.isArray(resolved.enum) && resolved.enum.length) return resolved.enum[0]
  if (resolved.type === 'array') return [schemaExample(asMap(resolved.items))]
  if (resolved.type === 'boolean') return true
  if (resolved.type === 'integer' || resolved.type === 'number') return 0
  if (resolved.type === 'string') return resolved.format ? `<${String(resolved.format)}>` : 'string'

  const output: Record<string, unknown> = {}
  for (const [key, value] of Object.entries(asMap(resolved.properties))) output[key] = schemaExample(asMap(value))
  return Object.keys(output).length ? output : {}
}
</script>

<style scoped>
.documentation { min-height: calc(100vh - 80px); padding: 2.2rem 1rem; }
.documentation-panel { max-width: 1480px; margin: 0 auto; padding: 2.25rem; }
.documentation-header { text-align: center; margin-bottom: 2.5rem; }
.documentation-intro { max-width: 920px; margin: 1rem auto 0; line-height: 1.9; color: rgba(255,255,255,.82); }
.documentation-layout { display: grid; grid-template-columns: 240px minmax(0, 1fr); gap: 2.5rem; align-items: start; }
.documentation-menu { position: sticky; top: 100px; display: flex; flex-direction: column; gap: .9rem; padding: 1rem; background: rgba(0,0,0,.24); border: 1px solid rgba(100,255,218,.22); border-radius: 14px; }
.menu-tab { color: #aaa; background: rgba(0,0,0,.35); border: 2px solid rgba(100,255,218,.18); border-radius: 8px; padding: .9rem 1rem; text-align: left; font-weight: 700; cursor: pointer; line-height: 1.45; }
.menu-tab.active { color: #1a1a1a; background: #64ffda; border-color: #1a1a1a; box-shadow: 3px 3px 0 rgba(0,0,0,.35); }
.documentation-content { min-width: 0; }
.documentation-section { animation: page-in .16s ease-out; }
@keyframes page-in { from { opacity: 0; transform: translateY(6px); } to { opacity: 1; transform: translateY(0); } }
.section-kicker { color: #ffd700; text-transform: uppercase; font-size: .75rem; letter-spacing: .08em; }
h2 { color: #64ffda; margin: .55rem 0 1.2rem; line-height: 1.35; }
h3, h4 { color: #ffd700; margin-bottom: 1rem; line-height: 1.45; }
p, li { color: rgba(255,255,255,.82); line-height: 1.95; }
.doc-grid, .mods-list { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.35rem; }
.mods-list { grid-template-columns: 1fr; }
.doc-card { background: rgba(0,0,0,.28); border: 1px solid rgba(100,255,218,.18); border-radius: 12px; padding: 1.45rem; min-width: 0; }
.important-card { border-color: rgba(255,215,0,.55); }
.auth-card, .warning-card { margin-bottom: 1.5rem; }
.warning-card { border-color: rgba(255,193,7,.55); }
.download-link { display: flex; align-items: center; justify-content: center; width: 100%; max-width: 100%; box-sizing: border-box; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; color: #1a1a1a; background: #64ffda; border: 2px solid #1a1a1a; border-radius: 8px; padding: .85rem 1rem; text-decoration: none; font-weight: 800; box-shadow: 3px 3px 0 rgba(0,0,0,.35); }
pre { overflow: auto; background: rgba(0,0,0,.58); color: #d8fff6; border-radius: 10px; padding: 1rem; font-size: .85rem; line-height: 1.7; }
code { color: #64ffda; overflow-wrap: anywhere; }
.api-route-list { display: grid; gap: 1.65rem; }
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
@media (max-width: 768px) { .documentation { padding: 1rem .55rem; } .documentation-panel { padding: 1rem; } .doc-grid { grid-template-columns: 1fr; } .doc-card { padding: 1rem; } .menu-tab { text-align: center; } .download-link { white-space: normal; } }
</style>
