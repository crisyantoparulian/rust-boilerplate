use crate::domain::user::repository::RepositoryError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn user_exists_by_email(
    users: Arc<RwLock<HashMap<uuid::Uuid, crate::domain::user::entities::User>>>,
    email: &str,
) -> Result<bool, RepositoryError> {
    let user_map = users.read().await;
    Ok(user_map.values().any(|user| user.email == email))
}