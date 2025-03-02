use aardwolf_api_common::{Post, PostData};

// Backend-agnostic endpoint definitions
pub(crate) async fn create_post(data: PostData) -> Post {
    // Logic to create a new post (replace this with actual implementation)
    Post { id: 1, title: "New Post".to_string(), content: "This is a new post".to_string(), created_at: "".to_string(), updated_at: "".to_string() }
}

pub(crate) async fn get_posts() -> Vec<Post> {
    // Logic to retrieve a list of posts (replace this with actual implementation)
    vec![Post { id: 1, title: "Post 1".to_string(), content: "This is post 1".to_string(), created_at: "".to_string(), updated_at: "".to_string() }]
}