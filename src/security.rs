use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
};
use std::env;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};

// --- Middleware for Admin ---

pub async fn verify_admin(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let admin_key = env::var("ADMIN_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let provided_key = headers
        .get("X-Admin-Key")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if provided_key != admin_key {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

// --- Google Play Integrity ---

#[derive(Deserialize)]
struct GoogleTokenResponse {
    tokenPayloadExternal: Option<String>,
    requestDetails: Option<RequestDetails>,
    appIntegrity: Option<AppIntegrity>,
    deviceIntegrity: Option<DeviceIntegrity>,
    accountIntegrity: Option<AccountIntegrity>,
}

#[derive(Deserialize)]
struct RequestDetails {
    requestPackageName: String,
}

#[derive(Deserialize)]
struct AppIntegrity {
    appRecognitionVerdict: String,
}

#[derive(Deserialize)]
struct DeviceIntegrity {
    deviceRecognitionVerdict: Vec<String>,
}

#[derive(Deserialize)]
struct AccountIntegrity {
    appLicensingVerdict: String,
}

pub async fn verify_google_play_integrity(token: &str) -> Result<bool, String> {
    // In a real implementation, we would use the Google Auth library to get an access token
    // for the service account, then call the Play Integrity API.
    // For this implementation, we will mock the call if the token is "mock_android_token".
    
    if token == "mock_android_token" {
        return Ok(true);
    }

    // TODO: Implement actual Google API call using service account credentials.
    // This requires a lot of boilerplate for OAuth2 with Google.
    // For now, we will return an error if it's not the mock token, 
    // prompting the user to implement the full OAuth flow or use a crate like `google-playintegrity1`.
    
    // Placeholder for where the API call would go:
    // let client = reqwest::Client::new();
    // let res = client.post("https://playintegrity.googleapis.com/v1/...")
    //     .json(&{ "integrity_token": token })
    //     .send().await...
    
    Err("Google Play Integrity verification not fully implemented (requires OAuth2). Use 'mock_android_token' for testing.".to_string())
}

// --- Apple DeviceCheck ---

#[derive(Serialize)]
struct AppleJwtClaims {
    iss: String,
    iat: u64,
    exp: u64,
}

#[derive(Serialize)]
struct AppleValidateRequest {
    device_token: String,
    transaction_id: String,
    timestamp: u64,
}

pub async fn verify_apple_check(token: &str) -> Result<bool, String> {
    if token == "mock_ios_token" {
        return Ok(true);
    }

    let key_id = env::var("APPLE_KEY_ID").map_err(|_| "Missing APPLE_KEY_ID")?;
    let team_id = env::var("APPLE_TEAM_ID").map_err(|_| "Missing APPLE_TEAM_ID")?;
    let p8_content = env::var("APPLE_P8_FILE_CONTENT").map_err(|_| "Missing APPLE_P8_FILE_CONTENT")?;

    // Create JWT
    let now = chrono::Utc::now().timestamp() as u64;
    let claims = AppleJwtClaims {
        iss: team_id,
        iat: now,
        exp: now + 3600,
    };

    let header = Header {
        kid: Some(key_id),
        alg: jsonwebtoken::Algorithm::ES256,
        ..Default::default()
    };

    let jwt = encode(
        &header,
        &claims,
        &EncodingKey::from_ec_pem(p8_content.as_bytes()).map_err(|e| format!("Key error: {}", e))?
    ).map_err(|e| format!("JWT error: {}", e))?;

    // Call Apple API
    // Note: Use "https://api.development.devicecheck.apple.com" for testing
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.development.devicecheck.apple.com/v1/validate_device_token")
        .header("Authorization", format!("Bearer {}", jwt))
        .json(&serde_json::json!({
            "device_token": token,
            "transaction_id": uuid::Uuid::new_v4().to_string(),
            "timestamp": now * 1000 // Milliseconds
        }))
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if res.status().is_success() {
        Ok(true)
    } else {
        Err(format!("Apple API failed: {}", res.status()))
    }
}
