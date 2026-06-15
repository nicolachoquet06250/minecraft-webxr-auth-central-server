use axum::{Json, response::IntoResponse};
use serde_json::{json, Value};

pub async fn openapi_json() -> impl IntoResponse {
    Json(openapi_document())
}

fn ok_json(schema: Value) -> Value {
    json!({
        "description": "OK",
        "content": {
            "application/json": {
                "schema": schema
            }
        }
    })
}

fn ok_array(ref_path: &str) -> Value {
    ok_json(json!({
        "type": "array",
        "items": { "$ref": ref_path }
    }))
}

fn svg_response() -> Value {
    json!({
        "description": "Image SVG",
        "content": {
            "image/svg+xml": {
                "schema": { "type": "string" }
            }
        }
    })
}

fn description(text: &str) -> Value {
    json!({ "description": text })
}

fn json_body(ref_path: &str) -> Value {
    json!({
        "required": true,
        "content": {
            "application/json": {
                "schema": { "$ref": ref_path }
            }
        }
    })
}

fn id_path_parameter() -> Value {
    json!({
        "name": "id",
        "in": "path",
        "required": true,
        "schema": {
            "type": "string",
            "pattern": "^[0-9a-fA-F-]{36}$"
        }
    })
}

fn protected(summary: &str, responses: Value) -> Value {
    json!({
        "summary": summary,
        "security": [{ "bearerAuth": [] }],
        "responses": responses
    })
}

