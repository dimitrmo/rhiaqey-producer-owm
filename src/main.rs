use log::{debug, info};
use rhiaqey_sdk_rs::producer::{Producer, ProducerMessage, ProducerMessageReceiver};
use rhiaqey_sdk_rs::settings::Settings;
use serde::Deserialize;
use serde_json::json;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Mutex;

#[derive(Default)]
struct OWM {
    sender: Option<UnboundedSender<ProducerMessage>>,
    settings: Arc<Mutex<OWMSettings>>,
}

#[derive(Deserialize, Clone, Debug)]
enum OWMSettingsMode {
    XML,
    HTML,
}

#[derive(Deserialize, Clone, Debug)]
enum OWMSettingsUnits {
    STANDARD,
    METRIC,
    IMPERIAL,
}

// every 5 minutes
fn default_interval() -> Option<u64> {
    Some(300_000)
}

// request timeouts out in 5 seconds
fn default_timeout() -> Option<u64> {
    Some(5000)
}

fn default_url() -> Option<String> {
    Some("https://api.openweathermap.org/data/2.5/weather".to_string())
}

#[derive(Deserialize, Clone, Default, Debug)]
struct OWMSettings {
    #[serde(alias = "Latitude")]
    pub latitude: f64,
    #[serde(alias = "Longitude")]
    pub longitude: f64,
    #[serde(alias = "APIKey")]
    pub api_key: String,
    #[serde(alias = "Mode")]
    pub mode: Option<OWMSettingsMode>,
    #[serde(alias = "Units")]
    pub units: Option<OWMSettingsUnits>,
    #[serde(alias = "Language")]
    pub language: Option<String>,
    #[serde(alias = "Url", default = "default_url")]
    pub url: Option<String>,
    #[serde(alias = "Interval", default = "default_interval")]
    pub interval_in_millis: Option<u64>,
    #[serde(alias = "Timeout", default = "default_timeout")]
    pub timeout_in_millis: Option<u64>,
}

impl Settings for OWMSettings {
    //
}

impl Producer<OWMSettings> for OWM {
    fn setup(&mut self, settings: Option<OWMSettings>) -> ProducerMessageReceiver {
        info!("setting up {}", Self::kind());

        self.settings = Arc::new(Mutex::new(settings.unwrap_or(OWMSettings::default())));

        let (sender, receiver) = unbounded_channel::<ProducerMessage>();
        self.sender = Some(sender);

        Ok(receiver)
    }

    async fn set_settings(&mut self, settings: OWMSettings) {
        let mut locked_settings = self.settings.lock().await;
        *locked_settings = settings;
        debug!("new settings updated");
    }

    async fn start(&mut self) {
        info!("starting {}", Self::kind());

        let sender = self.sender.clone().unwrap();
        let settings = self.settings.clone();

        tokio::task::spawn(async move {
            loop {
                let settings = settings.lock().await.clone();
                let interval = settings.interval_in_millis;
                // TODO: Implement me
                tokio::time::sleep(Duration::from_millis(interval.unwrap())).await;
            }
        });
    }

    fn schema() -> serde_json::value::Value {
        json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "Latitude": {
                    "type": "number",
                    "examples": [ 29.051368 ],
                },
                "Longitude": {
                    "type": "number",
                    "examples": [ 16.424534 ],
                },
                "APIKey": {
                    "type": "string",
                    "examples": [ "b57ef517-3296-45eb-af2a-0e3fbf130a2c" ],
                },
                "Mode": {
                    "type": "string",
                    "enum": [ "xml", "html" ]
                },
                "Units": {
                    "type": "string",
                    "enum": [ "standard", "metric", "imperial" ]
                },
                "Language": {
                    "type": "string"
                },
                "Url": {
                    "type": "string",
                    "examples": [ "https://api.openweathermap.org/data/2.5/weather" ],
                },
                "Interval": {
                    "type": "integer",
                    "examples": [ 5000 ],
                    "minimum": 1000
                },
                "Timeout": {
                    "type": "integer",
                    "examples": [ 15000 ],
                    "minimum": 1000
                }
            },
            "required": [
                "Latitude",
                "Longitude",
                "APIKey",
                "Mode"
            ],
            "additionalProperties": false
        })
    }

    async fn metrics(&self) -> serde_json::value::Value {
        json!({})
    }

    fn kind() -> String {
        String::from("owm")
    }
}

#[tokio::main]
async fn main() {
    rhiaqey_producers::exe::run::<OWM, OWMSettings>().await
}
