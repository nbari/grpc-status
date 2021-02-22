pub mod hello_world {
    tonic::include_proto!("helloworld");
}
use anyhow::Result;
use grpc_status::options;
use hello_world::{greeter_client::GreeterClient, HelloRequest};
use tonic::Code;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = options::new();
    let mut client = GreeterClient::connect(endpoint).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    if let Err(e) = client.say_hello(request).await {
        let x: Code = e.code();
        println!(
            "Number: {}\nCode: {:?}\nDescription: {}",
            x as i32,
            x,
            x.description()
        );
    };
    Ok(())
}
