use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng as ArgonOsRng,
    },
};

use axum::{
    Form, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header::SET_COOKIE},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use clap::Parser;
use hmac::{Hmac, Mac};
use rand::distr::{Alphanumeric, SampleString};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};
use std::time::Duration;

type HmacSha256 = Hmac<Sha256>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long)]
    flags: Option<String>,

    #[arg(short, long, default_value = "3000")]
    port: u16,
}

#[derive(Clone)]
struct AppState {
    users: Arc<HashMap<String, UserData>>,
    server_secret: Arc<[u8; 32]>,
    sessions: Arc<RwLock<HashMap<String, String>>>,
}

#[derive(Clone, Debug)]
struct UserData {
    username: String,
    password_hash: String,
    flag: String,
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: Option<String>,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct FlagResponse {
    success: bool,
    flag: Option<String>,
}

const TEST_USERNAME: &str = "testuser";
const TEST_PASSWORD: &str = "TestPassword123";
const PASSWORD_LENGTH: usize = 16;

fn extract_session_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("cookie")
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find_map(|cookie| cookie.trim().strip_prefix("session_token="))
        })
}

fn generate_session_token(username: &str, server_secret: &[u8; 32]) -> String {
    let mut mac = HmacSha256::new_from_slice(server_secret).expect("HMAC can take key of any size");
    mac.update(username.as_bytes());
    let full = mac.finalize().into_bytes();
    let mac_hex = hex::encode(&full[..8]);
    format!("{username}:{mac_hex}")
}

fn parse_session_token(token: &str) -> Option<(&str, &str)> {
    token.split_once(':')
}

fn generate_password() -> String {
    loop {
        let candidate = Alphanumeric.sample_string(&mut rand::rng(), PASSWORD_LENGTH);
        let has_lower = candidate.chars().any(|c| c.is_ascii_lowercase());
        let has_upper = candidate.chars().any(|c| c.is_ascii_uppercase());
        let has_digit = candidate.chars().any(|c| c.is_ascii_digit());
        if has_lower && has_upper && has_digit {
            return candidate;
        }
    }
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut ArgonOsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn parse_csv(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, ',').collect();
        if parts.len() == 2 {
            let username = parts[0].trim().to_string();
            let flag = parts[1].trim().to_string();
            map.insert(username, flag);
        }
    }
    map
}
// Can you see the issue?
async fn string_compare(input: &str, expected: &str) -> bool {
    if input.len() != expected.len() {
        return false;
    }
    let input_bytes = input.as_bytes();
    let expected_bytes = expected.as_bytes();
    for i in 0..input_bytes.len() {
        if input_bytes[i] != expected_bytes[i] {
            return false;
        }
        // Add extra timing to make task easier
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    true
}

fn password_success_response(user: &UserData, token: &str, state: &AppState) -> Response {
    let mut sessions = state.sessions.write().unwrap();
    sessions.insert(token.to_string(), user.username.clone());
    drop(sessions);

    let body = axum::Json(LoginResponse {
        success: true,
        message: format!("Login successful! Welcome, {}", user.username),
    });
    let mut response = (StatusCode::OK, body).into_response();
    let cookie_value = format!(
        "session_token={}; Max-Age=3600; HttpOnly; SameSite=Lax; Path=/",
        token
    );
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie_value).expect("invalid cookie value"),
    );
    response
}

async fn index(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let template = include_str!("../templates/index.html");

    if let Some(session_token) = extract_session_token(&headers)
        && let Some((username, _)) = parse_session_token(session_token)
        && let Some(user) = state.users.get(username)
    {
        let expected_token = generate_session_token(&user.username, &state.server_secret);
        if string_compare(session_token, &expected_token).await {
            let sessions = state.sessions.read().unwrap();
            let is_active = sessions.contains_key(session_token);
            drop(sessions);

            if is_active {
                let html = template
                    .replace("{{TITLE}}", &format!("Welcome, {}!", user.username))
                    .replace("{{LOGIN_HIDDEN}}", "hidden")
                    .replace("{{LOGGED_IN_HIDDEN}}", "");
                return Html(html).into_response();
            }
        }
    }

    let html = template
        .replace("{{TITLE}}", "Secure Login Portal")
        .replace("{{LOGIN_HIDDEN}}", "")
        .replace("{{LOGGED_IN_HIDDEN}}", "hidden");
    Html(html).into_response()
}

