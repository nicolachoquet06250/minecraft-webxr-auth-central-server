use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use crate::{
    models::{avatar, friend_request, friendship, user, Avatar, FriendRequest, Friendship, User},
    services::Claims,
    AppState,
};

const STATUS_PENDING: &str = "pending";
const STATUS_ACCEPTED: &str = "accepted";
const STATUS_REFUSED: &str = "refused";
const STATUS_CANCELLED: &str = "cancelled";

#[derive(Debug, Deserialize)]
pub struct CreateFriendRequestPayload {
    pub receiver_user_id: String,
}

#[derive(Debug, Serialize)]
pub struct FriendAvatarResponse {
    pub kind: String,
    pub base_kind: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct FriendUserResponse {
    pub id: String,
    pub username: String,
    pub avatar: FriendAvatarResponse,
}

#[derive(Debug, Serialize)]
pub struct FriendRequestResponse {
    pub id: String,
    pub requester: FriendUserResponse,
    pub receiver: FriendUserResponse,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct FriendResponse {
    pub user: FriendUserResponse,
    pub created_at: String,
}

pub async fn create_friend_request(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateFriendRequestPayload>,
) -> Result<Json<FriendRequestResponse>, StatusCode> {
    let requester_user_id = claims.sub.clone();
    let receiver_user_id = payload.receiver_user_id.trim().to_string();

    if receiver_user_id.is_empty() || requester_user_id == receiver_user_id {
        return Err(StatusCode::BAD_REQUEST);
    }

    User::find_by_id(receiver_user_id.clone())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if find_friendship(&state, &requester_user_id, &receiver_user_id).await?.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let pending_request = FriendRequest::find()
        .filter(friend_request_pair_condition(&requester_user_id, &receiver_user_id))
        .filter(friend_request::Column::Status.eq(STATUS_PENDING))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if pending_request.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let now = chrono::Utc::now().naive_utc();
    let request = friend_request::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        requester_user_id: Set(requester_user_id),
        receiver_user_id: Set(receiver_user_id),
        status: Set(STATUS_PENDING.to_string()),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(friend_request_to_response(&state, request).await?))
}

pub async fn get_incoming_friend_requests(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FriendRequestResponse>>, StatusCode> {
    let requests = FriendRequest::find()
        .filter(friend_request::Column::ReceiverUserId.eq(claims.sub))
        .filter(friend_request::Column::Status.eq(STATUS_PENDING))
        .order_by_desc(friend_request::Column::CreatedAt)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    friend_requests_to_response(&state, requests).await.map(Json)
}

pub async fn get_outgoing_friend_requests(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FriendRequestResponse>>, StatusCode> {
    let requests = FriendRequest::find()
        .filter(friend_request::Column::RequesterUserId.eq(claims.sub))
        .filter(friend_request::Column::Status.eq(STATUS_PENDING))
        .order_by_desc(friend_request::Column::CreatedAt)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    friend_requests_to_response(&state, requests).await.map(Json)
}

pub async fn accept_friend_request(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(request_id): Path<String>,
) -> Result<Json<FriendRequestResponse>, StatusCode> {
    let request = FriendRequest::find_by_id(request_id)
        .filter(friend_request::Column::ReceiverUserId.eq(claims.sub.clone()))
        .filter(friend_request::Column::Status.eq(STATUS_PENDING))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let now = chrono::Utc::now().naive_utc();
    let requester_user_id = request.requester_user_id.clone();
    let receiver_user_id = request.receiver_user_id.clone();

    let mut active_request: friend_request::ActiveModel = request.into();
    active_request.status = Set(STATUS_ACCEPTED.to_string());
    active_request.updated_at = Set(now);
    let updated_request = active_request
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    ensure_friendship(&state, &requester_user_id, &receiver_user_id).await?;

    Ok(Json(friend_request_to_response(&state, updated_request).await?))
}

pub async fn refuse_friend_request(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(request_id): Path<String>,
) -> Result<Json<FriendRequestResponse>, StatusCode> {
    let request = FriendRequest::find_by_id(request_id)
        .filter(friend_request::Column::ReceiverUserId.eq(claims.sub))
        .filter(friend_request::Column::Status.eq(STATUS_PENDING))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active_request: friend_request::ActiveModel = request.into();
    active_request.status = Set(STATUS_REFUSED.to_string());
    active_request.updated_at = Set(chrono::Utc::now().naive_utc());
    let updated_request = active_request
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(friend_request_to_response(&state, updated_request).await?))
}

pub async fn delete_friend(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let current_user_id = claims.sub.clone();
    if current_user_id == user_id {
        return Err(StatusCode::BAD_REQUEST);
    }

    if let Some(friendship) = find_friendship(&state, &current_user_id, &user_id).await? {
        friendship
            .delete(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(StatusCode::NO_CONTENT);
    }

    let outgoing_request = FriendRequest::find()
        .filter(friend_request::Column::RequesterUserId.eq(&current_user_id))
        .filter(friend_request::Column::ReceiverUserId.eq(&user_id))
        .filter(friend_request::Column::Status.eq(STATUS_PENDING))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(outgoing_request) = outgoing_request {
        let mut active_request: friend_request::ActiveModel = outgoing_request.into();
        active_request.status = Set(STATUS_CANCELLED.to_string());
        active_request.updated_at = Set(chrono::Utc::now().naive_utc());
        active_request
            .update(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(StatusCode::NO_CONTENT);
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn get_friends(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FriendResponse>>, StatusCode> {
    let user_id = claims.sub.clone();
    let friendships = Friendship::find()
        .filter(
            Condition::any()
                .add(friendship::Column::UserAId.eq(&user_id))
                .add(friendship::Column::UserBId.eq(&user_id)),
        )
        .order_by_desc(friendship::Column::CreatedAt)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let friend_user_ids = friendships
        .iter()
        .map(|friendship| {
            if friendship.user_a_id == user_id {
                friendship.user_b_id.clone()
            } else {
                friendship.user_a_id.clone()
            }
        })
        .collect::<Vec<_>>();

    let users_by_id = users_by_id(&state, &friend_user_ids).await?;
    let active_avatars_by_user_id = active_avatars_by_user_id(&state, &friend_user_ids).await?;

    let mut response = Vec::new();
    for friendship in friendships {
        let friend_user_id = if friendship.user_a_id == user_id {
            friendship.user_b_id.clone()
        } else {
            friendship.user_a_id.clone()
        };

        if let Some(user) = users_by_id.get(&friend_user_id) {
            response.push(FriendResponse {
                user: user_to_friend_response(user.clone(), &active_avatars_by_user_id),
                created_at: friendship.created_at.to_string(),
            });
        }
    }

    Ok(Json(response))
}

async fn ensure_friendship(state: &Arc<AppState>, user_a_id: &str, user_b_id: &str) -> Result<(), StatusCode> {
    if find_friendship(state, user_a_id, user_b_id).await?.is_some() {
        return Ok(());
    }

    let (user_a_id, user_b_id) = friendship_pair(user_a_id, user_b_id);
    friendship::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        user_a_id: Set(user_a_id),
        user_b_id: Set(user_b_id),
        created_at: Set(chrono::Utc::now().naive_utc()),
    }
    .insert(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn find_friendship(state: &Arc<AppState>, user_a_id: &str, user_b_id: &str) -> Result<Option<friendship::Model>, StatusCode> {
    let (user_a_id, user_b_id) = friendship_pair(user_a_id, user_b_id);
    Friendship::find()
        .filter(friendship::Column::UserAId.eq(user_a_id))
        .filter(friendship::Column::UserBId.eq(user_b_id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn friendship_pair(user_a_id: &str, user_b_id: &str) -> (String, String) {
    if user_a_id <= user_b_id {
        (user_a_id.to_string(), user_b_id.to_string())
    } else {
        (user_b_id.to_string(), user_a_id.to_string())
    }
}

fn friend_request_pair_condition(user_a_id: &str, user_b_id: &str) -> Condition {
    Condition::any()
        .add(
            Condition::all()
                .add(friend_request::Column::RequesterUserId.eq(user_a_id))
                .add(friend_request::Column::ReceiverUserId.eq(user_b_id)),
        )
        .add(
            Condition::all()
                .add(friend_request::Column::RequesterUserId.eq(user_b_id))
                .add(friend_request::Column::ReceiverUserId.eq(user_a_id)),
        )
}

async fn friend_requests_to_response(state: &Arc<AppState>, requests: Vec<friend_request::Model>) -> Result<Vec<FriendRequestResponse>, StatusCode> {
    let mut response = Vec::with_capacity(requests.len());
    for request in requests {
        response.push(friend_request_to_response(state, request).await?);
    }
    Ok(response)
}

async fn friend_request_to_response(state: &Arc<AppState>, request: friend_request::Model) -> Result<FriendRequestResponse, StatusCode> {
    let requester = User::find_by_id(request.requester_user_id.clone())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let receiver = User::find_by_id(request.receiver_user_id.clone())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let user_ids = vec![requester.id.clone(), receiver.id.clone()];
    let active_avatars_by_user_id = active_avatars_by_user_id(state, &user_ids).await?;

    Ok(FriendRequestResponse {
        id: request.id,
        requester: user_to_friend_response(requester, &active_avatars_by_user_id),
        receiver: user_to_friend_response(receiver, &active_avatars_by_user_id),
        status: request.status,
        created_at: request.created_at.to_string(),
        updated_at: request.updated_at.to_string(),
    })
}

async fn users_by_id(state: &Arc<AppState>, user_ids: &[String]) -> Result<HashMap<String, user::Model>, StatusCode> {
    if user_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let users = User::find()
        .filter(user::Column::Id.is_in(user_ids.to_vec()))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(users.into_iter().map(|user| (user.id.clone(), user)).collect())
}

async fn active_avatars_by_user_id(state: &Arc<AppState>, user_ids: &[String]) -> Result<HashMap<String, avatar::Model>, StatusCode> {
    if user_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let avatars = Avatar::find()
        .filter(avatar::Column::UserId.is_in(user_ids.to_vec()))
        .filter(avatar::Column::IsActive.eq(true))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(avatars.into_iter().map(|avatar| (avatar.user_id.clone(), avatar)).collect())
}

fn user_to_friend_response(user: user::Model, active_avatars_by_user_id: &HashMap<String, avatar::Model>) -> FriendUserResponse {
    let avatar_url = format!("/api/users/{}/profile-pic.svg", user.id);
    let avatar = if let Some(active_avatar) = active_avatars_by_user_id.get(&user.id) {
        FriendAvatarResponse {
            kind: "custom".to_string(),
            base_kind: active_avatar.base_kind.clone(),
            name: active_avatar.name.clone(),
            url: avatar_url,
        }
    } else {
        let base_kind = user.avatar.clone();
        FriendAvatarResponse {
            kind: "default".to_string(),
            name: base_avatar_name(&base_kind).to_string(),
            base_kind,
            url: avatar_url,
        }
    };

    FriendUserResponse {
        id: user.id,
        username: user.username,
        avatar,
    }
}

fn base_avatar_name(base_kind: &str) -> &str {
    match base_kind {
        "steve" => "Steve",
        "alex" => "Alex",
        _ => "Avatar par défaut",
    }
}
