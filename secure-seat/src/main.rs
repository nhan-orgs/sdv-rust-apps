#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;

use simple_kuksa_client::common::{self, ClientError, Datapoint, Value};
use simple_kuksa_client::kuksa_client::SubscribeResponse;
use simple_kuksa_client::KuksaClient;
use tokio;
use tokio::sync::{mpsc, oneshot};
use tonic::Streaming;

type Responder<T> = oneshot::Sender<T>;

const SERVER_ASSREDD: &str = "http://127.0.0.1:55555";
const MAX_MESSAGES: usize = 10;

const IS_BELTED: &str = "Vehicle.Cabin.Seat.Row1.Pos1.IsBelted";
const SPEED: &str = "Vehicle.Speed";
const IS_HAZARD_ON: &str = "Vehicle.Body.Lights.IsHazardOn";
const LEFT_FAN_SPEED: &str = "Vehicle.Cabin.HVAC.Station.Row1.Left.FanSpeed";
const RIGHT_FAN_SPEED: &str = "Vehicle.Cabin.HVAC.Station.Row1.Right.FanSpeed";

const MAX_FAN_SPEED: &str = "100";
const MIN_FAN_SPEED: &str = "0";

#[derive(Debug)]
enum Command {
    GetCurrentValue {
        signal: String,
        resp: Responder<Result<Option<Datapoint>, ClientError>>,
    },
    GetTargetValue {
        signal: String,
        resp: Responder<Result<Option<Datapoint>, ClientError>>,
    },
    SetCurrentValue {
        signal: String,
        value: String,
        resp: Responder<Result<(), ClientError>>,
    },
    SetTargetValue {
        signal: String,
        value: String,
        resp: Responder<Result<(), ClientError>>,
    },
    SubscribeCurrentValue {
        signal: String,
        resp: Responder<Result<Streaming<SubscribeResponse>, ClientError>>,
    },
    SubscribeTargetValue {
        signal: String,
        resp: Responder<Result<Streaming<SubscribeResponse>, ClientError>>,
    },
    IsAlertingState {
        resp: Responder<bool>,
    },
    SetIsAlerting {
        value: bool,
    },
    IsBeltedState {
        resp: Responder<Option<Value>>,
    },
    SetIsBelted {
        value: Option<Value>,
    },
    SpeedState {
        resp: Responder<Option<Value>>,
    },
    SetSpeed {
        value: Option<Value>,
    },
}

fn value_from_message(message: SubscribeResponse) -> Value {
    for entry_update in message.updates {
        if let Some(entry) = entry_update.entry {
            return common::value_from_option_datapoint(entry.value);
        }
    }
    Value::String("not found".to_string())
}

async fn is_safe_condition(tx: &mpsc::Sender<Command>) -> bool {
    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Command::SpeedState { resp: resp_tx }).await;
    let speed_state = resp_rx.await.unwrap();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Command::IsBeltedState { resp: resp_tx }).await;
    let seatbelt_state = resp_rx.await.unwrap();

    println!("seatbelt: {:?} - speed: {:?}", seatbelt_state, speed_state);

    seatbelt_state == Some(Value::Bool(true))
        || speed_state == Some(Value::Float(0.0))
        || seatbelt_state == None
        || speed_state == None
}