async fn login(State(state): State<AppState>, Form(form): Form<LoginForm>) -> Response {
    let user = match state.users.get(&form.username) {
        Some(u) => u,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(LoginResponse {
                    success: false,
                    message: "Invalid username or password".to_string(),
                }),
            )
                .into_response();
        }
    };

    if let Some(password) = &form.password {
        if verify_password(password, &user.password_hash) {
            let token = generate_session_token(&user.username, &state.server_secret);
            return password_success_response(user, &token, &state);
        } else {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(LoginResponse {
                    success: false,
                    message: "Invalid username or password".to_string(),
                }),
            )
                .into_response();
        }
    }

    (
        StatusCode::BAD_REQUEST,
        axum::Json(LoginResponse {
            success: false,
            message: "Please provide password".to_string(),
        }),
    )
        .into_response()
}

fn flag_error() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        axum::Json(FlagResponse {
            success: false,
            flag: None,
        }),
    )
        .into_response()
}

async fn flag(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let Some(token) = extract_session_token(&headers) else {
        return flag_error();
    };

    let Some((username, _)) = parse_session_token(token) else {
        return flag_error();
    };

    let Some(user) = state.users.get(username) else {
        return flag_error();
    };

    let expected_token = generate_session_token(&user.username, &state.server_secret);
    if !string_compare(token, &expected_token).await {
        return flag_error();
    }

    let sessions = state.sessions.read().unwrap();
    if !sessions.contains_key(token) {
        return flag_error();
    }

    (
        StatusCode::OK,
        axum::Json(FlagResponse {
            success: true,
            flag: Some(user.flag.clone()),
        }),
    )
        .into_response()
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let mut active = false;
    if let Some(token) = extract_session_token(&headers) {
        let sessions = state.sessions.read().unwrap();
        active = sessions.get(token).is_some();
    }

    let mut response = (
        StatusCode::OK,
        axum::Json(LoginResponse {
            success: true,
            message: if active {
                "Logged out successfully".to_string()
            } else {
                "No active session found".to_string()
            },
        }),
    )
        .into_response();

    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_static("session_token=; Max-Age=0; Path=/; HttpOnly; SameSite=Lax"),
    );

    response
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut flags_map = HashMap::new();
    if let Some(csv_path) = &args.flags {
        if let Ok(content) = fs::read_to_string(csv_path) {
            flags_map = parse_csv(&content);
            println!("Loaded {} users from CSV", flags_map.len());
        } else {
            eprintln!("Warning: Could not read CSV file: {}", csv_path);
        }
    }

    let mut server_secret = [0u8; 32];
    rand::rng().fill_bytes(&mut server_secret);

    let mut users = HashMap::new();
    let mut sessions = HashMap::new();

    let password_hash = hash_password(TEST_PASSWORD);
    let session_token = generate_session_token(TEST_USERNAME, &server_secret);
    sessions.insert(session_token, TEST_USERNAME.to_string());
    let sample_flag = "flag{THIS_IS_A_TEST_FLAG}";

    users.insert(
        TEST_USERNAME.to_string(),
        UserData {
            username: TEST_USERNAME.to_string(),
            password_hash,
            flag: sample_flag.to_string(),
        },
    );

    println!("Additional users:");
    println!("---");
    for username in flags_map.keys() {
        let password = generate_password();
        let password_hash = hash_password(&password);
        let flag = flags_map
            .get(username)
            .cloned()
            .unwrap_or_else(|| format!("FLAG{{{}}}", username));

        let session_token = generate_session_token(username, &server_secret);

        println!("Username: {}", username);
        println!("Password: {}", password);
        println!("Session Token: {}", session_token);
        println!("---");
        sessions.insert(session_token, username.clone());

        users.insert(
            username.clone(),
            UserData {
                username: username.clone(),
                password_hash,
                flag,
            },
        );
    }

    let state = AppState {
        users: Arc::new(users),
        server_secret: Arc::new(server_secret),
        sessions: Arc::new(RwLock::new(sessions)),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/flag", get(flag))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", args.port);
    println!("\nServer running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Server failed");
}
