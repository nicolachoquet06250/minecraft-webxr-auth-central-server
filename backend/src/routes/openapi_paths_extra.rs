use serde_json::{json, Value};

pub fn extra_openapi_paths() -> Value {
    json!({
        "/users/{id}": {
            "get": operation_with_id("users", "Récupérer un profil public", None, response_ref("UserResponse"))
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
            "get": operation_with_id("servers", "Récupérer un serveur public", None, response_ref("ServerResponse")),
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

fn id_param() -> Value { json!({ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }) }
fn ref_body(schema: &str) -> Value { json!({ "required": true, "content": { "application/json": { "schema": schema_ref(schema) } } }) }
fn schema_ref(schema: &str) -> Value { json!({ "$ref": format!("#/components/schemas/{schema}") }) }
fn response_ref(schema: &str) -> Value { json!({ "200": { "description": "OK", "content": { "application/json": { "schema": schema_ref(schema) } } } }) }
fn response_array_ref(schema: &str) -> Value { json!({ "200": { "description": "OK", "content": { "application/json": { "schema": { "type": "array", "items": schema_ref(schema) } } } } }) }
fn response_no_content(description: &str) -> Value { json!({ "204": { "description": description } }) }
