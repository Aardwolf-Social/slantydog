// aardwolf-api/frontend-api/src/api_user_routes.rs

use crate::routes;
use crate::handlers;

// Define the main function to create the HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/users").route(web::get().to(handlers::get_users)))
            .configure(|cfg| {
                cfg.service(
                    web::resource("/api/users/:userId/posts")
                        .route(web::get().to(handlers::get_user_posts)),
                )
                .service(
                    web::resource("/api/users/:userId/followers")
                        .route(web::get().to(handlers::get_user_followers)),
                )
                .service(
                    web::resource("/api/users/follow/:userId")
                        .route(web::get().to(handlers::follow_user)),
                )
                .service(
                    web::resource("/api/users/unfollow/:userId")
                        .route(web::get().to(handlers::unfollow_user)),
                );
            })
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}