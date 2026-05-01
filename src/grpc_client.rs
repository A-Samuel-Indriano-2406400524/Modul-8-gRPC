pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient,
    PaymentRequest, TransactionRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_payment().await?;
    test_transaction().await?;

    Ok(())
}

async fn test_payment() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let response = client.process_payment(request).await?;

    println!("Payment response: {:?}", response.into_inner());

    Ok(())
}

async fn test_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TransactionServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut stream = client.get_transaction_history(request).await?.into_inner();

    while let Some(transaction) = stream.message().await? {
        println!("Transaction: {:?}", transaction);
    }

    Ok(())
}