use rango_sdk::{
    client::Client,
    quote::{Asset, QuoteRequest},
};

#[tokio::main]
async fn main() {
    let rango = Client::new(
        "uuid-uuid-uuid",
        "4a624ab5-16ff-4f96-90b7-ab00ddfc342c",
        None,
    );

    // let result = rango.meta.swappers().await;
    // let result = rango.meta.messaging_protocols().await;

    // println!("{:?}", result);

    let _swappers = vec!["OneInchEth".to_owned()];
    let request = QuoteRequest {
        from: Asset {
            address: None,
            blockchain: "BSC".to_owned(),
            symbol: "BNB".to_owned(),
        },
        to: Asset {
            address: Some("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d".to_owned()),
            blockchain: "BSC".to_owned(),
            symbol: "USDC".to_owned(),
        },
        amount: "2".to_owned(),
        swappers_exclude: None,
        swappers_groups_exclude: None,
        swappers: None,
        messaging_protocols: None,
        swapper_groups: None,
        destination_contract: None,
        im_message: None,
        source_contract: None,
        contract_call: None,
    };

    let result = rango.quote(request).await.unwrap();

    println!("{:?}", result);
}
