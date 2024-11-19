/*
Force the wipers off while the hood is open

Idea: Use an infinite loop with a delay duration, turn off the wipers if the hood and the wipers are open at the same time

Methods: `get_current_value`, `set_target_value`

Vehicle.Body.Windshield.Front.Wiping.Mode:
    value set: "OFF", "SLOW", "MEDIUM", "FAST", "INTERVAL", "RAIN_SENSOR"
    example: Some(String("OFF"))

Vehicle.Body.Hood.IsOpen:
    value set: true, false
*/

use simple_kuksa_client::{
    common::{self, Value},
    KuksaClient,
};
use std::{process::exit, thread::sleep, time::Duration};
use tokio;

const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";
const DELAY_TIME: u64 = 3000;

const HOOD_SIGNAL: &str = "Vehicle.Body.Hood.IsOpen"; // VSS 4.0 - actuartor
const WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.Mode"; // VSS 4.0 - actuator

async fn get(vehicle: &mut KuksaClient, signal: &str) -> Option<Value> {
    match vehicle.get_current_value(signal).await {
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

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new(SERVER_ADDRESS);

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    loop {
        let hood_status = get(&mut vehicle, HOOD_SIGNAL).await;

        if hood_status == Some(Value::Bool(true)) {
            println!("----------");
            println!("Hood is opening");

            let wiper_status = get(&mut vehicle, WIPER_SIGNAL).await;
            println!("Wiper: {:?}", wiper_status);

            if wiper_status != Some(Value::String("OFF".to_string())) {
                println!("Wipers are also open!");

                println!("Turning OFF wipers...");
                set(&mut vehicle, WIPER_SIGNAL, "OFF").await;
            }

        }

        sleep(Duration::from_millis(DELAY_TIME));
    }
}