mod b_state;
mod bloo_tui;

use b_state::BState;
use bloo_tui::BlooTui;
use log::debug;
use std::rc::Rc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;
    let mut b_state: BState = BState::new().await;
    let b_state_rc = Rc::new(&mut b_state);

    let mut tui_controller = BlooTui::new()?;
    let mut devices: Vec<String> = Vec::default();
    tui_controller
        .start_tui(&mut devices, b_state_rc.as_ref())
        .await?;
    return Ok(());
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    debug!("Creating bloo session");
    Ok(())
}
