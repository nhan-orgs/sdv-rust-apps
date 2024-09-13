use simple_kuksa_client::kuksa_client::SubscribeResponse;
use simple_kuksa_client::{
    common::{self, Value},
    KuksaClient,
};
use std::process::exit;
use tokio;
use tonic::Streaming;

const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";

const HOOD_SIGNAL: &str = "Vehicle.Body.Hood.IsOpen";
const WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.Mode";

fn value_from_message(message: SubscribeResponse) -> Option<Value> {
    for entry_update in message.updates {
        if let Some(entry) = entry_update.entry {
            return common::value_from_datapoint(entry.value);
        }
    }
    return None;
}

async fn get(vehicle: &mut KuksaClient, signal: &str) -> Option<Value> {
    match vehicle.get_current_value(HOOD_SIGNAL).await {
        Ok(data_value) => common::value_from_datapoint(data_value),
        Err(error) => {
            println!("Get value of {} failed", signal);
            println!("Error: {:?}", error);
            exit(-1);
        }
    }
}

async fn set(vehicle: &mut KuksaClient, signal: &str, value: &str) {
    if let Err(error) = vehicle.set_target_value(signal, value).await {
        println!("Set {} = {} failed", signal, value);
        println!("Error: {:?}", error);
        exit(-2);
    };
}

async fn subscribe(vehicle: &mut KuksaClient, signal: &str) -> Streaming<SubscribeResponse> {
    match vehicle.subscribe_current_value(signal).await {
        Ok(response_stream) => response_stream,
        Err(error) => {
            println!("Subscribe {} failed", signal);
            println!("Error: {:?}", error);
            exit(-3);
        }
    }
}

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new(SERVER_ADDRESS);

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    };

    let mut hood_response_stream = subscribe(&mut vehicle, HOOD_SIGNAL).await;

    loop {
        match hood_response_stream.message().await {
            Ok(Some(message)) => {
                let hood_status = value_from_message(message);

                if hood_status == Some(common::Value::Bool(true)) {
                    let wiper_status = get(&mut vehicle, WIPER_SIGNAL).await;

                    if wiper_status == Some(common::Value::Bool(true)) {
                        println!("Hood and Wipers are open !!!");

                        set(&mut vehicle, WIPER_SIGNAL, "OFF").await;
                    }
                }
            }
            Ok(None) => {
                println!("Server gone");
                break;
            }
            Err(error) => {
                println!("Error on hood subscribe stream");
                println!("Error: {:?}", error);
                break;
            }
        }
    }
}
