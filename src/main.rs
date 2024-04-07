use rhiaqey_sdk_rs::producer::{Producer, ProducerMessageReceiver};
use rhiaqey_sdk_rs::settings::Settings;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Default)]
struct OWM {
    //
}

#[derive(Deserialize, Clone, Default, Debug)]
struct OWMSettings {
    //
}

impl Settings for OWMSettings {
    //
}

impl Producer<OWMSettings> for OWM {
    fn setup(&mut self, _settings: Option<OWMSettings>) -> ProducerMessageReceiver {
        todo!()
    }

    async fn set_settings(&mut self, _settings: OWMSettings) {
        todo!()
    }

    async fn start(&mut self) {
        todo!()
    }

    fn schema() -> serde_json::value::Value {
        todo!()
    }

    async fn metrics(&self) -> serde_json::value::Value {
        todo!()
    }

    fn kind() -> String {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    rhiaqey_producers::exe::run::<OWM, OWMSettings>().await
}
