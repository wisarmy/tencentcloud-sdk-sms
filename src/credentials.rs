#[derive(Default, Clone)]
pub struct Credential {
    pub secret_id: String,
    pub secret_key: String,
    pub token: Option<String>,
}

impl Credential {
    pub fn new(secret_id: String, secret_key: String, token: Option<String>) -> Self {
        Self {
            secret_id,
            secret_key,
            token,
        }
    }
}
