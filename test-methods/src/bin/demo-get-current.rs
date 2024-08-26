use simple_kuksa_client::{common, KuksaClient};
use tokio;

const WRONG_SIGNAL: &str = "vehicle.Speed";
const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";
const SENSOR_WIPER_SIGNAL: &str  = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.Pos1.Position";

async fn test_get_current(vehicle: &mut KuksaClient, signal: &str) {
    println!("signal: {signal}");

    match vehicle.get_current_value(signal).await {
        Ok(value) => {
            println!("{:?}", common::value_from_datapoint(value));
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
    test_get_current(&mut vehicle, WRONG_SIGNAL).await;
    
    // status: PASSED
    // expected: error [not a sensor/actuator]/[signal not found]
    // result: error [signal not found]
    test_get_current(&mut vehicle, BRANCH_SYSTEM_SIGNAL).await;
    
    // status: PASSED
    // expected: none / bool
    // result: none / bool
    test_get_current(&mut vehicle, SENSOR_WIPER_SIGNAL).await;

    // status: PASSED
    // expected: none / Uint
    // result: none / Uint
    test_get_current(&mut vehicle, ACT_POSITION_SIGNAL).await;
}
