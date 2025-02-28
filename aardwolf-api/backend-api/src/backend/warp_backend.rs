// Warp-specific implementation of the endpoints
use warp::Filter;
use crate::backend::endpoints;

async fn create_post_warp(data: PostData) -> Result<impl Reply, Rejection> {
    let post = endpoints::create_post(data).await;
    Ok(post.into_response())
}

async fn get_posts_warp() -> Result<impl Reply, Rejection> {
    let posts = endpoints::get_posts().await;
    Ok(posts.into_response())
}