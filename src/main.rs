// Got this piece of code from
// https://github.com/deviceplug/btleplug/blob/master/examples/subscribe_notify_characteristic.rs

// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.
mod bloo_tui;

use bloo_tui::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_tui()?;
    println!("Hello World");
    stop_tui()?;
    return Ok(());
}
