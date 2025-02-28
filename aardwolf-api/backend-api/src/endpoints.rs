// Backend-agnostic endpoint definitions
async fn create_post(data: PostData) -> Post {
    // Logic to create a new post (replace this with actual implementation)
    Post { id: 1, title: "New Post".to_string(), content: "This is a new post".to_string() }
}

async fn get_posts() -> Vec<Post> {
    // Logic to retrieve a list of posts (replace this with actual implementation)
    vec![Post { id: 1, title: "Post 1".to_string(), content: "This is post 1".to_string() }]
}