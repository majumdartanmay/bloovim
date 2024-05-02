use btleplug::api::Central;
use btleplug::platform::Manager;
use btleplug::{api::Manager as _, Result};

struct BlooDevice {
    device_name: String,
    device_mac: String,
    is_connected: bool,
}

struct AppState {
    devices: Vec<BlooDevice>,
    adapter_info: String,
}

pub async fn get_adapter_name() -> Result<String> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    let adapter_info = match adapter_list.first() {
        Some(info) => info.adapter_info().await?,
        None => String::from("NA"),
    };
    Ok(adapter_info)
}
