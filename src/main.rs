use crate::owm::{OWMSettings, OWM};

pub mod owm;

#[tokio::main]
async fn main() {
    rhiaqey_producers::exe::run::<OWM, OWMSettings>().await
}
