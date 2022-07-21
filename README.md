# Tencent Cloud sms rust sdk
version: 2021-01-11
signature: v3

# Examples
```rust
    // prepare param
    let secret_id = "";
    let secret_key = "";
    let phone_number = "+86..";
    let phone_number_set = vec![phone_number.to_owned()];
    let sms_sdk_app_id = "";
    let template_id = "";
    let sign_name = "";
    let template_param_set = vec!["".to_owned()];
    let region = "ap-guangzhou";

    let credential = Credential::new(secret_id.to_owned(), secret_key.to_owned(), None);
    let mut cpf = ClientProfile::default();
    cpf.http_profile.end_point = "sms.tencentcloudapi.com".to_owned();
    //cpf.debug = true;
    let client = Client::new(credential, region.to_owned(), cpf);

    let mut request = SendSmsRequest::default();
    request.params.phone_number_set = phone_number_set;
    request.params.sms_sdk_app_id = sms_sdk_app_id.to_owned();
    request.params.template_id = template_id.to_owned();
    request.params.sign_name = sign_name.to_owned();
    request.params.template_param_set = template_param_set;

    let response = client.send_sms(request).await;
    match response {
        Ok(res) => {
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
```
