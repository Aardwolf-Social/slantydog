// backend-api/src/routes/responses.rs
pub trait Response {
    type Output;

    fn into_response(self) -> Self::Output;
}

#[derive(Debug)]
pub struct ErrorResponse {
    pub message: String,
}

impl std::error::Error for ErrorResponse {}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
