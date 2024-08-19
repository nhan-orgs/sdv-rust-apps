use simple_kuksa_client::KuksaClient;
use tokio;

const WRONG_SIGNAL: &str = "vehicle.Speed";
const WRONG_VALUE: &str = "100";

const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";

const SENSOR_WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const SENSOR_WIPER_VALUE: &str = "true";

const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.Pos1.Position";
const ACT_POSITION_VALUE: &str = "2";
const ACT_WRONG_POSITION_VALUE: &str = "abc";

async fn test_set_current(vehicle: &mut KuksaClient, signal: &str, value: &str) {
    println!("signal: {} - value: {}", signal, value);

    match vehicle.set_current_value(signal, value).await {
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
    test_set_current(&mut vehicle, WRONG_SIGNAL, WRONG_VALUE).await;

    // status: PASSED
    // expected: error [not an Actuator]/[signal not found]
    // result: error [signal not found]
    test_set_current(&mut vehicle, BRANCH_SYSTEM_SIGNAL, WRONG_VALUE).await;

    // status: PASSED
    // expected: passed
    // result: passed
    test_set_current(&mut vehicle, SENSOR_WIPER_SIGNAL, SENSOR_WIPER_VALUE).await;

    // status: PASSED
    // expected: error [wrong data type]/[parse error]
    // result: error [wrong data type]/[parse error]
    test_set_current(&mut vehicle, ACT_POSITION_SIGNAL, ACT_WRONG_POSITION_VALUE).await;

    // status: PASSED
    // expected: passed
    // result: passed
    test_set_current(&mut vehicle, ACT_POSITION_SIGNAL, ACT_POSITION_VALUE).await;
}
