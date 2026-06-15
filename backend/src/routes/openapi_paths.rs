use serde_json::{json, Value};

pub fn openapi_paths() -> Value {
    json!({
        "/openapi.json": {
            "get": operation("openapi", "Récupérer le document OpenAPI", None, response_object())
        },
        "/auth/register": {
            "post": operation("auth", "Créer un compte utilisateur", Some(ref_body("RegisterRequest")), response_ref("AuthResponse"))
        },
        "/auth/login": {
            "post": operation("auth", "Connecter un utilisateur", Some(ref_body("LoginRequest")), response_ref("AuthResponse"))
        },
        "/auth/discord/url": {
            "get": operation("auth", "Récupérer l'URL OAuth Discord", None, response_ref("DiscordOAuthUrl"))
        },
        "/users/me": {
            "get": secured_operation("users", "Récupérer le profil connecté", None, response_ref("UserResponse")),
            "put": secured_operation("users", "Mettre à jour le profil connecté", Some(ref_body("UpdateUserRequest")), response_ref("UserResponse"))
        },
        "/servers": {
            "get": secured_operation("servers", "Lister les serveurs du compte connecté", None, response_array_ref("ServerResponse")),
            "post": secured_operation("servers", "Créer un serveur de jeu", Some(ref_body("CreateServerRequest")), response_ref("ServerResponse"))
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

fn ref_body(schema: &str) -> Value {
    json!({ "required": true, "content": { "application/json": { "schema": schema_ref(schema) } } })
}

fn schema_ref(schema: &str) -> Value {
    json!({ "$ref": format!("#/components/schemas/{schema}") })
}

fn response_ref(schema: &str) -> Value {
    json!({ "200": { "description": "OK", "content": { "application/json": { "schema": schema_ref(schema) } } } })
}

fn response_array_ref(schema: &str) -> Value {
    json!({ "200": { "description": "OK", "content": { "application/json": { "schema": { "type": "array", "items": schema_ref(schema) } } } } })
}

fn response_object() -> Value {
    json!({ "200": { "description": "OK", "content": { "application/json": { "schema": { "type": "object" } } } } })
}
