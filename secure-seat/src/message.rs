use simple_kuksa_client::common::{ClientError, Datapoint, Value};
use simple_kuksa_client::kuksa_client::SubscribeResponse;
use tokio::sync::{mpsc, oneshot};
use tonic::Streaming;

pub type Responder<T> = oneshot::Sender<T>;

#[derive(Debug)]
pub enum Command {
    GetCurrentValue {
        signal: String,
        resp: Responder<Result<Option<Datapoint>, ClientError>>,
    },
    SetCurrentValue {
        signal: String,
        value: String,
        resp: Responder<Result<(), ClientError>>,
    },
    SubscribeCurrentValue {
        signal: String,
        resp: Responder<Result<Streaming<SubscribeResponse>, ClientError>>,
    },
    GetTargetValue {
        signal: String,
        resp: Responder<Result<Option<Datapoint>, ClientError>>,
    },
    SetTargetValue {
        signal: String,
        value: String,
        resp: Responder<Result<(), ClientError>>,
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
        resp: Responder<bool>,
    },
    IsBeltedState {
        resp: Responder<Option<Value>>,
    },
    SetIsBelted {
        value: Option<Value>,
        resp: Responder<Option<Value>>,
    },
    SpeedState {
        resp: Responder<Option<Value>>,
    },
    SetSpeed {
        value: Option<Value>,
        resp: Responder<Option<Value>>,
    },
}

pub async fn send_command<T, F>(tx: &mpsc::Sender<Command>, cmd_constructor: F) -> T
where
    F: FnOnce(Responder<T>) -> Command,
    T: Send + 'static,
{
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = cmd_constructor(resp_tx);
    tx.send(cmd).await.unwrap();
    resp_rx.await.unwrap()
}