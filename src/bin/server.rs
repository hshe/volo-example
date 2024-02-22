use std::net::SocketAddr;

use volo_grpc::server::{Server, ServiceBuilder};
use volo_http::server::Server as HttpServer;

use volo_example::S;
use volo_http::Router;
use volo_example::LogLayer;
// use volo_http::{
//     server::{route::{from_handler, from_service, get, post, service_fn, MethodRouter, Router}},
//     Address,
// };
use volo_http::route::{get};

async fn hello() -> &'static str {
    "hello, world\n"
}

fn index_router() -> Router {
    // curl http://127.0.0.1:8080/
    Router::new().route("/", get(hello))
}

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    let http_addr: SocketAddr = "[::]:8081".parse().unwrap();
    let http_addr = volo::net::Address::from(http_addr);

    let server = Server::new()
        .add_service(
            ServiceBuilder::new(volo_gen::volo::example::ItemServiceServer::new(S)).build(),
        )
        .run(addr);

    let app = Router::new()
    .merge(index_router());

    let http_server = HttpServer::new(app)
        .run(http_addr);

    let (_result1, _result2) = tokio::join!(server, http_server);

    // Server::new()
    // .add_service(ServiceBuilder::new(volo_gen::volo::example::ItemServiceServer::new(S)).build())
    // .layer_front(LogLayer)
    // .run(addr)
    // .await
    // .unwrap();
}