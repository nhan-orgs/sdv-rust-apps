use simple_kuksa_client::kuksa_client::SubscribeResponse;
use simple_kuksa_client::{
    common::{self, Value},
    KuksaClient,
};
use tokio;

const HOOD_SIGNAL: &str = "Vehicle.Body.Hood.IsOpen";
const WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";

fn value_from_message(message: SubscribeResponse) -> Value {
    for entry_update in message.updates {
        if let Some(entry) = entry_update.entry {
            return common::value_from_option_datapoint(entry.value);
        }
    }
    Value::String("not found".to_string())
}

async fn prepare(vehicle: &mut KuksaClient) {
    // turn on the wipers
    match vehicle.set_target_value(WIPER_SIGNAL, "true").await {
        Ok(_) => {
            println!("Turn on wipers!");
        }
        Err(error) => {
            println!("Error while turning on the wipers {:?}", error);
        }
    }

    // turn off the hood
    match vehicle.set_target_value(HOOD_SIGNAL, "false").await {
        Ok(_) => {
            println!("Turn off hood!");
        }
        Err(error) => {
            println!("Error while turning off the hood {:?}", error);
        }
    }
}

#[tokio::main]
async fn main() {
    println!(">>> DEMO SMART WIPERS (SUBSCRIBE) <<<");

    // connect to kuksa client
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    };

    // prepare
    // println!("___ Prepare...");
    // prepare(&mut vehicle).await;

    println!("# Subscribe hood...");
    let mut hood_response_stream = match vehicle.subscribe_current_value(HOOD_SIGNAL).await {
        Ok(hood_response_stream) => hood_response_stream,
        Err(error) => {
            println!("Subscribe hood failed: {:?}", error);
            return;
        }
    };

    println!("# Waiting for hook event...");
    loop {
        match hood_response_stream.message().await {
            Ok(Some(message)) => {
                let hood_status = value_from_message(message);

                if hood_status == common::Value::Bool(true) {
                    let wiper_status = match vehicle.get_current_value(WIPER_SIGNAL).await {
                        Ok(data_value) => common::value_from_option_datapoint(data_value),
                        Err(error) => {
                            println!("Get wipers status failed: {:?}", error);
                            return;
                        }
                    };

                    if wiper_status == common::Value::Bool(true) {
                        println!("[Hood manager] Hood and Wipers are open !!!");

                        match vehicle.set_target_value(WIPER_SIGNAL, "false").await {
                            Ok(_) => {
                                println!("[Hood manager] Turn off wipers!\n");
                            }
                            Err(error) => {
                                println!(
                                    "[Hood manager] Error while turning off the wipers {:?}",
                                    error
                                );
                                return;
                            }
                        }
                    }
                }
            }
            Ok(None) => {
                println!("[Hood manager] Server gone");
                break;
            }
            Err(error) => {
                println!("[Hood manager] Error: {:?}", error);
                break;
            }
        }
    }
}
