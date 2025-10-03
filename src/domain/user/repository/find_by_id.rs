use crate::domain::user::entities::User;
use crate::domain::user::repository::RepositoryError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn find_user_by_id(
    users: Arc<RwLock<HashMap<uuid::Uuid, User>>>,
    id: uuid::Uuid,
) -> Result<Option<User>, RepositoryError> {
    let user_map = users.read().await;
    Ok(user_map.get(&id).cloned())
}