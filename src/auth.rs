use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordAccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTokenRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTokenResponse {
    pub token: DiscordAccessToken,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenValidRequest {
    pub token: DiscordAccessToken,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenValidResponse {
    pub refreshed_token: Option<DiscordAccessToken>,
}
