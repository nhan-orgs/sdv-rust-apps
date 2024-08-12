use simple_kuksa_client::KuksaClient;
use tokio;

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    // >>>> [DONE] TEST PUBLISH LEAF ENTRY
    match vehicle
        .publish_entry_data("Vehicle.ADAS.ABS.IsEnabled", "true")
        .await
    {
        Ok(_) => {
            println!("Publish done!");
        }
        Err(error) => {
            println!("Error while publishing entry data: {:?}", error);
        }
    }
}
