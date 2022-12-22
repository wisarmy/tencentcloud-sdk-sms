#[derive(Default, Clone)]
pub struct Credential {
    pub secret_id: String,
    pub secret_key: String,
    pub token: Option<String>,
}

impl Credential {
    pub fn new(
        secret_id: impl Into<String>,
        secret_key: impl Into<String>,
        token: Option<String>,
    ) -> Self {
        Self {
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
            token,
        }
    }
}
