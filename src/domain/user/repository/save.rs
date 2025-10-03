use crate::domain::user::entities::User;
use crate::domain::user::repository::RepositoryError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn save_user(
    users: Arc<RwLock<HashMap<uuid::Uuid, User>>>,
    user: &User,
) -> Result<(), RepositoryError> {
    let mut user_map = users.write().await;
    user_map.insert(user.id, user.clone());
    Ok(())
}