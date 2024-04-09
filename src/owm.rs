use anyhow::Context;
use log::{debug, info, trace, warn};
use reqwest::Response;
use rhiaqey_sdk_rs::message::MessageValue;
use rhiaqey_sdk_rs::producer::{Producer, ProducerMessage, ProducerMessageReceiver};
use rhiaqey_sdk_rs::settings::Settings;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha256::digest;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct OWM {
    sender: Option<UnboundedSender<ProducerMessage>>,
    settings: Arc<Mutex<OWMSettings>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OWMSettingsUnits {
    STANDARD,
    METRIC,
    IMPERIAL,
}

impl Display for OWMSettingsUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OWMSettingsUnits::STANDARD => String::from("standard"),
            OWMSettingsUnits::METRIC => String::from("metrics"),
            OWMSettingsUnits::IMPERIAL => String::from("imperial"),
        };
        write!(f, "{}", str)
    }
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
    Some("https://api.openweathermap.org/data/3.0/onecall".to_string())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OWMSettingsExcludePart {
    #[serde(rename = "current")]
    Current,
    #[serde(rename = "minutely")]
    Minutely,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "alerts")]
    Alerts,
}

impl Display for OWMSettingsExcludePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OWMSettingsExcludePart::Current => String::from("current"),
            OWMSettingsExcludePart::Minutely => String::from("minutely"),
            OWMSettingsExcludePart::Hourly => String::from("hourly"),
            OWMSettingsExcludePart::Daily => String::from("daily"),
            OWMSettingsExcludePart::Alerts => String::from("alerts"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct OWMSettings {
    #[serde(alias = "Latitude")]
    pub latitude: f32,
    #[serde(alias = "Longitude")]
    pub longitude: f32,
    #[serde(alias = "APIKey")]
    pub api_key: String,
    #[serde(alias = "Exclude")]
    pub exclude: Option<Vec<OWMSettingsExcludePart>>,
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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponseWeather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponseCurrent {
    dt: u64,
    sunrise: u64,
    sunset: u64,
    temp: f32,
    feels_like: f32,
    pressure: f32,
    humidity: f32,
    dew_point: f32,
    uvi: f32,
    clouds: i32,
    visibility: f32,
    wind_speed: f32,
    wind_deg: f32,
    weather: Vec<OWMAPIOneCallResponseWeather>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponseHourly {
    dt: u64,
    temp: f32,
    feels_like: f32,
    humidity: f32,
    dew_point: f32,
    uvi: f32,
    clouds: i32,
    wind_speed: f32,
    wind_deg: f32,
    wind_gust: f32,
    weather: Vec<OWMAPIOneCallResponseWeather>,
    pop: i32,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponseDailyTemp {
    day: f32,
    min: f32,
    max: f32,
    night: f32,
    eve: f32,
    morn: f32,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponseDailyFeelsLike {
    day: f32,
    night: f32,
    eve: f32,
    morn: f32,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponseDaily {
    dt: u64,
    sunrise: u64,
    sunset: u64,
    moonrise: u64,
    moonset: u64,
    moon_phase: f32,
    summary: String,
    temp: OWMAPIOneCallResponseDailyTemp,
    feels_like: OWMAPIOneCallResponseDailyFeelsLike,
    pressure: f32,
    humidity: f32,
    dew_point: f32,
    wind_speed: f32,
    wind_deg: f32,
    wind_gust: f32,
    weather: Vec<OWMAPIOneCallResponseWeather>,
    clouds: i32,
    uvi: f32,
    pop: i32,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponseMinutely {
    dt: u64,
    precipitation: i32,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponseAlert {
    sender_name: String,
    event: String,
    start: u64,
    end: u64,
    description: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
struct OWMAPIOneCallResponse {
    lat: f32,
    lon: f32,
    timezone: String,
    timezone_offset: i32,
    current: Option<OWMAPIOneCallResponseCurrent>,
    minutely: Option<Vec<OWMAPIOneCallResponseMinutely>>,
    hourly: Option<Vec<OWMAPIOneCallResponseHourly>>,
    alerts: Option<Vec<OWMAPIOneCallResponseAlert>>,
}

impl OWM {
    fn create_endpoint(settings: OWMSettings) -> anyhow::Result<String> {
        let mut url = format!(
            "{}?lat={}&lon={}&appid={}",
            settings.url.unwrap_or(default_url().unwrap()),
            settings.latitude,
            settings.longitude,
            settings.api_key
        );

        if let Some(exclude) = settings.exclude {
            let parts = exclude
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            url = format!("{}&exclude={}", url, parts);
        }

        if let Some(units) = settings.units {
            url = format!("{}&units={}", url, units.to_string());
        } else {
            url = format!("{}&units={}", url, OWMSettingsUnits::STANDARD.to_string());
        }

        if let Some(lang) = settings.language {
            url = format!("{}&lang={}", url, lang);
        }

        trace!("url ready: {}", url);

        Ok(url)
    }

    async fn send_request(settings: OWMSettings) -> anyhow::Result<Response> {
        info!("fetching owm");

        let endpoint = Self::create_endpoint(settings.clone())?;

        let client = reqwest::Client::new();
        let timeout = settings
            .timeout_in_millis
            .unwrap_or(default_timeout().unwrap());

        client
            .get(endpoint)
            .timeout(Duration::from_millis(timeout))
            .send()
            .await
            .context("failed to send request")
    }

    async fn fetch_weather(settings: OWMSettings) -> anyhow::Result<OWMAPIOneCallResponse> {
        info!("downloading weather");

        let res = Self::send_request(settings)
            .await
            .context("failed to send request")?;

        let text = res
            .text()
            .await
            .context("failed to get full response as text")?;

        trace!("response as text: {}", text);

        let response = serde_json::from_str::<OWMAPIOneCallResponse>(text.as_str())
            .context("failed to deserialize api onecall")?;

        trace!("owm report downloaded");

        Ok(response)
    }

    fn prepare_message(payload: OWMAPIOneCallResponse) -> anyhow::Result<ProducerMessage> {
        debug!("preparing message from response");

        let timestamp = match payload.current {
            None => {
                let epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
                epoch.as_secs()
            }
            Some(ref payload) => payload.dt,
        };

        let tag = Some(digest(format!(
            "{}-{}-{}",
            timestamp, payload.lat, payload.lon
        )));

        let json = serde_json::to_value(payload.clone())?;

        Ok(ProducerMessage {
            key: String::from("owm/onecall"),
            value: MessageValue::Json(json),
            category: Some(String::from("weather")),
            size: None,
            timestamp: Some(timestamp * 1000),
            tag,
        })
    }
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
                let mut interval = settings
                    .interval_in_millis
                    .unwrap_or(default_interval().unwrap());

                trace!("settings dump {:?}", settings);

                if settings.api_key.is_empty() {
                    warn!("APIKey was not found");
                    interval = 5000;
                } else if settings.latitude == 0f32 {
                    warn!("Invalid value for Latitude: 0.0");
                    interval = 5000;
                } else if settings.longitude == 0f32 {
                    warn!("Invalid value for Longitude: 0.0");
                    interval = 5000;
                } else {
                    match Self::fetch_weather(settings).await {
                        Ok(response) => {
                            trace!("we have our response {:?}", response);
                            let msg =
                                Self::prepare_message(response).expect("failed to prepare message");
                            sender.send(msg).expect("failed to send message");
                            trace!("message sent");
                        }
                        Err(err) => {
                            warn!("error fetching weather: {}", err);
                        }
                    }
                }

                tokio::time::sleep(Duration::from_millis(interval)).await;
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
                "Exclude": {
                    "type": "array",
                    "items": {
                        "type": "string",
                        "enum": [ "current", "minutely", "hourly", "daily", "alerts" ]
                    }
                },
                "Units": {
                    "type": "string",
                    "enum": [ "standard", "metric", "imperial" ]
                },
                "Language": {
                    "type": "string",
                    "minLength": 2,
                    "maxLength": 5
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
                "APIKey"
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
