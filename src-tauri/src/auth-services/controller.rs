use bcrypt::{hash, verify, DEFAULT_COST};
use crate::{AppState};
use crate::model::users::{NewUser,LoginRequest};
use crate::auth_services::{repository, service};

pub async fn register_user(user: NewUser) -> axum::Json<String> {
    let hashed = hash(user.password, DEFAULT_COST).unwrap();
    let user = NewUser {
        password: hashed,
        ..user
    };
    service::insert_new_user(user).await;
    axum::Json("User registered successfully".to_string())
}

pub async fn login_user(login: LoginRequest, state: &AppState) -> axum::Json<String> {
    let found_user = repository::get_user_by_email(&login.email, state).await;
    if let Some(user) = found_user {
        if verify(&login.password, &user.password).unwrap() {
            return axum::Json("Login successful".to_string());
        }
    }
    axum::Json("Invalid credentials".to_string())
}

pub async fn login_by_google(state: &AppState) -> axum::Json<String>{
    axum::Json("Failed to login".to_string())
}
