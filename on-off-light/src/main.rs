use simple_kuksa_client::KuksaClient;
use tokio;
use std::{thread::sleep, time::Duration};

// const LIGHT_SIGNAL: &str = "Vehicle.Body.Lights.Beam.Low.IsOn"; // 4.0 signal
const LIGHT_SIGNAL: &str = "Vehicle.Body.Lights.IsLowBeamOn"; // 3.0 signal

#[tokio::main]
async fn main() {
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    let mut on = true;

    loop {
        println!("On = {}", on);

        if on {
            match vehicle
                .set_target_value(LIGHT_SIGNAL, "true")
                .await
            {
                Ok(_) => {
                    println!("LIGHT_SIGNAL on!");
                }
                Err(error) => {
                    println!("Error while turning on LIGHT_SIGNAL: {:?}", error);
                }
            }
        } else {
            match vehicle
                .set_target_value(LIGHT_SIGNAL, "false")
                .await
            {
                Ok(_) => {
                    println!("LIGHT_SIGNAL off!");
                }
                Err(error) => {
                    println!("Error while turning off LIGHT_SIGNAL: {:?}", error);
                }
            }
        }

        sleep(Duration::from_millis(1000));
        on = !on;
    }
}
