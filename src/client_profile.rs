use crate::http_profile::HttpProfile;

#[derive(Debug, Clone)]
pub struct ClientProfile {
    pub http_profile: HttpProfile,
    pub sign_method: String,
    pub language: String,
    pub disable_region_breaker: bool,
    pub debug: bool,
}

impl Default for ClientProfile {
    fn default() -> Self {
        Self {
            http_profile: HttpProfile::default(),
            sign_method: "TC3-HMAC-SHA256".to_owned(),
            language: "zh-CN".to_owned(),
            disable_region_breaker: true,
            debug: false,
        }
    }
}

impl ClientProfile {
    pub fn set_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}
