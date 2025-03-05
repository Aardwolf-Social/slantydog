use crate::direct_messages::PrivateMessageReply;
use crate::endpoints::{create_post, get_posts};
use aardwolf_api_common::models::PostData;
use aardwolf_api_common::Post;
use actix_web::body::BoxBody;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

/// Creates a new post and returns the created post as a JSON response.
async fn create_post_actix(data: web::Json<PostData>) -> HttpResponse {
    match create_post(data.into_inner()).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

async fn get_posts_actix() -> HttpResponse {
    let posts = get_posts().await;
    HttpResponse::Ok().json(posts)
}

/// A trait for converting a post to an HTTP response.
trait PostResponder {
    /// Converts a post to an HTTP response.
    fn as_response(&self, post: &Post) -> HttpResponse;
}

impl<F> PostResponder for F
where
    F: Fn(&Post) -> HttpResponse,
{
    fn as_response(&self, post: &Post) -> HttpResponse {
        self(post)
    }
}

impl Responder for PrivateMessageReply {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(&self.0)
    }
}