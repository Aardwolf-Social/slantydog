// aardwolf-api/src/main.rs

extern crate aardwolf_api_common as common;
use common::Post;

fn main() {
    // Test the models module
    let post = common::PostImpl {
        id: 1,
        title: "Test Post".to_string(),
        content: "This is a test post".to_string(),
    };
    println!("Post Data: {:?}", post_data);

    let post = Post {
        id: 1,
        title: "Test Post".to_string(),
        content: "This is a test post".to_string(),
    };
    println!("Post: {:?}", post);

    // Test other modules here...
}