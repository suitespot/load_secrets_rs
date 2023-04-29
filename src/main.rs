use aws_sdk_secretsmanager::{config::Region, Client};

#[tokio::main]
async fn main() {
    let region = std::env::var("AWS_REGION").ok();
    let config_loader = aws_config::from_env();
    let config_loader = match region {
        Some(region) => config_loader.region(Region::new(region)),
        None => config_loader,
    };
    let aws_config = config_loader.load().await;
    let client = Client::new(&aws_config);

    let secret_id = std::env::var("AWS_SECRET_ID").expect("AWS_SECRET_ID is not set");

    let resp = client.get_secret_value().secret_id(secret_id).send().await.expect("request for secrets failed");

    if resp.secret_binary().is_some() {
        panic!("secret_binary is not supported");
    }
    let secret = resp.secret_string.expect("secret_string is not set");

    let secret_json = serde_json::from_str::<serde_json::Value>(&secret)
        .map(|v| v.as_object().ok_or("not an object").map(|o| o.clone()))
        .unwrap_or_else(|_| Err("secret could not be parsed as json"));
    match secret_json {
        Ok(secret_obj) => {
            for (key, value) in secret_obj {
                println!("export {}={};", key, value);
            }
        }
        Err(_) => {
            println!("{}", secret);
        }
    }
}
