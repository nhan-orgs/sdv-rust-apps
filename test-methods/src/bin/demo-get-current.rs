/*
    This program demo how to use method `get_current_value`
        _ VSS v4.0
        _ Kuksa Databroker v3.0

    Method `get_current_value` allows user to get value of a sensor or an actuator.
    Notice that this method is not for a branch signal.

    This program runs demo on 4 cases:
            1. The signal is not correct
            2. The signal is a BRANCH signal
        3 & 4. The signal is a SENSOR/ an ACTUATOR signal

    When you run the program, each case will print the content follow this format:
    ------
    Signal:
        <signal want to get value here>
    Expected:
        <expected result is described here>
    Result:
        <result that returned from get_current_value method>
    
*/

// import to use structs/functions from other libraries
// simple_kuksa_client: interact with Kuksa Databroker
// tokio: for asynchronous functions
use simple_kuksa_client::{common, KuksaClient};
use tokio;

// address of Kuksa Databroker
const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";

// list of signals will be test below
const WRONG_SIGNAL: &str = "vehicle.Speed";
const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";
const SENSOR_WIPER_SIGNAL: &str  = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.DriverSide.Height";

// this function help us to get current value of a signal by using a KuksaClient instance
// this is an async function, it allows you to call .await() methods
// &<T>: immutable reference to type <T>, you can only get, can not set value
// &mut<T>: mutable reference to type <T>, you can get and set value
// for more detail, read Ownership and Borrowing rules
async fn test_get_current(vehicle: &mut KuksaClient, signal: &str, expected: &str) {
    println!("------");
    println!("Signal:\n\t{signal}");
    println!("Expected:\n\t{expected}");

    println!("Result:");
    // `get_current_value` returns a Result<> type
    // we use `match` statement to catch all possible results
    match vehicle.get_current_value(signal).await {
        Ok(value) => {
            println!("\tValue: {:?}", common::value_from_datapoint(value));
        }
        Err(error) => {
            println!("\tError: {:?}", error);
        }
    }
}

// the entry point of this program (start to read the code from here)
// this line set up the async runtime, you don't need to care about it too much :>
#[tokio::main]
async fn main() {
    // create a KuksaClient instance
    let mut vehicle = KuksaClient::new(SERVER_ADDRESS);

    // connect KuksaClient to Kuksa Databroker
    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    println!("_______________________________");
    println!("### METHOD: `get_current_value`");

    let expected = "This is a wrong signal. Method should return errors (no entries found for the provided path).";
    test_get_current(&mut vehicle, WRONG_SIGNAL, expected).await;
    
    let expected = "This is a branch signal, which is not allowed to get current value. Method should return errors (path is not a leaf entry).";
    test_get_current(&mut vehicle, BRANCH_SYSTEM_SIGNAL, expected).await;
    
    let expected = "This is a correct sensor signal. Method should return None (if signal value is not initialized) or its value.";
    test_get_current(&mut vehicle, SENSOR_WIPER_SIGNAL, expected).await;

    let expected = "This is a correct actuator signal. Method should return None (if signal value is not initialized) or its value.";
    test_get_current(&mut vehicle, ACT_POSITION_SIGNAL, expected).await;
    
    println!("_______________________________");
}