// users.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub struct UserData {
    pub id: i32,
    pub name: String,
    pub email: String,  
} 