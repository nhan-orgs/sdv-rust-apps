use simple_kuksa_client::{KuksaClient, common};
use std::{thread::sleep, time::Duration};
use tokio;

const HOOD_SIGNAL: &str = "Vehicle.Body.Hood.IsOpen";
const WIPER_SIGNAL: &str = "Vehicle.Body.Windshield.Front.Wiping.System.IsWiping";

async fn prepare(vehicle: &mut KuksaClient) {
    // turn on the wiper
    match vehicle
        .publish_entry_data(WIPER_SIGNAL, "true")
        .await
    {
        Ok(_) => {
            println!("Turn on wipers!");
        }
        Err(error) => {
            println!("Error while turning on the wipers {:?}", error);
        }
    }

    // turn off the hood
    match vehicle
        .publish_entry_data(HOOD_SIGNAL, "false")
        .await
    {
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
    println!(">>> DEMO SMART WIPERS <<<");

    // connect to kuksa client
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    // prepare
    println!("___ Prepare...");
    prepare(&mut vehicle).await;

    // waiting for hood open
    println!("___ Execute function...");

    loop {
        let hood_cur_status = match vehicle.get_entry_data(HOOD_SIGNAL).await {
            Ok(data_value) => {
                common::value_from_option_datapoint(data_value)
            }
            Err(error) => {
                println!("Get hood status failed: {:?}", error);
                return;
            }
        };
        
        if hood_cur_status == common::Value::Bool(true) {
            println!("Hood is opening");

            let wiper_status = match vehicle.get_entry_data(WIPER_SIGNAL).await {
                Ok(data_value) => {
                    common::value_from_option_datapoint(data_value)
                }
                Err(error) => {
                    println!("Get wipers status failed: {:?}", error);
                    return;
                }
            };

            if wiper_status == common::Value::Bool(true) {
                println!("Wipers are also open!");

                match vehicle
                    .publish_entry_data(WIPER_SIGNAL, "false")
                    .await
                {
                    Ok(_) => {
                        println!("Turn off wipers!\n");
                    }
                    Err(error) => {
                        println!("Error while turning off the wipers {:?}", error);
                        return;
                    }
                }
            }
        }

        sleep(Duration::from_millis(3000));
        println!(".");
    }
}