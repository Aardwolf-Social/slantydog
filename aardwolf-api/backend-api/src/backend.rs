// Main file for the backend API
pub mod endpoints;
pub mod actix_backend;
pub mod warp_backend;

use aardwolf_api_common::models::{PostData, Post};


fn main() {
    // Use the backend implementation based on the feature
}