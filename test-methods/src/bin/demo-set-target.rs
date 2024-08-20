use simple_kuksa_client::KuksaClient;
use tokio;

const WRONG_SIGNAL: &str = "vehicle.Speed";
const WRONG_VALUE: &str = "100";

const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";

const SENSOR_WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const SENSOR_WIPER_VALUE: &str = "false";

const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.Pos1.Position";
const ACT_POSITION_VALUE: &str = "5";
const ACT_WRONG_POSITION_VALUE: &str = "abc";

async fn test_set_target(vehicle: &mut KuksaClient, signal: &str, value: &str) {
    println!("signal: {} - value: {}", signal, value);

    match vehicle.set_target_value(signal, value).await {
        Ok(_) => {
            println!("Publish done!");
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
    // expected: failed [signal not found]
    // result: failed [signal not found]
    test_set_target(&mut vehicle, WRONG_SIGNAL, WRONG_VALUE).await;

    // status: PASSED
    // expected: failed [signal not found]
    // result: failed [signal not found]
    test_set_target(&mut vehicle, BRANCH_SYSTEM_SIGNAL, WRONG_VALUE).await;

    // status: PASSED
    // expected: failed [not an Actuator] / [signal not found]
    // result: failed [not an Actuator] 
    test_set_target(&mut vehicle, SENSOR_WIPER_SIGNAL, SENSOR_WIPER_VALUE).await;

    // status: PASSED
    // expected: failed [wrong data type]/[parse error]
    // result: failed [wrong data type]/[parse error]
    test_set_target(&mut vehicle, ACT_POSITION_SIGNAL, ACT_WRONG_POSITION_VALUE).await;

    // status: PASSED
    // expected: passed
    // result: passed
    test_set_target(&mut vehicle, ACT_POSITION_SIGNAL, ACT_POSITION_VALUE).await;
}
