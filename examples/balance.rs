use rango_sdk::{client::Client, request::BalanceRequest};

#[tokio::main]
async fn main() {
    let rango = Client::new(
        "uuid-uuid-uuid",
        "4a624ab5-16ff-4f96-90b7-ab00ddfc342c",
        None,
    );

    get_balance(&rango).await;
}

async fn get_balance(client: &Client) {
    let request = BalanceRequest {
        address: "0x000000000000000000000000000000000000dead".into(),
        blockchain: "BSC".into(),
    };
    let result = client.balance(request).await;
    println!("{:?}", result);
}
