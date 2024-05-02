// Got this piece of code from
// https://github.com/deviceplug/btleplug/blob/master/examples/subscribe_notify_characteristic.rs

// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.
mod bloo_tui;
mod controller;

use bloo_tui::*;
use controller::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adapter_name = get_adapter_name().await?;
    start_tui(&adapter_name)?;

    stop_tui()?;
    return Ok(());
}
