use tonic::transport::Server;

pub mod ping;

tonic::include_proto!("ping");

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8301".parse().unwrap();
    // let cert = std::fs::read_to_string(
    //     PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "deploy"])
    //         .join("tonic-web-play.localhost.crt"),
    // )
    // .unwrap();
    // let key = std::fs::read_to_string(
    //     PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "deploy"])
    //         .join("tonic-web-play.localhost.key"),
    // )
    // .unwrap();
    // let identity = Identity::from_pem(cert, key);
    println!("> Running on tonic-web-play.localhost:8301");
    Server::builder()
        .accept_http1(true)
        .layer(tonic_web::GrpcWebLayer::new())
        // .tls_config(tonic::transport::ServerTlsConfig::new().identity(identity)).unwrap()
        .add_service(crate::ping::ping_server::PingServer::new(
            crate::ping::PingService::default(),
        ))
        .serve(addr)
        .await
        .unwrap();
}
