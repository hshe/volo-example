use std::net::SocketAddr;
use lazy_static::lazy_static;
use volo_example::LogLayer;


lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "[::1]:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .address(addr)
            .build()
    };

    // static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
    //     let addr: SocketAddr = "[::1]:8080".parse().unwrap();
    //     volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
    //         .layer_outer(LogLayer)
    //         .address(addr)
    //         .build()
    // };
}



// client.rs

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let req = volo_gen::volo::example::GetItemRequest { id: 1024 };
    let resp = CLIENT.get_item(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
}
