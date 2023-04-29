use aws_sdk_secretsmanager::{config::Region, Client};
use std::env;

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
    
    let secret_id = env::var("AWS_SECRET_ID").expect("AWS_SECRET_ID is not set");
    let resp = client.get_secret_value().secret_id(secret_id).send().await.expect("failed to get secret value");
    
    let secret = match resp.secret_binary() {
        Some(_) => panic!("secret_binary is not supported"),
        None => resp.secret_string.expect("secret_string is not set"),
    };
    
    let secret_json = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&secret);
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
