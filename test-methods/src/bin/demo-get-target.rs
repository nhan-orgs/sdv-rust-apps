use simple_kuksa_client::{common, KuksaClient};
use tokio;

const WRONG_SIGNAL: &str = "vehicle.Speed";
const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";
const SENSOR_WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.Pos1.Position";

async fn test_get_target(vehicle: &mut KuksaClient, signal: &str) {
    println!("signal: {signal}");

    match vehicle.get_target_value(signal).await {
        Ok(value) => {
            println!("{:?}", common::value_from_option_datapoint(value));
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
    println!();
}

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    // status: PASSED
    // expected: error [signal not found]
    // result: error [signal not found]
    test_get_target(&mut vehicle, WRONG_SIGNAL).await;

    // status: PASSED
    // expected: error [not an Actuator]/[signal not found]
    // result: error [signal not found]
    test_get_target(&mut vehicle, BRANCH_SYSTEM_SIGNAL).await;


    // status: PASSED
    // expected: error [not an Actuator]
    // result: error [not an Actuator]
    test_get_target(&mut vehicle, SENSOR_WIPER_SIGNAL).await;

    // status: PASSED
    // expected: NotAvaiable / Uint
    // result: NotAvaiable / Uint
    test_get_target(&mut vehicle, ACT_POSITION_SIGNAL).await;
}
