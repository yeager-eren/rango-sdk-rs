use rango_sdk::client::Client;

#[tokio::main]
async fn main() {
    let rango = Client::new(
        "uuid-uuid-uuid",
        "4a624ab5-16ff-4f96-90b7-ab00ddfc342c",
        None,
    );
    // let result = rango.meta.chains().await;
    let result = rango.meta.messaging_protocols().await;

    println!("{:?}", result)
}
