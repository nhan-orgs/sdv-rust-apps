use simple_kuksa_client::common::{self, Value};
use simple_kuksa_client::kuksa_client::SubscribeResponse;

pub fn value_from_message(message: SubscribeResponse) -> Option<Value> {
    for entry_update in message.updates {
        if let Some(entry) = entry_update.entry {
            return common::value_from_datapoint(entry.value);
        }
    }
    return None;
}
