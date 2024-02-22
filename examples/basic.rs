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

    // get_swappers(&rango).await;
    // get_messaging_protocols(&rango).await;
    // get_check_status(&rango).await;
    // get_is_approved(&rango).await;
    // get_balance(&rango).await;
    // get_quote(&rango).await;
    // get_swap(&rango).await;
    get_meta(&rango).await;
}

async fn get_swappers(client: &Client) {
    let result = client.swappers().await;
    println!("{:?}", result);
}

async fn get_messaging_protocols(client: &Client) {
    let result = client.messaging_protocols().await;
    println!("{:?}", result);
}

async fn get_quote(client: &Client) {
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

    let result = client.quote(request).await.unwrap();

    println!("{:?}", result);
}

async fn get_is_approved(client: &Client) {
    let request = IsApprovedRequest {
        request_id: "b10e822e-69d6-42ef-a84d-987cce8c6e73".into(),
        tx_id: Some("0xafa05e208d3ef0e5603776ae8e76e6efcb9f4272f4b11160ea3c240c11efb6a4".into()),
    };
    let result = client.is_approved(request).await.unwrap();
    println!("{:?}", result);
}

async fn get_check_status(client: &Client) {
    let request = StatusRequest {
        request_id: "b10e822e-69d6-42ef-a84d-987cce8c6e73".into(),
        tx_id: "0xafa05e208d3ef0e5603776ae8e76e6efcb9f4272f4b11160ea3c240c11efb6a4".into(),
    };
    let result = client.status(request).await.unwrap();
    println!("{:?}", result);
}

async fn get_balance(client: &Client) {
    let request = BalanceRequest {
        address: "0x000000000000000000000000000000000000dead".into(),
        blockchain: "BSC".into(),
    };
    let result = client.balance(request).await;
    println!("{:?}", result);
}

async fn get_swap(client: &Client) {
    let request = SwapRequest {
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
        disable_estimate: None,
        referrer_address: None,
        referrer_fee: None,
        from_address: "0x000000000000000000000000000000000000dead".into(),
        to_address: "0x000000000000000000000000000000000000dead".into(),
        slippage: "3".into(),
    };
    let result = client.swap(request).await.unwrap();

    println!("{:?}", result);
}

async fn get_meta(client: &Client) {
    let result = client.meta().await;
    println!("{:?}", result);
}
