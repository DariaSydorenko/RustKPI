use actix_web::{HttpResponse, web, Responder};
use actix_session::Session;
use serde::Deserialize;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct ChatMessage {
    text: String,
}

lazy_static! {
    static ref USERS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// Головна сторінка
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <h1>Welcome to the Chat!</h1>
            <button onclick="window.location.href='/login'">Login</button>
            <button onclick="window.location.href='/register'">Register</button>
        "#)
}

// Сторінка входу
pub async fn login_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <h2>Login</h2>
            <form action="/login" method="post">
                <label for="username">Username:</label>
                <input type="text" id="username" name="username" required>
                <label for="password">Password:</label>
                <input type="password" id="password" name="password" required>
                <button type="submit">Login</button>
            </form>
        "#)
}

// Сторінка реєстрації
pub async fn register_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <h2>Register</h2>
            <form action="/register" method="post">
                <label for="username">Username:</label>
                <input type="text" id="username" name="username" required>
                <label for="password">Password:</label>
                <input type="password" id="password" name="password" required>
                <button type="submit">Register</button>
            </form>
        "#)
}

// Обробка входу
pub async fn login(session: Session, user: web::Form<User>) -> impl Responder {
    let users = USERS.lock().unwrap();
    if let Some(stored_password) = users.get(&user.username) {
        if &user.password == stored_password {
            session.insert("user_id", &user.username).unwrap();
            return HttpResponse::Found()
                .header("Location", "/chat")
                .finish();
        } else {
            return HttpResponse::Unauthorized().body("Incorrect password.");
        }
    }
    HttpResponse::NotFound().body("User not found. Please register.")
}

// Обробка реєстрації
pub async fn register(user: web::Form<User>) -> impl Responder {
    let mut users = USERS.lock().unwrap();
    if users.contains_key(&user.username) {
        return HttpResponse::Conflict().body("User already exists.");
    }
    users.insert(user.username.clone(), user.password.clone());
    HttpResponse::Ok().body("Registration successful! You can now log in.")
}

// Сторінка чату
pub async fn chat(session: Session) -> impl Responder {
    if session.get::<String>("user_id").unwrap_or(None).is_some() {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(r#"
                <h2>Welcome to the Chat!</h2>
                <form action="/send_message" method="post">
                    <label for="text">Message:</label>
                    <input type="text" id="text" name="text" required>
                    <button type="submit">Send</button>
                </form>
                <button onclick="window.location.href='/'">Logout</button>
            "#)
    } else {
        HttpResponse::Unauthorized()
            .body("Unauthorized. Please log in to access the chat.")
    }
}

// Надсилання повідомлення
pub async fn send_message(session: Session, message: web::Form<ChatMessage>) -> impl Responder {
    if session.get::<String>("user_id").unwrap_or(None).is_some() {
        HttpResponse::Ok().body(format!("Message sent: {}", message.text))
    } else {
        HttpResponse::Unauthorized().body("Unauthorized. Please log in to send messages.")
    }
}