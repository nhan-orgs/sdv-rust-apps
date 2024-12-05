/*
This application will switch the status of light (on/off) after every 1 second

Method: `set_target_value`

Ignore the error of `set_target_value` because this is the simplest version code - only use for understanding the idea of the application and Rust syntax for beginner.
*/

use simple_kuksa_client::KuksaClient;
use std::{thread::sleep, time::Duration};
use tokio;

const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";
const DELAY_TIME: u64 = 1000;

const LIGHT_SIGNAL: &str = "Vehicle.Body.Lights.Beam.Low.IsOn"; // 4.0 signal

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new(SERVER_ADDRESS);

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    // by default, you can not change value of a variable
    // in this case, `light_value` will be changed each 1s
    // so we add `mut` to mutate it
    let mut light_value = true;

    // this is an infinite loop...
    loop {
        light_value = !light_value;

        // `if ... else ...` will returns a &str value (`return` keyword is ignored)
        // that value will be set to `str_light_value`
        let str_light_value: &str = if light_value { "true" } else { "false" };

        // `set_target_value` returns Result<> type
        // however, we do not manage the error case
        // _ will skip the return value 
        let _ = vehicle
            .set_target_value(LIGHT_SIGNAL, str_light_value)
            .await;

        sleep(Duration::from_millis(DELAY_TIME));
    }
}