async fn turn_on_alert(tx: &mpsc::Sender<Command>) {
    println!("TURNING ON ALERT");

    // set_target_value(vehicle, IS_HAZARD_ON, "true").await;
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::SetTargetValue {
        signal: IS_HAZARD_ON.to_string(),
        value: "true".to_string(),
        resp: resp_tx,
    };
    let _ = tx.send(cmd).await;
    if let Err(error) = resp_rx.await.unwrap() {
        println!("Turn on hazard failed {:?}", error);
    }

    // // set_target_value(vehicle, LEFT_FAN_SPEED, MAX_FAN_SPEED).await;
    // let (resp_tx, resp_rx) = oneshot::channel();
    // let cmd = Command::SetTargetValue {
    //     signal: LEFT_FAN_SPEED.to_string(),
    //     value: MAX_FAN_SPEED.to_string(),
    //     resp: resp_tx,
    // };
    // let _ = tx.send(cmd).await;
    // if let Err(error) = resp_rx.await.unwrap() {
    //     println!("Turn on left fan failed {:?}", error);
    // }

    // // set_target_value(vehicle, RIGHT_FAN_SPEED, MAX_FAN_SPEED).await;
    // let (resp_tx, resp_rx) = oneshot::channel();
    // let cmd = Command::SetTargetValue {
    //     signal: RIGHT_FAN_SPEED.to_string(),
    //     value: MAX_FAN_SPEED.to_string(),
    //     resp: resp_tx,
    // };
    // let _ = tx.send(cmd).await;
    // if let Err(error) = resp_rx.await.unwrap() {
    //     println!("Turn on right fan failed {:?}", error);
    // }

    // alerting = true
    let _ = tx.send(Command::SetIsAlerting { value: true }).await;
}

async fn turn_off_alert(tx: &mpsc::Sender<Command>) {
    println!("TURNING OFF ALERT");

    // set_target_value(vehicle, IS_HAZARD_ON, "false").await;
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::SetTargetValue {
        signal: IS_HAZARD_ON.to_string(),
        value: "false".to_string(),
        resp: resp_tx,
    };
    let _ = tx.send(cmd).await;
    if let Err(error) = resp_rx.await.unwrap() {
        println!("Turn on hazard failed {:?}", error);
    }

    // // set_target_value(vehicle, LEFT_FAN_SPEED, MIN_FAN_SPEED).await;
    // let (resp_tx, resp_rx) = oneshot::channel();
    // let cmd = Command::SetTargetValue {
    //     signal: LEFT_FAN_SPEED.to_string(),
    //     value: MIN_FAN_SPEED.to_string(),
    //     resp: resp_tx,
    // };
    // let _ = tx.send(cmd).await;
    // if let Err(error) = resp_rx.await.unwrap() {
    //     println!("Turn off left fan failed {:?}", error);
    // }

    // // set_target_value(vehicle, RIGHT_FAN_SPEED, MAX_FAN_SPEED).await;
    // let (resp_tx, resp_rx) = oneshot::channel();
    // let cmd = Command::SetTargetValue {
    //     signal: RIGHT_FAN_SPEED.to_string(),
    //     value: MIN_FAN_SPEED.to_string(),
    //     resp: resp_tx,
    // };
    // let _ = tx.send(cmd).await;
    // if let Err(error) = resp_rx.await.unwrap() {
    //     println!("Turn off right fan failed {:?}", error);
    // }

    // alerting = false
    let _ = tx.send(Command::SetIsAlerting { value: false }).await;
}

