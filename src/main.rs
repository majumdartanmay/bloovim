mod bloo_controller;
mod bloo_tui;

use bloo_tui::TuiController;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // stop_tui()?;
    let mut tui_controller = TuiController::new()?;
    tui_controller.start_tui()?;

    tui_controller.stop_tui()?;
    return Ok(());
}
