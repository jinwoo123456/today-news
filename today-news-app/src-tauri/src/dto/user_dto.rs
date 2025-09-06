use serde:;{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct SignUpResponse {
    pub success: String,
    pub username: String,
    pub email: String,
}
