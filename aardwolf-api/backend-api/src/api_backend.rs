// api_backend
mod actix_routing;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_routing::main().await
}