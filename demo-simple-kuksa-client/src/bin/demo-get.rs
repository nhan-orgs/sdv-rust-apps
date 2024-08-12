use simple_kuksa_client::{common, KuksaClient};
use tokio;

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    // >>>> [DONE] TEST GET LEAF ENTRY DATA
    let path = "Vehicle.ADAS.ABS.IsEnabled";
    match vehicle.get_entry_data(path).await {
        Ok(data_value) => {
            let value = common::value_from_option_datapoint(data_value);
            println!("{}: {:?}", path, value);
        }
        Err(error) => {
            println!("Get entries value failed: {:?}", error);
        }
    }
}
