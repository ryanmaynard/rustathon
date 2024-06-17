use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub stripe_secret_key: String,
    pub twilio_account_sid: String,
    pub twilio_auth_token: String,
    pub openai_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            google_client_id: env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set"),
            stripe_secret_key: env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set"),
            twilio_account_sid: env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID must be set"),
            twilio_auth_token: env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN must be set"),
            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
        }
    }
}

