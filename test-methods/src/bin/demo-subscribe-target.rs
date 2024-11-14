/*
    This program demo how to use method `subscribe_target_value`
        _ VSS v4.0
        _ Kuksa Databroker v3.0

    Method `subscribe_target_value` allows user to subscribe target value of an actuator (recieves notifications for any changes of these signals)
    Notice that it is not for a branch/sensor signal.

    This program runs demo on 43 cases:
            1. The signal is not correct
            2. The signal is a BRANCH signal
            3. The signal is an SENSOR signal
            4. The signal is an ACTUATOR signal

    When you run the program, each case will print the content follow this format:
    ------
    Signal:
        <signal want to subscribe target value here>
    Expected:
        <expected result is described here>
    Result:
        <result that returned from subscribe_target_value method>
    
*/

use simple_kuksa_client::{common, KuksaClient};
use tokio;

const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";

const WRONG_SIGNAL: &str = "vehicle.Speed";
const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";
const SENSOR_WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.DriverSide.Backrest.Lumbar.Height";

async fn test_subscribe_target(vehicle: &mut KuksaClient, signal: &str, expected: &str) {
    println!("------");
    println!("Signal:\n\t{signal}");
    println!("Expected:\n\t{expected}");
    println!("Result:");

    let mut response_stream = match vehicle.subscribe_target_value(signal).await {
        // how to keep the client (ValClient) alive
        Ok(response_stream) => {
            println!("Subscribe successful! Waiting for signal changes...");
            response_stream
        },
        Err(error) => {
            println!("Subscribe failed: {:?}", error);
            println!();
            return;
        }
    };

    loop {
        match response_stream.message().await {
            Ok(response) => {
                match response {
                    None => {
                        // The stream was closed by the sender
                        // and no more messages will be delivered
                        println!("[None] Server gone");
                        break;
                    }
                    Some(message) => {
                        // The sender streamed a valid response message val
                        println!("[Message]: ");
                        for entry_update in message.updates {
                            if let Some(entry) = entry_update.entry {
                                println!(
                                    "{:?}: {:?}\n",
                                    entry.path,
                                    common::value_from_datapoint(entry.actuator_target)
                                );
                            }
                        }
                    }
                }
            }
            Err(err) => {
                // a gRPC error was sent by the sender instead of a valid response message.
                // Refer to Status::code and Status::message to examine possible error causes.
                println!("[Error] {:?}", err);
            }
        }
    }

    println!();
}

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new(SERVER_ADDRESS);

    if let Err(error) = vehicle.connect().await {
        println!("Connect failed: {:?}", error);
        return;
    }

    println!("_______________________________");
    println!("### METHOD: `subscribe_target_value`");

    let expected = "This is a wrong signal. Method should return errors (no entries found for the provided path).";
    test_subscribe_target(&mut vehicle, WRONG_SIGNAL, expected).await;
    
    let expected = "This is a branch signal, which is not allowed to set target value. Method should return errors (path is not an actuator).";
    test_subscribe_target(&mut vehicle, BRANCH_SYSTEM_SIGNAL, expected).await;

    let expected = "This is a sensor signal, which is not allowed to set target value. Method should return errors (path is not an actuator).";
    test_subscribe_target(&mut vehicle, SENSOR_WIPER_SIGNAL, expected).await;

    let expected = "This is an actuator signal. The execution should be successful.";
    test_subscribe_target(&mut vehicle, ACT_POSITION_SIGNAL, expected).await;
}