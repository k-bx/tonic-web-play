use ping_server::Ping;
use tonic::{Request, Response, Status};


tonic::include_proto!("ping");

#[derive(Default)]
pub struct PingService {}

#[tonic::async_trait]
impl Ping for PingService {
    async fn ping(&self, request: Request<PingReq>) -> Result<Response<PingRsp>, Status> {
        let addr = request.remote_addr();

        let req = request.into_inner();
        println!("Got a request from: {} {:?}", req.name, addr);

        let reply = PingRsp {
            message: format!("Hello {}!", req.name),
        };

        Ok(Response::new(reply))
    }
}

