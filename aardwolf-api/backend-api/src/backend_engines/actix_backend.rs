// aardwolf-api/src/backend_engines/actix_backend.rs
use aardwolf_api_common::models::direct_messages::{PrivateMessage, PrivateMessageReply}; // ✅ Use common direct_messages module
use crate::routes::posts::{create_post, get_posts};
use aardwolf_api_common::models::posts::PostImpl;
use actix_web::body::BoxBody;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};

/// Creates a new post and returns the created post as JSON.
async fn create_post_actix(data: web::Json<PostImpl>) -> HttpResponse {
    match create_post(data.into_inner()).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

/// Retrieves all posts as JSON.
async fn get_posts_actix() -> HttpResponse {
    match get_posts().await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

/// Sends a new private message.
async fn send_private_message(data: web::Json<PrivateMessage>) -> HttpResponse {
    HttpResponse::Ok().json(data.into_inner()) // ✅ Placeholder response for now
}

/// Retrieves a private message reply.
async fn get_private_message_reply(data: web::Json<PrivateMessage>) -> HttpResponse {
    let reply = PrivateMessageReply(data.into_inner());
    HttpResponse::Ok().json(reply) // ✅ Placeholder response for now
}

impl Responder for PrivateMessageReply {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(&self.0)
    }
}

/// Configures Actix routes.
pub fn configure_routes() -> Scope {
    web::scope("/api")
        .route("/posts", web::get().to(get_posts_actix))
        .route("/posts", web::post().to(create_post_actix))
        .route("/messages", web::post().to(send_private_message))
        .route("/messages/reply", web::post().to(get_private_message_reply))
}
