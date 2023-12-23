mod protected;
pub use super::types::export_types::{Role, User};
use crate::api_utils;
pub use api_utils::test;

pub use protected::*;


pub async fn fetch_user_id (login: &str) -> Option<String> {
    api_utils::user_by_login::<User>(login).await.map(|user| user.id)
}