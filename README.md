# Tencent Cloud sms rust sdk
version: 2021-01-11

signature: v3

# Examples
```rust
    // prepare param
    let secret_id = "your secret_id";
    let secret_key = "your secret_key";
    let sms_sdk_app_id = "your sms_sdk_app_id";
    let template_id = "your template_id";
    let sign_name = "your sign_name";
    let region = "ap-guangzhou";
    let phone_number_set = vec!["+86".to_owned()];
    let template_param_set = vec!["123456".to_owned()];
    // build client
    let credential = Credential::new(secret_id, secret_key, None);
    let client = Client::new(credential, region);
    // build request
    let request = SendSmsRequest::new(
        phone_number_set.clone(),
        sms_sdk_app_id,
        template_id,
        sign_name,
        template_param_set,
    );
    // send
    let response = client.send_sms(request).await;
    // check
    match response {
        Ok(res) => {
            phone_number_set.into_iter().for_each(|phone_number| {
                println!(
                    "send {}: {:?}",
                    phone_number,
                    res.check_is_success(phone_number.to_owned())
                );
            });
        }
        Err(e) => {
            println!("send error: {}", e);
        }
    }
```
