use crate::constants::Constants;
use crate::message::{send_command, Command};

use simple_kuksa_client::common::{ClientError, Value};

use tokio;
use tokio::sync::mpsc;

async fn is_safe_condition(tx: &mpsc::Sender<Command>) -> bool {
    let speed_state = send_command(tx, |resp_tx| Command::SpeedState { resp: resp_tx }).await;
    let seatbelt_state = send_command(tx, |resp_tx| Command::IsBeltedState { resp: resp_tx }).await;

    println!("seatbelt: {:?} - speed: {:?}", seatbelt_state, speed_state);

    seatbelt_state == Some(Value::Bool(true))
        || speed_state == Some(Value::Float(0.0))
        || seatbelt_state == None
        || speed_state == None
}

async fn set_alert_state(tx: &mpsc::Sender<Command>, state: bool) -> Result<(), ClientError> {
    let hazard_value = if state { "true" } else { "false" };
    send_command(tx, |resp_tx| Command::SetTargetValue {
        signal: Constants::IS_HAZARD_ON.to_string(),
        value: hazard_value.to_string(),
        resp: resp_tx,
    })
    .await?;

    let fan_speed_value = if state {
        Constants::MAX_FAN_SPEED
    } else {
        Constants::MIN_FAN_SPEED
    };
    send_command(tx, |resp_tx| Command::SetTargetValue {
        signal: Constants::LEFT_FAN_SPEED.to_string(),
        value: fan_speed_value.to_string(),
        resp: resp_tx,
    })
    .await?;

    send_command(tx, |resp_tx| Command::SetTargetValue {
        signal: Constants::RIGHT_FAN_SPEED.to_string(),
        value: fan_speed_value.to_string(),
        resp: resp_tx,
    })
    .await?;

    send_command(tx, |resp_tx| Command::SetIsAlerting {
        value: state,
        resp: resp_tx,
    })
    .await;

    Ok(())
}

pub async fn check_alert(tx: &mpsc::Sender<Command>) {
    println!("\nchecking alert...");

    let is_alerting = send_command(tx, |resp_tx| Command::IsAlertingState { resp: resp_tx }).await;
    let is_safe = is_safe_condition(tx).await;

    println!(
        "--> is_safe: {:?}  - is_alerting: {:?}",
        is_safe, is_alerting
    );

    if is_safe && is_alerting {
        match set_alert_state(tx, false).await {
            Ok(_) => {
                println!("Turn off ALERT");
            }
            Err(error) => {
                println!("Error while turning off ALERT: {:?}", error);
            }
        }
    } else if !is_safe && !is_alerting {
        match set_alert_state(tx, true).await {
            Ok(_) => {
                println!("Turn on ALERT");
            }
            Err(error) => {
                println!("Error while turning on ALERT: {:?}", error);
            }
        }
    }
}
