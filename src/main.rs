use aws_sdk_secretsmanager::Client;
use std::env;
use setenv::get_shell;

#[tokio::main]
async fn main() {
    let aws_config = aws_config::from_env().load().await;
    let client = Client::new(&aws_config);
    
    let secret_id = env::var("AWS_SECRETS_ID").expect("AWS_SECRETS_ID is not set");
    let resp = client.get_secret_value().secret_id(secret_id).send().await.expect("failed to get secret value");
    
    let secret = match resp.secret_binary() {
        Some(_) => panic!("secret_binary is not supported"),
        None => resp.secret_string.expect("secret_string is not set"),
    };
    
    let secret_json = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&secret);
    match secret_json {
        Ok(secret_obj) => {
            let shell = get_shell();
            for (key, value) in secret_obj {
                let value_str = value.as_str().unwrap_or("");
                shell.setenv(key, value_str);
            }
        }
        Err(_) => {
            println!("{}", secret);
        }
    }
}
