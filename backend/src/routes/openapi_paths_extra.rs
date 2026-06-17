use serde_json::{json, Value};

pub fn extra_openapi_paths() -> Value {
    json!({
        "/users/search": {
            "get": {
                "tags": ["users"],
                "summary": "Lister ou rechercher des utilisateurs",
                "security": [{ "bearerAuth": [] }],
                "parameters": [
                    {
                        "name": "q",
                        "in": "query",
                        "required": false,
                        "description": "Recherche par nom d'utilisateur. Si absent ou vide, tous les utilisateurs sont listés en pagination.",
                        "schema": { "type": "string", "minLength": 2 }
                    },
                    {
                        "name": "page",
                        "in": "query",
                        "required": false,
                        "schema": { "type": "integer", "format": "uint64", "minimum": 1, "default": 1 }
                    },
                    {
                        "name": "page_size",
                        "in": "query",
                        "required": false,
                        "schema": { "type": "integer", "format": "uint64", "minimum": 1, "maximum": 50, "default": 20 }
                    }
                ],
                "responses": response_paginated_user_search()
            }
        },
        "/users/{id}": {
            "get": secured_operation_with_id("users", "Récupérer un profil utilisateur", None, response_ref("UserResponse"))
        },
        "/friends": {
            "get": secured_operation("friends", "Lister les amis du compte connecté", None, response_array(friend_response_schema()))
        },
        "/friends/requests": {
            "post": secured_operation("friends", "Envoyer une demande d'ami", Some(create_friend_request_body()), response_schema(friend_request_response_schema()))
        },
        "/friends/requests/incoming": {
            "get": secured_operation("friends", "Lister les demandes d'ami reçues", None, response_array(friend_request_response_schema()))
        },
        "/friends/requests/outgoing": {
            "get": secured_operation("friends", "Lister les demandes d'ami envoyées", None, response_array(friend_request_response_schema()))
        },
        "/friends/requests/{id}/accept": {
            "post": secured_operation_with_id("friends", "Accepter une demande d'ami", None, response_schema(friend_request_response_schema()))
        },
        "/friends/requests/{id}/refuse": {
            "post": secured_operation_with_id("friends", "Refuser une demande d'ami", None, response_schema(friend_request_response_schema()))
        },
        "/friends/{userId}": {
            "delete": secured_operation_with_user_id("friends", "Supprimer un ami ou annuler une demande envoyée", None, response_no_content("Ami supprimé ou demande annulée"))
        },
        "/users/me/avatar": {
            "get": secured_operation("avatars", "Récupérer l'avatar actif", None, response_ref("ActiveAvatarResponse")),
            "delete": secured_operation("avatars", "Désactiver l'avatar personnalisé actif", None, response_no_content("Avatar actif supprimé"))
        },
        "/users/me/avatars": {
            "get": secured_operation("avatars", "Lister les avatars personnalisés", None, response_array_ref("AvatarResponse")),
            "post": secured_operation("avatars", "Créer une copie d'avatar", Some(ref_body("SaveAvatarRequest")), response_ref("AvatarResponse"))
        },
        "/users/me/avatars/{id}": {
            "put": secured_operation_with_id("avatars", "Modifier un avatar personnalisé", Some(ref_body("UpdateAvatarRequest")), response_ref("AvatarResponse")),
            "delete": secured_operation_with_id("avatars", "Supprimer un avatar personnalisé", None, response_no_content("Avatar supprimé"))
        },
        "/users/me/avatars/{id}/select": {
            "put": secured_operation_with_id("avatars", "Sélectionner un avatar personnalisé", None, response_no_content("Avatar sélectionné"))
        },
        "/servers/{id}": {
            "get": secured_operation_with_id("servers", "Récupérer un serveur", None, response_ref("ServerResponse")),
            "put": secured_operation_with_id("servers", "Modifier un serveur", Some(ref_body("UpdateServerRequest")), response_ref("ServerResponse")),
            "delete": secured_operation_with_id("servers", "Supprimer un serveur", None, response_no_content("Serveur supprimé"))
        }
    })
}

fn operation(tag: &str, summary: &str, request_body: Option<Value>, responses: Value) -> Value {
    let mut value = json!({ "tags": [tag], "summary": summary, "responses": responses });
    if let Some(body) = request_body { value["requestBody"] = body; }
    value
}

fn secured_operation(tag: &str, summary: &str, request_body: Option<Value>, responses: Value) -> Value {
    let mut value = operation(tag, summary, request_body, responses);
    value["security"] = json!([{ "bearerAuth": [] }]);
    value
}

fn operation_with_id(tag: &str, summary: &str, request_body: Option<Value>, responses: Value) -> Value {
    let mut value = operation(tag, summary, request_body, responses);
    value["parameters"] = json!([id_param()]);
    value
}

