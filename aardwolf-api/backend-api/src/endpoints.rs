// Backend-agnostic endpoint definitions
use aardwolf_api_common::models::posts::{Post, PostImpl};
use chrono::Utc;
use std::sync::Arc;

pub(crate) async fn create_post(post: PostImpl) -> Result<Arc<dyn Post>, String> {
    Ok(Arc::new(PostImpl {
        id: 1,
        title: post.title,
        content: post.content,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    }))
}

pub(crate) async fn get_posts() -> Vec<Arc<dyn Post>> {
    vec![Arc::new(PostImpl {
        id: 1,
        title: "Post 1".to_string(),
        content: "This is post 1".to_string(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    })]
}
