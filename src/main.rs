mod b_state;
mod bloo_controller;
mod bloo_tui;

use b_state::BState;
use bloo_controller::start_bluetooth_stream;
use bloo_tui::BlooTui;
use btleplug::platform::PeripheralId;
use log::debug;
use std::rc::Rc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;
    let (tx, mut rx) = mpsc::channel(32);
    let mut b_state: BState = BState::new().await;
    let b_state_rc = Rc::new(&mut b_state);

    let mut tui_controller = BlooTui::new()?;
    let mut devices: Vec<PeripheralId> = Vec::default();
    let f1 = tui_controller.start_tui(&mut rx, &mut devices, b_state_rc.as_ref());
    let f2 = start_bluetooth_stream(&tx, b_state_rc.as_ref());

    let _ = tokio::join!(f1, f2);
    return Ok(());
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    debug!("Creating bloo session");
    Ok(())
}
