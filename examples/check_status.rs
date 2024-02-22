use rango_sdk::{check::status::StatusRequest, client::Client};

#[tokio::main]
async fn main() {
    let rango = Client::new(
        "uuid-uuid-uuid",
        "4a624ab5-16ff-4f96-90b7-ab00ddfc342c",
        None,
    );

    get_check_status(&rango).await;
}

async fn get_check_status(client: &Client) {
    let request = StatusRequest {
        request_id: "b10e822e-69d6-42ef-a84d-987cce8c6e73".into(),
        tx_id: "0xafa05e208d3ef0e5603776ae8e76e6efcb9f4272f4b11160ea3c240c11efb6a4".into(),
    };
    let result = client.status(request).await.unwrap();
    println!("{:?}", result);
}
