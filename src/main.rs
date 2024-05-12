mod bloo_controller;
mod bloo_tui;

use bloo_tui::BlooTui;
use btleplug::platform::PeripheralId;
use log::debug;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(32);

    let devices: Arc<Mutex<Vec<PeripheralId>>> = Arc::new(Mutex::new(Vec::default()));

    setup()?;
    //
    let mut tui_controller = BlooTui::new()?;
    let f2 = tui_controller.start_tui(Arc::new(Mutex::new(rx)), devices);
    let f1 = bloo_controller::start_bluetooth_stream(&tx);

    let _ = tokio::join!(f1, f2);

    tui_controller.stop_tui()?;
    return Ok(());
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    debug!("Creating bloo session");
    Ok(())
}
