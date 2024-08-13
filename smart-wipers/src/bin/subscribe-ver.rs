use simple_kuksa_client::kuksa_client::SubscribeResponse;
use simple_kuksa_client::{
    common::{self, Value},
    KuksaClient,
};
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

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

async fn prepare(vehicle: &Arc<Mutex<KuksaClient>>) {
    // turn on the wiper
    match vehicle
        .lock()
        .await
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
        .lock()
        .await
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

async fn turn_off_wipers(vehicle: &Arc<Mutex<KuksaClient>>) {
    let wiper_status = match vehicle.lock().await.get_entry_data(WIPER_SIGNAL).await {
        Ok(data_value) => common::value_from_option_datapoint(data_value),
        Err(error) => {
            println!("Get wipers status failed: {:?}", error);
            return;
        }
    };

    if wiper_status == common::Value::Bool(true) {
        println!("[Hood manager] Hood and Wipers are open !!!");

        match vehicle
            .lock()
            .await
            .publish_entry_data(WIPER_SIGNAL, "false")
            .await
        {
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

async fn check_hood_open(vehicle: &Arc<Mutex<KuksaClient>>) {
    let hood_status = match vehicle.lock().await.get_entry_data(HOOD_SIGNAL).await {
        Ok(data_value) => common::value_from_option_datapoint(data_value),
        Err(error) => {
            println!("Get hood status failed: {:?}", error);
            return;
        }
    };

    if hood_status == Value::Bool(true) {
        println!("[Wipers manager] Hood and Wipers are open !!!");

        match vehicle
            .lock()
            .await
            .publish_entry_data(WIPER_SIGNAL, "false")
            .await
        {
            Ok(_) => {
                println!("[Wipers manager] Turn off wipers!\n");
            }
            Err(error) => {
                println!(
                    "[Wipers manager] Error while turning off the wipers {:?}",
                    error
                );
                return;
            }
        }
    }
}

async fn manage_hood_subscribe(vehicle: Arc<Mutex<KuksaClient>>) {
    // subscribe hood
    println!("# Subscribe hood...");

    let mut hood_response_stream = match vehicle.lock().await.subscribe_entry(HOOD_SIGNAL).await {
        Ok(hood_response_stream) => hood_response_stream,
        Err(error) => {
            println!("Subscribe hood failed: {:?}", error);
            return;
        }
    };

    loop {
        // hood events
        if let Ok(Some(message)) = hood_response_stream.message().await {
            let hood_status = value_from_message(message);

            if hood_status == common::Value::Bool(true) {
                turn_off_wipers(&vehicle).await;
            }
        } else {
            println!("[Hood manager] Something went wrong");
            return;
        }
    }
}

async fn manage_wipers_subscribe(vehicle: Arc<Mutex<KuksaClient>>) {
    // subscribe wiper
    println!("# Subscribe wipers...");

    let mut wipers_response_stream = match vehicle.lock().await.subscribe_entry(WIPER_SIGNAL).await
    {
        Ok(wipers_response_stream) => wipers_response_stream,
        Err(error) => {
            println!("Subscribe wipers failed: {:?}", error);
            return;
        }
    };

    loop {
        // wipers events
        if let Ok(Some(message)) = wipers_response_stream.message().await {
            let wipers_status = value_from_message(message);

            if wipers_status == common::Value::Bool(true) {
                check_hood_open(&vehicle).await;
            }
        } else {
            println!("[Wipers manager] Something went wrong");
            return;
        }
    }
}

#[tokio::main]
async fn main() {
    println!(">>> DEMO SMART WIPERS (SUBSCRIBE) <<<");

    // connect to kuksa client
    let vehicle = Arc::new(Mutex::new(KuksaClient::new("http://127.0.0.1:55555")));

    if let Err(error) = vehicle.lock().await.connect().await {
        println!("{:?}", error);
        return;
    };

    // prepare
    println!("___ Prepare...");
    prepare(&vehicle).await;

    println!("___ Execute function...");

    let hood_vehicle = Arc::clone(&vehicle);
    let wipers_vehicle = Arc::clone(&vehicle);

    let hood_handler = tokio::spawn(async move { manage_hood_subscribe(hood_vehicle).await });

    let wipers_handler = tokio::spawn(async move { manage_wipers_subscribe(wipers_vehicle).await });

    hood_handler.await.unwrap();
    wipers_handler.await.unwrap();
}
