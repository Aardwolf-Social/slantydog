// Common API Models
pub mod posts;
pub mod users;

pub use serde::{Deserialize, Serialize};
pub use posts::{Post, PostData};
pub use users::{User, UserData};