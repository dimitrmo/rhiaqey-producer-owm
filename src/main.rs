use rhiaqey_producer_owm::owm::{OWMSettings, OWM};

#[tokio::main]
async fn main() {
    rhiaqey_producers::exe::run::<OWM, OWMSettings>().await
}
