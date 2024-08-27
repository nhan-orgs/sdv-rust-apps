mod alert;
mod constants;
mod message;
mod utils;

use alert::check_alert;
use constants::Constants;
use message::{send_command, Command};
use utils::value_from_message;

use simple_kuksa_client::KuksaClient;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::{self, sync::mpsc};

async fn handle_shared_data(mut rx: Receiver<Command>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut is_alerting = false;
        let mut is_belted = None;
        let mut speed = None;

        let mut vehicle = KuksaClient::new(Constants::SERVER_ADDRESS);
        if vehicle.connect().await.is_err() {
            println!("Connection failed");
            return;
        }

        while let Some(message) = rx.recv().await {
            match message {
                Command::GetCurrentValue { signal, resp } => {
                    let response = vehicle.get_current_value(&signal).await;
                    resp.send(response).unwrap();
                }
                Command::SetCurrentValue {
                    signal,
                    value,
                    resp,
                } => {
                    let response = vehicle.set_current_value(&signal, &value).await;
                    resp.send(response).unwrap();
                }
                Command::SubscribeCurrentValue { signal, resp } => {
                    let response = vehicle.subscribe_current_value(&signal).await;
                    resp.send(response).unwrap();
                }
                Command::GetTargetValue { signal, resp } => {
                    let response = vehicle.get_target_value(&signal).await;
                    resp.send(response).unwrap();
                }
                Command::SetTargetValue {
                    signal,
                    value,
                    resp,
                } => {
                    let response = vehicle.set_target_value(&signal, &value).await;
                    resp.send(response).unwrap();
                }
                Command::SubscribeTargetValue { signal, resp } => {
                    let response = vehicle.subscribe_target_value(&signal).await;
                    resp.send(response).unwrap();
                }
                Command::IsAlertingState { resp } => {
                    resp.send(is_alerting).unwrap();
                }
                Command::SetIsAlerting { value, resp } => {
                    is_alerting = value;
                    resp.send(is_alerting.clone()).unwrap();
                }
                Command::IsBeltedState { resp } => {
                    resp.send(is_belted.clone()).unwrap();
                }
                Command::SetIsBelted { value, resp } => {
                    is_belted = value;
                    resp.send(is_belted.clone()).unwrap();
                }
                Command::SpeedState { resp } => {
                    resp.send(speed.clone()).unwrap();
                }
                Command::SetSpeed { value, resp } => {
                    speed = value;
                    resp.send(speed.clone()).unwrap();
                }
            }
        }
    })
}

async fn handle_speed_subscription(tx: Sender<Command>) -> JoinHandle<()> {
    tokio::spawn(async move {
        println!("# Subscribe vehicle speed...");

        let mut speed_response_stream =
            match send_command(&tx, |resp_tx| Command::SubscribeCurrentValue {
                signal: Constants::SPEED.to_string(),
                resp: resp_tx,
            })
            .await
            {
                Ok(stream) => stream,
                Err(error) => {
                    println!("Subscription to {} failed: {:?}", Constants::SPEED, error);
                    return;
                }
            };

        loop {
            match speed_response_stream.message().await {
                Ok(Some(message)) => {
                    let value = value_from_message(message);

                    println!("\n___ Speed update: {:?}", value);

                    send_command(&tx, |resp_tx| Command::SetSpeed {
                        value: value,
                        resp: resp_tx,
                    })
                    .await;

                    check_alert(&tx).await;
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
    })
}

async fn handle_seatbelt_subscription(tx: Sender<Command>) -> JoinHandle<()> {
    tokio::spawn(async move {
        println!("# Subscribe driver's seatbelt...");

        let mut seatbelt_response_stream =
            match send_command(&tx, |resp_tx| Command::SubscribeCurrentValue {
                signal: Constants::IS_BELTED.to_string(),
                resp: resp_tx,
            })
            .await
            {
                Ok(stream) => stream,
                Err(error) => {
                    println!(
                        "Subscription to {} failed: {:?}",
                        Constants::IS_BELTED,
                        error
                    );
                    return;
                }
            };

        loop {
            match seatbelt_response_stream.message().await {
                Ok(Some(message)) => {
                    let value = value_from_message(message);

                    println!("\n___ Seatbelt update: {:?}", value);

                    send_command(&tx, |resp_tx| Command::SetIsBelted {
                        value: value,
                        resp: resp_tx,
                    })
                    .await;

                    check_alert(&tx).await;
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
    })
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(Constants::MAX_MESSAGES);

    let shared_data_handler = handle_shared_data(rx).await;

    let seatbelt_handler = handle_seatbelt_subscription(tx.clone()).await;

    let speed_handler = handle_speed_subscription(tx.clone()).await;

    let _ = tokio::join!(shared_data_handler, seatbelt_handler, speed_handler);
}
