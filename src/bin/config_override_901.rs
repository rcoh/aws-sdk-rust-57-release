// https://github.com/awslabs/aws-sdk-rust/issues/901
use aws_sdk_kms::config::{Credentials, Region};
use aws_sdk_kms::Client;
use aws_sdk_kms::primitives::Blob;

fn creds() -> Credentials {
    Credentials::new(
        "asdf",
        "secret",
        Some("session-token".to_string()),
        None,
        "test",
    )
}

#[tokio::main]
async fn main(){

    //Code to encrypt text (this works fine)
    tracing_subscriber::fmt::init();

    // Create a client with no credentials
    let conf = aws_sdk_kms::Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(conf);

    // Create a configuration override to be used on decrypt call
    let override_config = aws_sdk_kms::Config::builder()
        .credentials_provider(creds());

    // Decrypt call returns DispatchFailure when it should return the decrypted result
    let _result = client
        .decrypt()
        .ciphertext_blob(Blob::new("blob"))
        .customize()
        .config_override(override_config)
        .send()
        .await
        .unwrap();
    // This should fail with an error about invalid credentials, but instead the request is not sent!
}