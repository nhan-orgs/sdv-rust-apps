/*
    This program demo how to use method `subscribe_current_value`
        _ VSS v4.0
        _ Kuksa Databroker v3.0

    Method `subscribe_current_value` allows user to subscribe value of a signal (recieves notifications for any changes of these signals)
    Notice that if you subscribe on a branch signal, all it's children will be subscribed.

    This program runs demo on 3 cases:
            1. The signal is not correct
            2. The signal is a BRANCH signal
            3. The signal is an ACTUATOR signal (it also works for a SENSOR signal)

    Because I want to keep the program as simple as possible, I avoid to use multi-thread program here.
    It means there should be only one successful subscription, so I commented the second case (BRACH signal)
    You can comment the third case and uncomment the second case!!!

    When you run the program, each case will print the content follow this format:
    ------
    Signal:
        <signal want to subscribe current value here>
    Expected:
        <expected result is described here>
    Result:
        <result that returned from subscribe_current_value method>
    
*/

use simple_kuksa_client::{common, KuksaClient};
use tokio;

const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";

const WRONG_SIGNAL: &str = "vehicle.Speed";
const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";
const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.DriverSide.Backrest.Lumbar.Height";

async fn test_subscribe_current(vehicle: &mut KuksaClient, signal: &str, expected: &str) {
    println!("------");
    println!("Signal:\n\t{signal}");
    println!("Expected:\n\t{expected}");
    println!("Result:");

    let mut response_stream = match vehicle.subscribe_current_value(signal).await {
        Ok(response_stream) => {
            println!("Subscribe successful! Waiting for signal changes...");
            response_stream
        },
        Err(error) => {
            println!("Subscribe failed: {:?}", error);
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
                                    common::value_from_datapoint(entry.value)
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
    println!("### METHOD: `subscribe_current_value`");

    let expected = "This is a wrong signal. Method should return errors (no entries found for the provided path).";
    test_subscribe_current(&mut vehicle, WRONG_SIGNAL, expected).await;
    
    // let expected = "This is a branch signal. The execution will subscribe all its children!";
    // test_subscribe_current(&mut vehicle, BRANCH_SYSTEM_SIGNAL, expected).await;

    let expected = "This is a correct actuator signal. The execution should be successful.";
    test_subscribe_current(&mut vehicle, ACT_POSITION_SIGNAL, expected).await;
}