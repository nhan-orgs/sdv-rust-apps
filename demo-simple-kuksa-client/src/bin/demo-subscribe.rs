use simple_kuksa_client::{common, KuksaClient};
use tokio;

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("Connect failed: {:?}", error);
        return;
    }

    // >>>> TEST SUBSCRIBE ENTRIES
    let mut response_stream = match vehicle.subscribe_entry("Vehicle.ADAS").await {
        // how to keep the client (ValClient) alive
        Ok(response_stream) => response_stream,
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
                                    common::value_from_option_datapoint(entry.value)
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
}