async fn check_alert(tx: &mpsc::Sender<Command>) {
    println!("\nchecking alert...");

    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::IsAlertingState { resp: resp_tx };
    let _ = tx.send(cmd).await;
    let is_alerting = resp_rx.await.unwrap();

    let is_safe = is_safe_condition(tx).await;

    println!(
        "--> is_safe: {:?} \nis_alerting: {:?}",
        is_safe, is_alerting
    );

    if is_safe {
        if is_alerting {
            turn_off_alert(tx).await;
        }
    } else {
        if !is_alerting {
            turn_on_alert(tx).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // mpsc channel
    let (tx, mut rx) = mpsc::channel(MAX_MESSAGES);
    let seatbelt_tx = tx.clone();
    let speed_tx = tx.clone();

    // vehicle manager
    let shared_data_handler = tokio::spawn(async move {
        let mut is_alerting = false;
        let mut is_belted = None;
        let mut speed = None;

        let mut vehicle = KuksaClient::new(SERVER_ASSREDD);

        if let Err(error) = vehicle.connect().await {
            println!("{:?}", error);
            return;
        };

        while let Some(message) = rx.recv().await {
            match message {
                Command::GetCurrentValue { signal, resp } => {
                    let response = vehicle.get_current_value(&signal).await;
                    let _ = resp.send(response);
                }
                Command::GetTargetValue { signal, resp } => {
                    let response = vehicle.get_target_value(&signal).await;
                    let _ = resp.send(response);
                }
                Command::SetCurrentValue {
                    signal,
                    value,
                    resp,
                } => {
                    let response = vehicle.set_current_value(&signal, &value).await;
                    let _ = resp.send(response);
                }
                Command::SetTargetValue {
                    signal,
                    value,
                    resp,
                } => {
                    let response = vehicle.set_target_value(&signal, &value).await;
                    let _ = resp.send(response);
                }
                Command::SubscribeCurrentValue { signal, resp } => {
                    let response = vehicle.subscribe_current_value(&signal).await;
                    let _ = resp.send(response);
                }
                Command::SubscribeTargetValue { signal, resp } => {
                    let response = vehicle.subscribe_target_value(&signal).await;
                    let _ = resp.send(response);
                }
                Command::IsAlertingState { resp } => {
                    let _ = resp.send(is_alerting);
                }
                Command::SetIsAlerting { value } => {
                    is_alerting = value;
                    // println!("Set is alerting = {:?}", is_alerting);
                }
                Command::IsBeltedState { resp } => {
                    let _ = resp.send(is_belted.clone());
                }
                Command::SetIsBelted { value } => {
                    is_belted = value;
                    // println!("Set is belted = {:?}", is_belted);
                }
                Command::SpeedState { resp } => {
                    let _ = resp.send(speed.clone());
                }
                Command::SetSpeed { value } => {
                    speed = value;
                    // println!("Set speed = {:?}", speed);
                }
            }
        }
    });

    /* TEST ON - OFF ALERT FUNCTIONS
    let test = tokio::spawn(async move {
        turn_on_alert(&tx).await;

        sleep(Duration::from_secs(3));

        turn_off_alert(&tx).await;
    });

    let _ = tokio::join!(shared_data_handler, test);
    */

    // seatbelt subscribe
    let seatbelt_handler = tokio::spawn(async move {
        println!("# Subscribe driver's seatbelt...");

        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::SubscribeCurrentValue {
            signal: IS_BELTED.to_string(),
            resp: resp_tx,
        };
        seatbelt_tx.send(cmd).await.unwrap();

        let mut seatbelt_response_stream = match resp_rx.await.unwrap() {
            Ok(seatbelt_response_stream) => seatbelt_response_stream,
            Err(error) => {
                println!("Subscribe driver's seatbelt failed: {:?}", error);
                return;
            }
        };

        loop {
            match seatbelt_response_stream.message().await {
                Ok(Some(message)) => {
                    let value = value_from_message(message);

                    println!("\n___ Seatbelt update: {:?}", value);

                    let _ = seatbelt_tx
                        .send(Command::SetIsBelted { value: Some(value) })
                        .await;

                    check_alert(&seatbelt_tx).await;
                }
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
    });

    // speed subscribe
    let speed_handler = tokio::spawn(async move {
        println!("# Subscribe vehicle speed...");

        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::SubscribeCurrentValue {
            signal: SPEED.to_string(),
            resp: resp_tx,
        };
        speed_tx.send(cmd).await.unwrap();

        let mut speed_response_stream = match resp_rx.await.unwrap() {
            Ok(speed_response_stream) => speed_response_stream,
            Err(error) => {
                println!("Subscribe driver's speed failed: {:?}", error);
                return;
            }
        };

        loop {
            match speed_response_stream.message().await {
                Ok(Some(message)) => {
                    let value = value_from_message(message);

                    println!("\n___ Speed update: {:?}", value);

                    let _ = speed_tx
                        .send(Command::SetSpeed { value: Some(value) })
                        .await;

                    check_alert(&speed_tx).await;
                }
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
    });

    let _ = tokio::join!(shared_data_handler, seatbelt_handler, speed_handler);
}
