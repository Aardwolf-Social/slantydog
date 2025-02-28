// Actix-specific implementation of the endpoints
use aardwolf_api_common::models::PostData;
use actix_web::{web, HttpResponse, Responder};
use crate::backend::endpoints;

async fn create_post_actix(data: PostData) -> impl Responder {
    let post = endpoints::create_post(data).await;
    HttpResponse::Created().json(post)
}

async fn get_posts_actix() -> impl Responder {
    let posts = endpoints::get_posts().await;
    HttpResponse::Ok().json(posts)
}