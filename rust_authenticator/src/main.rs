use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm, TokenData};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const JWKS_URL: &str = "http://localhost:3000/.well-known/jwks.json";

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    n: String,
    e: String,
    alg: Option<String>,
    use_: Option<String>, // "use" is a reserved word
}

#[derive(Debug, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    iss: String,
    exp: usize,
    role: Option<String>,
    user_id: Option<u64>,
}

async fn fetch_jwks() -> Result<Jwks, String> {
    let res = reqwest::get(JWKS_URL)
        .await
        .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

    res.json::<Jwks>()
        .await
        .map_err(|e| format!("Failed to parse JWKS: {}", e))
}

async fn validate_jwt(token: &str) -> Result<TokenData<Claims>, String> {
    let header = decode_header(token).map_err(|e| format!("Invalid token header: {}", e))?;
    let kid = header.kid.ok_or("Missing kid in JWT header")?;

    let jwks = fetch_jwks().await?;
    let jwk = jwks
        .keys
        .iter()
        .find(|j| j.kid == kid)
        .ok_or("No matching JWK found for kid")?;

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|e| format!("Invalid RSA key: {}", e))?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["your-app"]);
    validation.set_issuer(&["your-auth-server"]);

    decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| format!("Token validation failed: {}", e))
}

#[get("/secure")]
async fn secure(req: HttpRequest) -> impl Responder {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
    if token.is_empty() {
        return HttpResponse::Unauthorized().body("Missing token");
    }

    match validate_jwt(token).await {
        Ok(data) => HttpResponse::Ok().json(data.claims),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");

    HttpServer::new(|| App::new().service(secure))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
