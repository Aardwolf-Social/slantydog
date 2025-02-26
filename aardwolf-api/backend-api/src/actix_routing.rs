// actix_routing

use super::backend_endpoints;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/auth/login")
            .route(web::get().to(backend_endpoints::login)),
    )
        .service(
            web::resource("/api/users")
                .route(web::get().to(backend_endpoints::get_users)),
        )
        .service(
            web::resource("/api/users/{userId}")
                .route(web::get().to(backend_endpoints::get_user))
                .route(web::put().to(backend_endpoints::update_user))
                .route(web::delete().to(backend_endpoints::delete_user)),
        )
        .service(
            web::resource("/api/posts")
                .route(web::get().to(backend_endpoints::get_posts)),
        );
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}