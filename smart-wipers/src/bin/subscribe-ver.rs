use simple_kuksa_client::{KuksaClient, common::{self, Value}};
use simple_kuksa_client::kuksa_client::SubscribeResponse;
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

fn value_from_message(message: SubscribeResponse) -> Value {
    for entry_update in message.updates {
        if let Some(entry) = entry_update.entry {
            return  common::value_from_option_datapoint(entry.value);
        }
    }
    Value::String("not found".to_string())
}

async fn turn_off_wipers(vehicle: &mut KuksaClient) {
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

#[tokio::main]
async fn main() {
    println!(">>> DEMO SMART WIPERS (SUBSCRIBE) <<<");

    // connect to kuksa client
    let mut vehicle = KuksaClient::new("http://127.0.0.1:55555");

    if let Err(error) = vehicle.connect().await {
        println!("{:?}", error);
        return;
    }

    // prepare
    println!("___ Prepare...");
    prepare(&mut vehicle).await;

    // subscribe hood
    println!("___ Subscribe hood...");

    let mut response_stream = match vehicle.subscribe_entry(HOOD_SIGNAL).await {
        Ok(response_stream) => response_stream,
        Err(error) => {
            println!("Subscribe failed: {:?}", error);
            return;
        }
    };

    // waiting for hood open
    println!("___ Execute function...");

    loop {
        if let Ok(Some(message)) = response_stream.message().await {
            let hood_status = value_from_message(message);
            
            if hood_status == common::Value::Bool(true) {
                println!("Hood is opening");
    
                turn_off_wipers(&mut vehicle).await;
            }
        } else {
            println!("[Hood subscribe] Something went wrong");
            return;
        }
    }
}
