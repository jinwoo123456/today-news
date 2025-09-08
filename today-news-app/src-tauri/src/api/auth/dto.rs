use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupReq {
    pub name : String,
    pub email : String,
    pub password : String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupRes {
    pub name : String,
    pub email : String,
    pub password : String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginReq {
    pub email : String,
    pub password : String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginRes {
    pub email : String,
    pub token : String,
}