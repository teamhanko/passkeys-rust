use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// // Define a static list of users
// static USERS: &[User] = &[
//     User {
//         id: "b3fbbdbd-6bb5-4558-9055-3b54a9469629",
//         email: "john.doe@example.com",
//         password: "password123",
//     },
//     User {
//         id: "22c81b3d-1e7d-4a72-a6b0-ad946e0c0965",
//         email: "sergio_mq@example.com",
//         password: "very_secure_password",
//     },
//     User {
//         id: "55c81b3d-1e7d-4a72-a6b0-ad946e0c0965",
//         email: "ab@g.com",
//         password: "123",
//     },
// ];

// #[derive(Debug, Clone)]
// pub struct User {
//     pub id: String,
//     pub email: String,
//     pub password: String,
// }