pub fn openapi_document() -> Value {
    json!({
        "openapi": "3.0.3",
        "info": {
            "title": "Voxicraft Auth API",
            "version": "1.0.0",
            "description": "API centrale d'authentification, de gestion des avatars et de registre des serveurs Voxicraft."
        },
        "servers": [{ "url": "/api" }],
        "components": {
            "securitySchemes": {
                "bearerAuth": { "type": "http", "scheme": "bearer", "bearerFormat": "JWT" }
            },
            "schemas": {
                "User": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "pattern": "^[0-9a-fA-F-]{36}$" },
                        "username": { "type": "string" },
                        "email": { "type": "string", "format": "email" },
                        "avatar": { "type": "string", "enum": ["steve", "alex"] },
                        "bio": { "type": "string", "nullable": true },
                        "birthdate": { "type": "string", "format": "date" },
                        "age_verified": { "type": "boolean" },
                        "discord_username": { "type": "string", "nullable": true },
                        "created_at": { "type": "string", "format": "date-time" }
                    }
                },
                "AuthResponse": {
                    "type": "object",
                    "properties": {
                        "token": { "type": "string" },
                        "user": { "$ref": "#/components/schemas/User" }
                    }
                },
                "RegisterData": {
                    "type": "object",
                    "required": ["username", "email", "password", "avatar", "birthdate"],
                    "properties": {
                        "username": { "type": "string", "pattern": "^[a-zA-Z0-9_-]{3,32}$" },
                        "email": { "type": "string", "format": "email" },
                        "password": { "type": "string", "format": "password", "minLength": 8 },
                        "avatar": { "type": "string", "enum": ["steve", "alex"] },
                        "birthdate": { "type": "string", "format": "date" },
                        "bio": { "type": "string" }
                    }
                },
                "LoginData": {
                    "type": "object",
                    "required": ["email", "password"],
                    "properties": {
                        "email": { "type": "string", "format": "email" },
                        "password": { "type": "string", "format": "password" }
                    }
                },
                "Server": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "pattern": "^[0-9a-fA-F-]{36}$" },
                        "owner_id": { "type": "string", "pattern": "^[0-9a-fA-F-]{36}$" },
                        "name": { "type": "string" },
                        "game_domain": { "type": "string" },
                        "description": { "type": "string", "nullable": true },
                        "is_active": { "type": "boolean" },
                        "created_at": { "type": "string", "format": "date-time" },
                        "updated_at": { "type": "string", "format": "date-time" }
                    }
                },
                "CreateServerData": {
                    "type": "object",
                    "required": ["name", "game_domain"],
                    "properties": {
                        "name": { "type": "string" },
                        "game_domain": { "type": "string", "pattern": "^[a-zA-Z0-9.-]+(:[0-9]{1,5})?$" },
                        "description": { "type": "string" }
                    }
                },
                "AvatarTextureData": {
                    "type": "object",
                    "properties": {
                        "version": { "type": "number", "enum": [1] },
                        "palette": { "type": "object" },
                        "parts": { "type": "object" }
                    }
                },
                "UserAvatar": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "pattern": "^[0-9a-fA-F-]{36}$" },
                        "name": { "type": "string" },
                        "base_kind": { "type": "string", "enum": ["steve", "alex", "custom"] },
                        "is_active": { "type": "boolean" },
                        "texture_data": { "$ref": "#/components/schemas/AvatarTextureData" },
                        "created_at": { "type": "string", "format": "date-time" },
                        "updated_at": { "type": "string", "format": "date-time" }
                    }
                },
                "SaveAvatarData": {
                    "type": "object",
                    "required": ["name", "base_kind", "texture_data"],
                    "properties": {
                        "name": { "type": "string" },
                        "base_kind": { "type": "string", "enum": ["steve", "alex", "custom"] },
                        "texture_data": { "$ref": "#/components/schemas/AvatarTextureData" }
                    }
                },
                "UpdateAvatarData": {
                    "type": "object",
                    "required": ["texture_data"],
                    "properties": {
                        "name": { "type": "string" },
                        "texture_data": { "$ref": "#/components/schemas/AvatarTextureData" }
                    }
                },
                "ActiveAvatarResponse": {
                    "type": "object",
                    "properties": {
                        "kind": { "type": "string", "enum": ["default", "custom"] },
                        "avatar": { "oneOf": [{ "$ref": "#/components/schemas/UserAvatar" }, { "type": "null" }] }
                    }
                },
                "DiscordUrlResponse": {
                    "type": "object",
                    "properties": { "url": { "type": "string", "format": "uri" } }
                },
                "MatrixColorResponse": {
                    "type": "object",
                    "properties": { "color": { "type": "string", "pattern": "^#[0-9a-fA-F]{6}$" } }
                }
            }
        },
        "paths": {
            "/openapi.json": {
                "get": { "summary": "Récupérer le document OpenAPI", "responses": { "200": ok_json(json!({ "type": "object" })) } }
            },
            "/auth/register": {
                "post": { "summary": "Créer un compte utilisateur", "requestBody": json_body("#/components/schemas/RegisterData"), "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/AuthResponse" })), "400": description("Requête invalide") } }
            },
            "/auth/login": {
                "post": { "summary": "Connecter un utilisateur", "requestBody": json_body("#/components/schemas/LoginData"), "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/AuthResponse" })), "401": description("Identifiants invalides") } }
            },
            "/auth/discord/url": {
                "get": { "summary": "Récupérer l'URL OAuth Discord", "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/DiscordUrlResponse" })) } }
            },
            "/auth/discord/callback": {
                "get": { "summary": "Callback OAuth Discord", "responses": { "302": description("Redirection après authentification Discord") } }
            },
            "/mail/status": {
                "get": { "summary": "Vérifier la configuration mail", "responses": { "200": ok_json(json!({ "type": "object" })) } }
            },
            "/contact": {
                "post": { "summary": "Envoyer un message de contact", "responses": { "200": ok_json(json!({ "type": "object" })) } }
            },
            "/support": {
                "post": { "summary": "Envoyer une demande de support", "responses": { "200": ok_json(json!({ "type": "object" })) } }
            },
            "/users/{id}": {
                "get": { "summary": "Récupérer un profil public", "parameters": [id_path_parameter()], "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/User" })), "404": description("Utilisateur introuvable") } }
            },
            "/users/me": {
                "get": protected("Récupérer le profil connecté", json!({ "200": ok_json(json!({ "$ref": "#/components/schemas/User" })) })),
                "put": protected("Mettre à jour le profil connecté", json!({ "200": ok_json(json!({ "$ref": "#/components/schemas/User" })) })),
                "delete": protected("Supprimer le compte connecté", json!({ "204": description("Compte supprimé") }))
            },
            "/users/me/discord": {
                "delete": protected("Délier Discord du compte connecté", json!({ "204": description("Discord délié") }))
            },
            "/users/me/avatar": {
                "get": protected("Récupérer l'avatar actif", json!({ "200": ok_json(json!({ "$ref": "#/components/schemas/ActiveAvatarResponse" })) })),
                "delete": protected("Désactiver l'avatar personnalisé actif", json!({ "204": description("Avatar actif supprimé") }))
            },
            "/users/me/avatars": {
                "get": protected("Lister les avatars personnalisés", json!({ "200": ok_array("#/components/schemas/UserAvatar") })),
                "post": { "summary": "Créer une copie d'avatar", "security": [{ "bearerAuth": [] }], "requestBody": json_body("#/components/schemas/SaveAvatarData"), "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/UserAvatar" })) } }
            },
            "/users/me/avatars/{id}": {
                "put": { "summary": "Modifier un avatar personnalisé", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "requestBody": json_body("#/components/schemas/UpdateAvatarData"), "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/UserAvatar" })) } },
                "delete": { "summary": "Supprimer un avatar personnalisé", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "responses": { "204": description("Avatar supprimé") } }
            },
            "/users/me/avatars/{id}/select": {
                "put": { "summary": "Sélectionner un avatar personnalisé", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "responses": { "204": description("Avatar sélectionné") } }
            },
            "/users/me/profile-pic.svg": {
                "get": protected("Récupérer la tête SVG de l'avatar connecté", json!({ "200": svg_response() }))
            },
            "/users/{id}/profile-pic.svg": {
                "get": { "summary": "Récupérer la tête SVG d'un utilisateur", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "responses": { "200": svg_response() } }
            },
            "/users/{id}/matrix-color": {
                "get": { "summary": "Récupérer la couleur Matrix d'un utilisateur", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/MatrixColorResponse" })) } }
            },
            "/servers": {
                "get": protected("Lister les serveurs du compte connecté", json!({ "200": ok_array("#/components/schemas/Server") })),
                "post": { "summary": "Créer un serveur de jeu", "security": [{ "bearerAuth": [] }], "requestBody": json_body("#/components/schemas/CreateServerData"), "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/Server" })) } }
            },
            "/servers/recent": {
                "get": protected("Lister les serveurs récemment visités", json!({ "200": ok_array("#/components/schemas/Server") }))
            },
            "/servers/favorites": {
                "get": protected("Lister les serveurs favoris", json!({ "200": ok_array("#/components/schemas/Server") }))
            },
            "/servers/visit": {
                "post": protected("Enregistrer une visite serveur", json!({ "200": description("Visite enregistrée") }))
            },
            "/servers/{id}": {
                "get": { "summary": "Récupérer un serveur public", "parameters": [id_path_parameter()], "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/Server" })) } },
                "put": { "summary": "Modifier un serveur", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "requestBody": json_body("#/components/schemas/CreateServerData"), "responses": { "200": ok_json(json!({ "$ref": "#/components/schemas/Server" })) } },
                "delete": { "summary": "Supprimer un serveur", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "responses": { "204": description("Serveur supprimé") } }
            },
            "/servers/{id}/favorite": {
                "post": { "summary": "Ajouter un serveur aux favoris", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "responses": { "200": description("Favori ajouté") } },
                "delete": { "summary": "Retirer un serveur des favoris", "security": [{ "bearerAuth": [] }], "parameters": [id_path_parameter()], "responses": { "204": description("Favori supprimé") } }
            },
            "/users/me/password/change-code": {
                "post": protected("Demander un code de changement de mot de passe", json!({ "200": description("Code envoyé") }))
            },
            "/users/me/password": {
                "put": protected("Confirmer le changement de mot de passe", json!({ "200": description("Mot de passe changé") }))
            }
        }
    })
}
