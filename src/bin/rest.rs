use std::error::Error;
use wealth_tracker::config::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new().unwrap();
    let _ = dbg!(settings);

    Ok(())
}
