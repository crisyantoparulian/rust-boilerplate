use crate::domain::user::entities::User;
use crate::domain::user::repository::RepositoryError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn list_users(
    users: Arc<RwLock<HashMap<uuid::Uuid, User>>>,
    page: u32,
    limit: u32,
) -> Result<(Vec<User>, u64), RepositoryError> {
    let user_map = users.read().await;
    let user_list: Vec<User> = user_map.values().cloned().collect();
    let total = user_list.len() as u64;

    let offset = ((page - 1) * limit) as usize;
    let end = std::cmp::min(offset + limit as usize, user_list.len());

    if offset >= user_list.len() {
        return Ok((vec![], total));
    }

    let paginated_users = user_list[offset..end].to_vec();
    Ok((paginated_users, total))
}