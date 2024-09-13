use simple_kuksa_client::{
    common::{self, Value},
    KuksaClient,
};
use std::{process::exit, thread::sleep, time::Duration};
use tokio;

const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";
const DELAY_TIME: u64 = 3000;

const HOOD_SIGNAL: &str = "Vehicle.Body.Hood.IsOpen"; // VSS 3.0 - actuartor
const WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.Mode"; // VSS 3.0 - sensor

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
            println!("Hood is opening");

            let wiper_status = get(&mut vehicle, WIPER_SIGNAL).await;

            if wiper_status == Some(Value::Bool(true)) {
                println!("Wipers are also open!");

                set(&mut vehicle, WIPER_SIGNAL, "OFF").await;
            }
        }

        sleep(Duration::from_millis(DELAY_TIME));
    }
}
