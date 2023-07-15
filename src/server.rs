use tonic::{transport::Server, Request, Response, Status};

use messages::messages_server::{Messages, MessagesServer};
use messages::{MessageReponse, MessageRequest};

pub mod messages {
    tonic::include_proto!("messages");
}

#[derive(Debug, Default)]
pub struct MessagesService {}

#[tonic::async_trait]
impl Messages for MessagesService {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageReponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = MessageReponse {
            success: true,
            message: format!("Sent {} to {}.", req.amount, req.to).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MessagesService::default();

    Server::builder()
        .add_service(MessagesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
