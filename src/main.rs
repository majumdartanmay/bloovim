mod bloo_controller;
mod bloo_tui;

use bloo_tui::BlooTui;
use log::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;
    //
    let mut tui_controller = BlooTui::new()?;
    tui_controller.start_tui().await?;
    bloo_controller::start_bluetooth_stream(&mut tui_controller).await?;

    tui_controller.stop_tui()?;
    return Ok(());
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    debug!("Creating bloo session");
    Ok(())
}
