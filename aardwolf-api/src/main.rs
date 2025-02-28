fn main() {
    // Test the models module
    let post_data = models::PostData {
        title: "Test Post".to_string(),
        content: "This is a test post".to_string(),
    };
    println!("Post Data: {:?}", post_data);

    let post = models::Post {
        id: 1,
        title: "Test Post".to_string(),
        content: "This is a test post".to_string(),
    };
    println!("Post: {:?}", post);

    // Test other modules here...
}