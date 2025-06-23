use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// In a real application, load this from an environment variable.
const JWT_SECRET: &[u8] = b"this-is-a-very-secret-key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,        // Subject (user id)
    exp: usize,         // Expiration time
    iat: usize,         // Issued at
    token_type: String, // "access" or "refresh"
}

#[derive(Deserialize)]
struct IssueJwtRequest {
    user_id: String,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Serialize)]
struct NewTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct VerifyRequest {
    token: String,
}

#[derive(Serialize)]
struct VerifyResponse {
    valid: bool,
    claims: Option<Claims>,
}

fn create_jwt(
    user_id: &str,
    expiration: Duration,
    token_type: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_owned(),
        iat: now.timestamp() as usize,
        exp: (now + expiration).timestamp() as usize,
        token_type: token_type.to_string(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

async fn issue_jwt_handler(req: web::Json<IssueJwtRequest>) -> impl Responder {
    let access_token = match create_jwt(&req.user_id, Duration::minutes(1), "access") {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let refresh_token = match create_jwt(&req.user_id, Duration::hours(1), "refresh") {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(TokenResponse {
        access_token,
        refresh_token,
    })
}

async fn refresh_jwt_handler(req: web::Json<RefreshRequest>) -> impl Responder {
    let validation = Validation::default();
    match decode::<Claims>(
        &req.refresh_token,
        &DecodingKey::from_secret(JWT_SECRET),
        &validation,
    ) {
        Ok(token_data) => {
            if token_data.claims.token_type != "refresh" {
                return HttpResponse::Unauthorized().finish();
            }
            let new_access_token =
                match create_jwt(&token_data.claims.sub, Duration::minutes(1), "access") {
                    Ok(token) => token,
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                };
            HttpResponse::Ok().json(NewTokenResponse {
                access_token: new_access_token,
            })
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

async fn verify_jwt_handler(req: web::Json<VerifyRequest>) -> impl Responder {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        &req.token,
        &DecodingKey::from_secret(JWT_SECRET),
        &validation,
    );

    match token_data {
        Ok(data) => {
            if data.claims.token_type == "access" {
                HttpResponse::Ok().json(VerifyResponse {
                    valid: true,
                    claims: Some(data.claims),
                })
            } else {
                HttpResponse::Ok().json(VerifyResponse {
                    valid: false,
                    claims: None,
                })
            }
        }
        Err(_) => HttpResponse::Ok().json(VerifyResponse {
            valid: false,
            claims: None,
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/issue-jwt", web::post().to(issue_jwt_handler))
            .route("/refresh-jwt", web::post().to(refresh_jwt_handler))
            .route("/verify-jwt", web::post().to(verify_jwt_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
