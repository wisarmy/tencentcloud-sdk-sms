#[derive(Debug, Clone)]
pub struct HttpProfile {
    pub req_method: String,
    pub req_timeout: u64,
    pub scheme: String,
    pub root_domain: String,
    pub end_point: String,
}

impl Default for HttpProfile {
    fn default() -> Self {
        Self {
            req_method: "POST".to_owned(),
            req_timeout: 30,
            scheme: "HTTPS".to_owned(),
            root_domain: "".to_owned(),
            end_point: "".to_owned(),
        }
    }
}
