use std::net::SocketAddr;

use volo_grpc::server::{Server, ServiceBuilder};

use volo_example::S;

use volo_example::LogLayer;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    Server::new()
        .add_service(
            ServiceBuilder::new(volo_gen::volo::example::ItemServiceServer::new(S)).build(),
        )
        .run(addr)
        .await
        .unwrap();

    // Server::new()
    // .add_service(ServiceBuilder::new(volo_gen::volo::example::ItemServiceServer::new(S)).build())
    // .layer_front(LogLayer)
    // .run(addr)
    // .await
    // .unwrap();
}
