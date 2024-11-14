/*
    This program demo how to use method `get_target_value`
        _ VSS v4.0
        _ Kuksa Databroker v3.0

    Method `get_target_value` allows user to get target value of an actuator.
    Notice that this method is not for a branch or sensor signal.

    Actuators are devices that converts energy into motion.
    The target value represents the goal or command that the actuator should reach.
    Sensors usually read or measure values, so they do not have target value.

    This program runs demo on 4 cases:
            1. The signal is not correct
        2 & 3. The signal is a BRANCH signal
            4. The signal is an ACTUATOR signal

    When you run the program, each case will print the content follow this format:
    ------
    Signal:
        <signal want to get target value here>
    Expected:
        <expected result is described here>
    Result:
        <result that returned from get_target_value method>
    
*/

use simple_kuksa_client::{common, KuksaClient};
use tokio;

const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";

const WRONG_SIGNAL: &str = "vehicle.Speed";
const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";
const SENSOR_WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.DriverSide.Height";

async fn test_get_target(vehicle: &mut KuksaClient, signal: &str, expected: &str) {
    println!("------");
    println!("Signal:\n\t{signal}");
    println!("Expected:\n\t{expected}");

    println!("Result:");
    // this method return a Result<> type
    // we use match statement to catch all possible results
    match vehicle.get_target_value(signal).await {
        Ok(value) => {
            println!("\tValue: {:?}", common::value_from_datapoint(value));
        }
        Err(error) => {
            println!("\tError: {:?}", error);
        }
    }
}

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new(SERVER_ADDRESS);

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    println!("_______________________________");
    println!("### METHOD: `get_target_value`");

    let expected = "This is a wrong signal. Method should return errors (no entries found for the provided path).";
    test_get_target(&mut vehicle, WRONG_SIGNAL, expected).await;

    let expected = "This is a branch signal, which is not allowed to get target value. Method should return errors (path is not an actuator).";
    test_get_target(&mut vehicle, BRANCH_SYSTEM_SIGNAL, expected).await;

    let expected = "This is a sensor signal, which is not allowed to get target value. Method should return errors (path is not an actuator).";
    test_get_target(&mut vehicle, SENSOR_WIPER_SIGNAL, expected).await;

    let expected = "This is a correct actuator signal. Method should return None (if signal value is not initialized) or its value.";
    test_get_target(&mut vehicle, ACT_POSITION_SIGNAL, expected).await;

    println!("_______________________________");
}