/*
    This program demo how to use method `set_current_value`
        _ VSS v4.0
        _ Kuksa Databroker v3.0

    Method `set_current_value` allows user to set value of a sensor or an actuator.
    Notice that this method is not for a branch signal.

    This program runs demo on 5 cases:
            1. The signal is not correct
            2. The signal is a BRANCH signal
            3. The signal is an ACTUATOR signal, but data type is wrong 
            4. The signal is a SENSOR signal
            5. The signal is an ACTUATOR signal

    When you run the program, each case will print the content follow this format:
    ------
    Signal:
        <signal want to set current value here>
    Expected:
        <expected result is described here>
    Result:
        <result that returned from set_current_value method>
    
*/

use simple_kuksa_client::KuksaClient;
use tokio;

const WRONG_SIGNAL: &str = "vehicle.Speed";
const WRONG_VALUE: &str = "100";

const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";

const SENSOR_WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const SENSOR_WIPER_VALUE: &str = "true";

const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.DriverSide.Backrest.Lumbar.Height";
const ACT_POSITION_VALUE: &str = "2";
const ACT_WRONG_POSITION_VALUE: &str = "abc";

async fn test_set_current(vehicle: &mut KuksaClient, signal: &str, value: &str, expected: &str) {
    println!("------");
    println!("Signal:\n\t{signal}");
    println!("Value:\n\t{value}");
    println!("Expected:\n\t{expected}");

    println!("Result:");
    match vehicle.set_current_value(signal, value).await {
        Ok(_) => {
            println!("\tPublish done!");
        }
        Err(error) => {
            println!("\tError: {:?}", error);
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

    println!("_______________________________");
    println!("### METHOD: `set_current_value`");

    let expected = "This is a wrong signal. Method should return errors (no entries found for the provided path).";
    test_set_current(&mut vehicle, WRONG_SIGNAL, WRONG_VALUE, expected).await;

    let expected = "This is a branch signal, which is not allowed to set current value. Method should return errors (path maybe not a leaf entry).";
    test_set_current(&mut vehicle, BRANCH_SYSTEM_SIGNAL, WRONG_VALUE, expected).await;

    let expected = "This is an actuator signal with WRONG datatype. Method should return errors (parse error).";
    test_set_current(&mut vehicle, ACT_POSITION_SIGNAL, ACT_WRONG_POSITION_VALUE, expected).await;

    let expected = "This is a sensor signal with correct datatype. The execution should be successful.";
    test_set_current(&mut vehicle, SENSOR_WIPER_SIGNAL, SENSOR_WIPER_VALUE, expected).await;

    let expected = "This is an actuator signal with correct datatype. The execution should be successful.";
    test_set_current(&mut vehicle, ACT_POSITION_SIGNAL, ACT_POSITION_VALUE, expected).await;
}