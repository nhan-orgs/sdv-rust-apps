use simple_kuksa_client::kuksa_client::SubscribeResponse;
use simple_kuksa_client::{
    common::{self, Value},
    KuksaClient,
};
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

const IS_BELTED:       &str = "Vehicle.Cabin.Seat.Row1.Pos1.IsBelted";
const SPEED:           &str = "Vehicle.Speed";
const IS_HAZARD_ON:    &str = "Vehicle.Body.Lights.IsHazardOn";
const LEFT_FAN_SPEED:  &str = "Vehicle.Cabin.HVAC.Station.Row1.Left.FanSpeed";
const RIGHT_FAN_SPEED: &str = "Vehicle.Cabin.HVAC.Station.Row1.Right.FanSpeed";

const MAX_FAN_SPEED: u8 = 100;
const MIN_FAN_SPEED: u8 = 0;

fn value_from_message(message: SubscribeResponse) -> Value {
    for entry_update in message.updates {
        if let Some(entry) = entry_update.entry {
            return common::value_from_option_datapoint(entry.value);
        }
    }
    Value::String("not found".to_string())
}

fn is_safe_condition() -> bool {
    // seatbelt = true || speed = 0
    true
}

async fn turn_on_alert() {
    // Alert = true
    // Vehicle.Body.Lights.IsHazardOn --> True
    // Vehicle.Cabin.HVAC.Station.Row1.Left.FanSpeed --> 100
    // Vehicle.Cabin.HVAC.Station.Row1.Right.FanSpeed --> 100
}

async fn turn_off_alert() {
    // Alert = false
    // Vehicle.Body.Lights.IsHazardOn --> False
    // Vehicle.Cabin.HVAC.Station.Row1.Left.FanSpeed --> 0
    // Vehicle.Cabin.HVAC.Station.Row1.Right.FanSpeed --> 0
}

async fn check_alert() {
    if is_safe_condition() {
        if true // being alert 
        {
            turn_off_alert().await;
        }
    } else {
        if true // not being alert
        {
            turn_on_alert().await;
        }
    }
}

async fn manage_speed_subscribe(vehicle: Arc<Mutex<KuksaClient>>) {
    // subscribe speed
    println!("# Subscribe vehicle speed...");

    let mut speed_response_stream = match vehicle.lock().await.subscribe_current_value(SPEED).await
    {
        Ok(speed_response_stream) => speed_response_stream,
        Err(error) => {
            println!("Subscribe speed failed: {:?}", error);
            return;
        }
    };

    loop {
        // speed events
        match speed_response_stream.message().await {
            Ok(Some(message)) => {
                // TODO: set current state of speed
                let speed_status = value_from_message(message);

                // TODO: if current state change, call check alert function
                check_alert().await;
            },
            Ok(None) => {
                println!("[Speed manager] Server gone");
                break;
            }
            Err(error) => {
                println!("[Speed manager] Error: {:?}", error);
                break;
            }
        }
    }
}

async fn manage_seatbelt_subscribe(vehicle: Arc<Mutex<KuksaClient>>) {
    // subscribe seatbelt
    println!("# Subscribe driver's seatbelt...");

    let mut seatbelt_response_stream = match vehicle.lock().await.subscribe_current_value(IS_BELTED).await
    {
        Ok(seatbelt_response_stream) => seatbelt_response_stream,
        Err(error) => {
            println!("Subscribe driver's seatbelt failed: {:?}", error);
            return;
        }
    };

    loop {
        // seatbelt events
        match seatbelt_response_stream.message().await {
            Ok(Some(message)) => {
                // TODO: set current state of seatbelt
                let seatbelt_status = value_from_message(message);

                // TODO: if current state change, call check alert function
                check_alert().await;
            },
            Ok(None) => {
                println!("[Seatbelt manager] Server gone");
                break;
            }
            Err(error) => {
                println!("[Seatbelt manager] Error: {:?}", error);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!(">>> DEMO SECURE SEAT <<<");

    // connect to kuksa client
    let vehicle = Arc::new(Mutex::new(KuksaClient::new("http://127.0.0.1:55555")));

    if let Err(error) = vehicle.lock().await.connect().await {
        println!("{:?}", error);
        return;
    };

    println!("___ Execute function...");

    // TODO: current state of seatbelt & speed & alert

    let seatbelt_vehicle = Arc::clone(&vehicle);
    let speed_vehicle = Arc::clone(&vehicle);

    let seatbelt_handler = tokio::spawn(async move { manage_seatbelt_subscribe(seatbelt_vehicle).await });
    let speed_handler = tokio::spawn(async move { manage_speed_subscribe(speed_vehicle).await });

    seatbelt_handler.await.unwrap();
    speed_handler.await.unwrap();
}
