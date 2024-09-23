use simple_kuksa_client::KuksaClient;
use std::{thread::sleep, time::Duration};
use tokio;

const SERVER_ADDRESS: &str = "http://127.0.0.1:55555";
const DELAY_TIME: u64 = 1000;

// const LIGHT_SIGNAL: &str = "Vehicle.Body.Lights.Beam.Low.IsOn"; // 4.0 signal
const LIGHT_SIGNAL: &str = "Vehicle.Body.Lights.IsLowBeamOn"; // 3.0 signal

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new(SERVER_ADDRESS);

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    let mut light_value = true;

    loop {
        light_value = !light_value;

        let str_light_value: &str = if light_value { "true" } else { "false" };
        println!("{}", str_light_value);

        let _ = vehicle
            .set_target_value(LIGHT_SIGNAL, str_light_value)
            .await;

        sleep(Duration::from_millis(DELAY_TIME));
    }
}
