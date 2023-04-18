use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnauthorizedError {
    pub name: String,
    pub message: String,
    pub code: u16,
    pub status: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotFound {
    pub name: String,
    pub message: String,
    pub code: u16,
    pub status: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BadRequest {
    name: String,
    message: String,
    code: u16,
    status: u16,
}
