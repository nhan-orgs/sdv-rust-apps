/*
Force the wipers off while the hood is open

Idea: 
+ Subscribe both the hood and the wipers;
+ Whenever the hood or the wipers are updated, 
turn off the wipers if the hood and the wipers 
are currently open

Methods: `get_current_value`, `set_target_value`, subscribe_current_value`

Vehicle.Body.Windshield.Front.Wiping.Mode:
    value set: "OFF", "SLOW", "MEDIUM", "FAST", "INTERVAL", "RAIN_SENSOR"
    example: Some(String("OFF"))

Vehicle.Body.Hood.IsOpen:
    value set: true, false
*/

use simple_kuksa_client::kuksa_client::SubscribeResponse;
use simple_kuksa_client::{
    common::{self, Value},
    KuksaClient,
};
use std::process::exit;
use std::sync::Arc;
use tokio::{self, sync::Mutex};
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

async fn get(vehicle: &Arc<Mutex<KuksaClient>>, signal: &str) -> Option<Value> {
    match vehicle.lock().await.get_current_value(signal).await {
        Ok(data_value) => common::value_from_datapoint(data_value),
        Err(error) => {
            println!("Get value of {} failed", signal);
            println!("Error: {:?}", error);
            exit(-1);
        }
    }
}

async fn set(vehicle: &Arc<Mutex<KuksaClient>>, signal: &str, value: &str) {
    if let Err(error) = vehicle.lock().await.set_target_value(signal, value).await {
        println!("Set {} = {} failed", signal, value);
        println!("Error: {:?}", error);
        exit(-2);
    };
}

async fn subscribe(
    vehicle: &Arc<Mutex<KuksaClient>>,
    signal: &str,
) -> Streaming<SubscribeResponse> {
    match vehicle.lock().await.subscribe_current_value(signal).await {
        Ok(response_stream) => response_stream,
        Err(error) => {
            println!("Subscribe {} failed", signal);
            println!("Error: {:?}", error);
            exit(-3);
        }
    }
}

async fn turn_off_wipers(vehicle: &Arc<Mutex<KuksaClient>>) {
    let wiper_status = get(vehicle, WIPER_SIGNAL).await;

    if wiper_status != Some(common::Value::String("OFF".to_string())) {
        set(vehicle, WIPER_SIGNAL, "OFF").await;
    }
}

async fn check_hood_open(vehicle: &Arc<Mutex<KuksaClient>>) {
    let hood_status = get(vehicle, HOOD_SIGNAL).await;

    if hood_status == Some(Value::Bool(true)) {
        set(vehicle, WIPER_SIGNAL, "OFF").await;
    }
}

async fn manage_hood_subscribe(vehicle: Arc<Mutex<KuksaClient>>) {
    let mut hood_response_stream = subscribe(&vehicle, HOOD_SIGNAL).await;

    loop {
        match hood_response_stream.message().await {
            Ok(Some(message)) => {
                let hood_status = value_from_message(message);

                if hood_status == Some(common::Value::Bool(true)) {
                    turn_off_wipers(&vehicle).await;
                }
            }
            Ok(None) => {
                println!("[Hood] Server gone");
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

async fn manage_wipers_subscribe(vehicle: Arc<Mutex<KuksaClient>>) {
    let mut wipers_response_stream = subscribe(&vehicle, WIPER_SIGNAL).await;

    loop {
        match wipers_response_stream.message().await {
            Ok(Some(message)) => {
                let wipers_status = value_from_message(message);

                if wipers_status == Some(common::Value::Bool(true)) {
                    check_hood_open(&vehicle).await;
                }
            }
            Ok(None) => {
                println!("[Wiper] Server gone");
                break;
            }
            Err(error) => {
                println!("Error on wiper subscribe stream");
                println!("Error: {:?}", error);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let vehicle = Arc::new(Mutex::new(KuksaClient::new(SERVER_ADDRESS)));

    if let Err(error) = vehicle.lock().await.connect().await {
        println!("{:?}", error);
        return;
    };

    let hood_vehicle = Arc::clone(&vehicle);
    let wipers_vehicle = Arc::clone(&vehicle);

    let hood_handler = tokio::spawn(async move { manage_hood_subscribe(hood_vehicle).await });
    let wipers_handler = tokio::spawn(async move { manage_wipers_subscribe(wipers_vehicle).await });

    let _ = tokio::join!(hood_handler, wipers_handler);
}
