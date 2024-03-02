mod protected;
use super::types::db_interaction::User;
use crate::server::types::export::Post;
use crate::store;
pub use protected::*;

pub async fn fetch_id_by_login(login: &str) -> Result<Option<String>, String> {
    let path_suffix = format!("users/?login={}", &login);
    store::fetch::<Vec<User>>(&path_suffix)
        .await
        .map(|users_vec| users_vec.into_iter().next().map(|user| user.id))
}

pub async fn fetch_post(post_id: &str) -> Result<Post, String> {
    let path_suffix = format!("posts/{post_id}");
    store::fetch::<Post>(&path_suffix).await
}