fn secured_operation_with_id(tag: &str, summary: &str, request_body: Option<Value>, responses: Value) -> Value {
    let mut value = secured_operation(tag, summary, request_body, responses);
    value["parameters"] = json!([id_param()]);
    value
}

fn secured_operation_with_user_id(tag: &str, summary: &str, request_body: Option<Value>, responses: Value) -> Value {
    let mut value = secured_operation(tag, summary, request_body, responses);
    value["parameters"] = json!([user_id_param()]);
    value
}

fn id_param() -> Value { json!({ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }) }
fn user_id_param() -> Value { json!({ "name": "userId", "in": "path", "required": true, "schema": { "type": "string" } }) }
fn ref_body(schema: &str) -> Value { json!({ "required": true, "content": { "application/json": { "schema": schema_ref(schema) } } }) }
fn schema_ref(schema: &str) -> Value { json!({ "$ref": format!("#/components/schemas/{schema}") }) }
fn response_ref(schema: &str) -> Value { json!({ "200": { "description": "OK", "content": { "application/json": { "schema": schema_ref(schema) } } } }) }
fn response_array_ref(schema: &str) -> Value { json!({ "200": { "description": "OK", "content": { "application/json": { "schema": { "type": "array", "items": schema_ref(schema) } } } } }) }
fn response_no_content(description: &str) -> Value { json!({ "204": { "description": description } }) }
fn response_schema(schema: Value) -> Value { json!({ "200": { "description": "OK", "content": { "application/json": { "schema": schema } } } }) }
fn response_array(schema: Value) -> Value { json!({ "200": { "description": "OK", "content": { "application/json": { "schema": { "type": "array", "items": schema } } } } }) }

fn create_friend_request_body() -> Value {
    json!({
        "required": true,
        "content": {
            "application/json": {
                "schema": {
                    "type": "object",
                    "required": ["receiver_user_id"],
                    "properties": {
                        "receiver_user_id": { "type": "string" }
                    }
                }
            }
        }
    })
}

fn friend_avatar_schema() -> Value {
    json!({
        "type": "object",
        "required": ["kind", "base_kind", "name", "url"],
        "properties": {
            "kind": { "type": "string", "enum": ["default", "custom"], "example": "custom" },
            "base_kind": { "type": "string", "example": "steve" },
            "name": { "type": "string", "example": "Avatar médiéval" },
            "url": { "type": "string", "example": "/api/users/00000000-0000-0000-0000-000000000000/profile-pic.svg" }
        }
    })
}

fn friend_user_schema() -> Value {
    json!({
        "type": "object",
        "required": ["id", "username", "avatar"],
        "properties": {
            "id": { "type": "string" },
            "username": { "type": "string" },
            "avatar": friend_avatar_schema()
        }
    })
}

fn friend_request_response_schema() -> Value {
    json!({
        "type": "object",
        "required": ["id", "requester", "receiver", "status", "created_at", "updated_at"],
        "properties": {
            "id": { "type": "string" },
            "requester": friend_user_schema(),
            "receiver": friend_user_schema(),
            "status": { "type": "string", "enum": ["pending", "accepted", "refused", "cancelled"] },
            "created_at": { "type": "string" },
            "updated_at": { "type": "string" }
        }
    })
}

fn friend_response_schema() -> Value {
    json!({
        "type": "object",
        "required": ["user", "created_at"],
        "properties": {
            "user": friend_user_schema(),
            "created_at": { "type": "string" }
        }
    })
}

fn response_paginated_user_search() -> Value {
    json!({
        "200": {
            "description": "OK",
            "content": {
                "application/json": {
                    "schema": {
                        "type": "object",
                        "required": ["items", "page", "page_size", "total", "total_pages", "next_url", "previous_url"],
                        "properties": {
                            "items": { "type": "array", "items": friend_user_schema() },
                            "page": { "type": "integer", "format": "uint64", "minimum": 1, "example": 1 },
                            "page_size": { "type": "integer", "format": "uint64", "minimum": 1, "maximum": 50, "example": 20 },
                            "total": { "type": "integer", "format": "uint64", "example": 42 },
                            "total_pages": { "type": "integer", "format": "uint64", "example": 3 },
                            "next_url": { "type": "string", "nullable": true, "example": "/api/users/search?page=2&page_size=20" },
                            "previous_url": { "type": "string", "nullable": true, "example": null }
                        }
                    }
                }
            }
        },
        "400": { "description": "Requête invalide, q doit contenir au moins 2 caractères quand il est renseigné" },
        "401": { "description": "JWT manquant ou invalide" }
    })
}
