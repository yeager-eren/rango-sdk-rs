use rango_sdk::{
    check::{balance::BalanceRequest, is_approved::IsApprovedRequest, status::StatusRequest},
    client::Client,
    quote::{Asset, QuoteRequest},
    swap::SwapRequest,
};

#[tokio::main]
async fn main() {
    let rango = Client::new(
        "uuid-uuid-uuid",
        "4a624ab5-16ff-4f96-90b7-ab00ddfc342c",
        None,
    );

    get_swappers(&rango).await;
}

async fn get_swappers(client: &Client) {
    let result = client.swappers().await;
    println!("{:?}", result);
}
