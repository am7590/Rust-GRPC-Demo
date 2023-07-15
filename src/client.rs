use messages::messages_client::MessagesClient;
use messages::MessageRequest;

pub mod messages {
    tonic::include_proto!("messages");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MessagesClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        MessageRequest {
            from: "123456".to_owned(),
            to: "654321".to_owned(),
            amount: 22
        }
    );

    let response = client.send_message(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}