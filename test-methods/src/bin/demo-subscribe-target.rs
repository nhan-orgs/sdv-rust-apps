use simple_kuksa_client::{common, KuksaClient};
use tokio;

const WRONG_SIGNAL: &str = "vehicle.Speed";
const BRANCH_SYSTEM_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System";
const SENSOR_WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";
const ACT_POSITION_SIGNAL: &str = "Vehicle.Cabin.Seat.Row1.Pos1.Position";

async fn test_subscribe_target(vehicle: &mut KuksaClient, signal: &str) {
    println!("signal: {signal}");

    let mut response_stream = match vehicle.subscribe_target_value(signal).await {
        // how to keep the client (ValClient) alive
        Ok(response_stream) => response_stream,
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
                                    common::value_from_option_datapoint(entry.actuator_target)
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
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("Connect failed: {:?}", error);
        return;
    }

    // status: PASSED
    // expected: error [signal not found]
    // result: error [signal not found]
    test_subscribe_target(&mut vehicle, WRONG_SIGNAL).await;
    
    // status: PASSED
    // expected: error [signal not found] / [not an actuator]
    // result: error [signal not found]
    test_subscribe_target(&mut vehicle, BRANCH_SYSTEM_SIGNAL).await;

    // status: FAILED
    // expected: error [not an actuator]
    // result: passed bool
    test_subscribe_target(&mut vehicle, SENSOR_WIPER_SIGNAL).await;

    // status: PASSED
    // expected: passed
    // result: passed (uint)
    test_subscribe_target(&mut vehicle, ACT_POSITION_SIGNAL).await;
}
