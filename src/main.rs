use aws_sdk_secretsmanager::{Client, config::Region};


#[tokio::main]
async fn main() {
    let region = std::env::var("AWS_REGION").expect("AWS_REGION is not set");
    let aws_config = aws_config::from_env().region(Region::new(region)).load().await;
    let client = Client::new(&aws_config);

    let secret_id = std::env::var("AWS_SECRET_ID").expect("AWS_SECRET_ID is not set");

    let resp = client.get_secret_value()
        .secret_id(secret_id)
        .send()
        .await;

    let resp = resp.unwrap();

    let secret = resp.secret_string.unwrap();
    println!("{:#?}", secret);
}
