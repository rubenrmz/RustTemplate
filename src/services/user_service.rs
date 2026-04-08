// src/services/user_service.rs
use uuid::Uuid;

use crate::dto::user_dto::{CreateUserDto, UpdateUserDto, UserListParams, UserListResponse, UserResponse};
use crate::errors::AppError;
use crate::models::user::User;
use crate::state::AppState;
use crate::store::user_store;
use crate::utils::password;

pub async fn list(state: &AppState, params: UserListParams) -> Result<UserListResponse, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let (users, total) = user_store::list(
        &state.db,
        per_page,
        offset,
        params.search.as_deref(),
    )
    .await?;

    Ok(UserListResponse {
        data: users.into_iter().map(to_response).collect(),
        total,
        page,
        per_page,
    })
}

pub async fn get_by_id(state: &AppState, id: Uuid) -> Result<UserResponse, AppError> {
    let user = user_store::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(to_response(user))
}

pub async fn create(state: &AppState, dto: CreateUserDto) -> Result<UserResponse, AppError> {
    let hash = password::hash(&dto.password)?;
    let role = dto.role.as_deref().unwrap_or("user");

    let email = dto.email.to_lowercase();
    let user = user_store::create_with_role(&state.db, &dto.name, &email, &hash, role).await?;

    Ok(to_response(user))
}

pub async fn update(state: &AppState, id: Uuid, dto: UpdateUserDto) -> Result<UserResponse, AppError> {
    let hash = match &dto.password {
        Some(pwd) => Some(password::hash(pwd)?),
        None => None,
    };

    let email = dto.email.as_deref().map(|e| e.to_lowercase());
    let user = user_store::update(
        &state.db,
        id,
        dto.name.as_deref(),
        email.as_deref(),
        hash.as_deref(),
        dto.role.as_deref(),
        dto.active,
    )
    .await?;

    Ok(to_response(user))
}

pub async fn delete(state: &AppState, id: Uuid) -> Result<(), AppError> {
    user_store::delete(&state.db, id).await
}

fn to_response(user: User) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        role: user.role,
        active: user.active,
        created_at: user.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        updated_at: user.updated_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    }
}
