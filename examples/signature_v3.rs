use tencentcloud_sdk_sms::{
    client::{Client, SendSmsRequest},
    credentials::Credential,
};
#[tokio::main]
async fn main() {
    // build client
    let credential = Credential::new("your secret_id", "your secret_key", None);
    let client = Client::new(credential, "ap-guangzhou");
    // build request
    let request = SendSmsRequest::new(
        vec!["+86..".to_owned()],
        "your sms_sdk_app_id",
        "your template_id",
        "your sign_name",
        vec!["your template param".to_owned()],
    );
    // send
    let response = client.send_sms(request).await;
    // check
    match response {
        Ok(res) => {
            let phone_number = "+86..";
            println!(
                "send {}: {:?}",
                phone_number,
                //res,
                res.check_is_success(phone_number.to_owned())
            );
        }
        Err(e) => {
            println!("send error: {}", e);
        }
    }
}
