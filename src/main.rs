mod bloo_controller;
mod bloo_tui;

use bloo_tui::BlooTui;
use log::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;
    //
    let mut tui_controller = BlooTui::new()?;
    let f2 = tui_controller.start_tui();
    let f1 = bloo_controller::start_bluetooth_stream();

    let _ = tokio::join!(f1, f2);

    tui_controller.stop_tui()?;
    return Ok(());
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    debug!("Creating bloo session");
    Ok(())
}
