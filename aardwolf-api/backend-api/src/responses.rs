// responses.rs
pub trait Response {
    fn into_response(self) -> Self;
}

// Other response-related types or traits can go here...