// Warp-specific endpoint definitions
use warp::{Filter, Rejection, Reply};
use warp::reject::Reject;
use aardwolf_api_common::PostData;
use crate::direct_messages::PrivateMessageReply;
use crate::endpoints::{create_post, get_posts};

async fn create_post_warp(data: PostData) -> Result<impl Reply, Rejection> {
    match create_post(data).await {
        Ok(post) => Ok(warp::reply::json(&post)),
        Err(err) => Err(warp::reject::custom(ErrorResponse {
            message: err,
        })),
    }
}

async fn get_posts_warp() -> Result<impl Reply, Rejection> {
    let posts = get_posts().await;
    Ok(warp::reply::json(&posts))
}

pub async fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let create_post_route = warp::post()
        .and(warp::path("posts"))
        .and(warp::body::json())
        .then(create_post_warp)
        .recover(handle_rejection);

    let get_posts_route = warp::get()
        .and(warp::path("posts"))
        .then(get_posts_warp)
        .recover(handle_rejection);

    create_post_route.or(get_posts_route)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    // Handle the error here
    Ok(warp::reply::with_status(
        "Internal Server Error",
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

#[tokio::main]
async fn main() {
    let routes = routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug)]
struct ErrorResponse {
    message: String,
}

impl Reject for ErrorResponse {}

impl Reply for PrivateMessageReply {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self.0).into_response()
    }
}