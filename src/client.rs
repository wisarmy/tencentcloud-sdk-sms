use crate::{client_profile::ClientProfile, credentials::Credential, http_profile::HttpProfile};
use chrono::{DateTime, Local};
use crypto_hash::{hex_digest, Algorithm};
use hmac::{digest::CtOutput, Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::Duration;

const API_VERSION: &str = "2021-01-11";
const ROOT_DOMAIN: &str = "tencentcloudapi.com";

#[derive(Default, Clone)]
pub struct Client {
    region: String,
    http_client: reqwest::Client,
    http_profile: HttpProfile,
    profile: ClientProfile,
    credential: Credential,
    sign_method: String,
    debug: bool,
}

impl Client {
    pub fn new(credential: Credential, region: impl Into<String>) -> Self {
        let client_profile = ClientProfile::default();
        Self {
            region: region.into(),
            http_client: reqwest::Client::default(),
            profile: client_profile.clone(),
            http_profile: client_profile.http_profile,
            credential,
            sign_method: "TC3-HMAC-SHA256".to_owned(),
            debug: client_profile.debug,
            ..Default::default()
        }
    }
    pub fn new_with_client_profile(
        credential: Credential,
        region: String,
        client_profile: ClientProfile,
    ) -> Self {
        Self {
            region,
            http_client: reqwest::Client::default(),
            profile: client_profile.clone(),
            http_profile: client_profile.http_profile,
            credential,
            sign_method: "TC3-HMAC-SHA256".to_owned(),
            debug: client_profile.debug,
            ..Default::default()
        }
    }

    pub fn get_content_type(&self) -> String {
        if self.http_profile.req_method == "Get" {
            "application/x-www-form-urlencoded".to_owned()
        } else {
            "application/json".to_owned()
        }
    }

    pub async fn send_sms(
        self,
        request: SendSmsRequest,
    ) -> Result<SendSmsResponse, reqwest::Error> {
        let url = format!(
            "{}://{}{}",
            self.http_profile.scheme, request.domain, request.path,
        );
        let now = Local::now();

        let response = self
            .http_client
            .post(url)
            .header("X-TC-Action", request.action.clone())
            .header("X-TC-Region", self.region.clone())
            .header("X-TC-Timestamp", now.timestamp().to_string())
            .header("X-TC-Version", request.version.clone())
            .header("X-TC-Language", self.profile.language.clone())
            .header("Content-Type", self.get_content_type())
            .header("Host", request.domain.clone())
            .header("Authorization", self.authorization(&request, now))
            .timeout(Duration::from_secs(self.http_profile.req_timeout))
            .json(&request.params)
            .send()
            .await?;

        match response.json().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e)?,
        }
    }

    fn authorization(&self, request: &SendSmsRequest, now: DateTime<Local>) -> String {
        let canonical_headers = format!(
            "content-type:{}\nhost:{}\n",
            self.get_content_type(),
            request.domain
        );
        let signed_headers = "content-type;host";
        let mut hashed_request_payload = "".to_string();
        if self.http_profile.req_method == "POST" {
            let payload = serde_json::to_string(&request.params).unwrap();
            hashed_request_payload = hex_digest(Algorithm::SHA256, payload.as_bytes());
        }
        // canonical_request
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.http_profile.req_method,
            "/",
            "",
            canonical_headers,
            signed_headers,
            hashed_request_payload
        );
        if self.debug {
            println!("canonical_request: {}", canonical_request);
        }
        let hashed_canonical_request = hex_digest(Algorithm::SHA256, canonical_request.as_bytes());

        // string2sign
        let date = now.naive_utc().date().to_string();
        let credential_scope = format!("{}/{}/tc3_request", date, request.service);
        let string2sign = format!(
            "{}\n{}\n{}\n{}",
            self.profile.sign_method,
            now.timestamp(),
            credential_scope,
            hashed_canonical_request
        );
        if self.debug {
            println!("string2sign: {}", string2sign);
        }
        // sign string
        let secret_date = self.hmacsha256(
            date.as_bytes(),
            format!("TC3{}", self.credential.secret_key).as_bytes(),
        );
        let secret_service =
            self.hmacsha256(request.service.as_bytes(), &*secret_date.into_bytes());
        let secret_key = self.hmacsha256("tc3_request".as_bytes(), &*secret_service.into_bytes());
        let signature = hex::encode(
            self.hmacsha256(string2sign.as_bytes(), &*secret_key.into_bytes())
                .into_bytes(),
        );

        let authorization = format!(
            "{} Credential={}/{}, SignedHeaders=content-type;host, Signature={}",
            self.sign_method, self.credential.secret_id, credential_scope, signature,
        );
        if self.debug {
            println!("authorization: {}", authorization);
        }
        authorization
    }
    fn hmacsha256(&self, content: &[u8], key: &[u8]) -> CtOutput<Hmac<Sha256>> {
        let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
        mac.update(content);
        let result = mac.finalize();

        result
    }
}
#[derive(Serialize, Clone)]
pub struct SendSmsRequest {
    pub service: String,
    pub version: String,
    pub action: String,
    pub domain: String,
    pub path: String,
    pub params: SmsRequestParams,
}

impl Default for SendSmsRequest {
    fn default() -> Self {
        Self {
            service: "sms".to_owned(),
            version: API_VERSION.to_owned(),
            action: "SendSms".to_owned(),
            domain: format!("{}.{}", "sms", ROOT_DOMAIN),
            path: "/".to_owned(),
            params: SmsRequestParams::default(),
        }
    }
}
impl SendSmsRequest {
    pub fn new(
        phone_number_set: Vec<String>,
        sms_sdk_app_id: impl Into<String>,
        template_id: impl Into<String>,
        sign_name: impl Into<String>,
        template_param_set: Vec<String>,
    ) -> Self {
        Self {
            service: "sms".to_owned(),
            version: API_VERSION.to_owned(),
            action: "SendSms".to_owned(),
            domain: format!("{}.{}", "sms", ROOT_DOMAIN),
            path: "/".to_owned(),
            params: SmsRequestParams {
                phone_number_set,
                sms_sdk_app_id: sms_sdk_app_id.into(),
                template_id: template_id.into(),
                sign_name: sign_name.into(),
                template_param_set,
            },
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SmsRequestParams {
    #[serde(rename(serialize = "PhoneNumberSet"))]
    pub phone_number_set: Vec<String>,
    #[serde(rename(serialize = "SmsSdkAppId"))]
    pub sms_sdk_app_id: String,
    #[serde(rename(serialize = "TemplateId"))]
    pub template_id: String,
    #[serde(rename(serialize = "SignName"))]
    pub sign_name: String,
    #[serde(rename(serialize = "TemplateParamSet"))]
    pub template_param_set: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendSmsResponse {
    #[serde(rename = "Response")]
    pub response: Response,
}

impl SendSmsResponse {
    pub fn check_is_success(&self, phone_number: String) -> bool {
        if let Some(send_status_set) = self.response.send_status_set.clone() {
            for send_status in send_status_set.into_iter() {
                if send_status.phone_number == phone_number {
                    return send_status.code == String::from("Ok");
                }
            }
        }
        false
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "SendStatusSet")]
    pub send_status_set: Option<Vec<SendStatusSet>>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
    #[serde(rename = "Error")]
    pub error: Option<Error>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendStatusSet {
    #[serde(rename = "SerialNo")]
    pub serial_no: String,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,
    #[serde(rename = "Fee")]
    pub fee: i64,
    #[serde(rename = "SessionContext")]
    pub session_context: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "IsoCode")]
    pub iso_code: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
}
