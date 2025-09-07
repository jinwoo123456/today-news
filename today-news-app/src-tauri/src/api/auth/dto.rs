use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SignupReq {
    name : String,
    email : String,
    password : String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SignupRes {
    name : String,
    email : String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct LoginReq {
    email : String,
    password : String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct LoginRes {
    email : String,
    token : String,
}