use rango_sdk::client::Client;

#[tokio::main]
async fn main() {
    let rango = Client::new(
        "uuid-uuid-uuid",
        "4a624ab5-16ff-4f96-90b7-ab00ddfc342c",
        None,
    );

    get_meta(&rango).await;
}

async fn get_meta(client: &Client) {
    let result = client.meta().await;
    println!("{:?}", result);
}
