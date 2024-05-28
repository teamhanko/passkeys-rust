use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use http::{header::{CONTENT_TYPE, SET_COOKIE}, HeaderValue};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest;

use crate::model::{Claims, LoginInfo, LoginResponse};
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct User {
    id: &'static str,
    email: &'static str,
    password: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct UserForRegistration {
    pub id: String,
    pub email: String,
}

lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(vec![
        User {
            id: "b3fbbdbd-6bb5-4558-9055-3b54a9469629",
            email: "john.doe@example.com",
            password: "password123",
        },
        User {
            id: "22c81b3d-1e7d-4a72-a6b0-ad946e0c0965",
            email: "sergio_mq@example.com",
            password: "very_secure_password",
        },
        User {
            id: "55c81b3d-1e7d-4a72-a6b0-ad946e0c0965",
            email: "ab@g.com",
            password: "123",
        },
    ]);
}

pub async fn hello_world_handler() -> Json<&'static str> {
    Json("Hello, World!")
}

pub async fn login_handler(Json(login_info): Json<LoginInfo>) -> Result<Response, StatusCode> {
    let useremail = &login_info.email;
    let password = &login_info.password;

    let is_valid = is_valid_user(useremail, password);

    if is_valid {
        let claims = Claims {
            sub: useremail.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let token: String = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        ) {
            Ok(tok) => tok,
            Err(e) => {
                eprintln!("Error generating token: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let mut headers = HeaderMap::new();
        let cookie_value = format!("token={}; HttpOnly; Path=/; Max-Age=86400", token);
        headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie_value).unwrap());

        Ok((headers, Json(LoginResponse { token })).into_response())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn logout_handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let cookie_value = "token=; HttpOnly; Path=/; Max-Age=0";
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie_value).unwrap());

    (headers, Json("Logged out"))
}

pub fn is_valid_user(useremail: &str, password: &str) -> bool {
    let users = USERS.lock().unwrap();
    users
        .iter()
        .any(|user| user.email == useremail && user.password == password)
}

pub async fn get_info_handler(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();
                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::default(),
                ) {
                    Ok(_) => {
                        let info = "You are valid, here is your info".to_string();
                        return Ok(Json(info));
                    }
                    Err(e) => {
                        eprintln!("Error decoding token: {}", e);
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

// passkeys

const TENANT_ID: &str = "03915657-8161-4f7a-8170-70df0e370114";
const API_KEY: &str = "xYyhiHZ8s9WbTH9ZZBSPgwgyr2TF-apKNoOdbZERspivln0tZTCDeMJTJ_Yfw9yx1WTUS7qUQfviizwpHFV2Kw==";
const BASE_URL: &str = "https://passkeys.hanko.io";

pub async fn start_registration_handler(Json(user): Json<UserForRegistration>) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}/registration/initialize", BASE_URL, TENANT_ID);
    let payload = json!({ "user_id": "55c81b3d-1e7d-4a72-a6b0-ad946e0c0965", "username": "ab@g.com" });

    let mut headers = HeaderMap::new();
    headers.insert("apikey", HeaderValue::from_static(API_KEY));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = client.post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let creation_options: serde_json::Value = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(creation_options))
}

pub async fn finalize_registration_handler(Json(data): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}/registration/finalize", BASE_URL, TENANT_ID);

    let mut headers = HeaderMap::new();
    headers.insert("apikey", HeaderValue::from_static(API_KEY));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = client.post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}


pub async fn start_login_handler() -> Result<Json<LoginOptions>, StatusCode> {
    let client = reqwest::Client::new();
    let url = format!("{}/login/initialize", BASE_URL);
    let headers = [("Content-Type", "application/json")]; // Add more headers as needed

    let res = client.post(&url)
        .headers(headers.into_iter().map(|(k, v)| (k, HeaderValue::from_static(v))).collect())
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = res.text().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let login_options: LoginOptions = serde_json::from_str(&body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(login_options))
}

pub async fn finalize_login_handler(Json(client_data): Json<ClientData>) -> Result<Response, StatusCode> {
    let client = reqwest::Client::new();
    let url = format!("{}/login/finalize", BASE_URL);
    let headers = [("Content-Type", "application/json")]; // Add more headers as needed

    let res = client.post(&url)
    .headers(headers.into_iter().map(|(k, v)| (http::header::HeaderName::from_static(k), HeaderValue::from_static(v))).collect())
        .json(&client_data)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = res.text().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result: serde_json::Value = serde_json::from_str(&body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = result.get("token").and_then(|t| t.as_str()).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let decoded = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let claims = decoded.claims;
    let user_id = claims.sub;

    let new_token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&format!("token={}; HttpOnly; Path=/; Max-Age=86400", new_token)).unwrap());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .headers(headers)
        .body(Json(json!({"message": "Login successful"})).into())
        .unwrap())
}



