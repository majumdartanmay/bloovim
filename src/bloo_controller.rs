use btleplug::api::Central;
use btleplug::api::CentralEvent;
use btleplug::api::ScanFilter;
use btleplug::platform::Manager;
use btleplug::{api::Manager as _, Result};
use futures::StreamExt;

pub async fn start_bluetooth_stream() -> Result<()> {
    let manager = Manager::new().await?;

    let adapters = manager.adapters().await?;
    let central = adapters.first().unwrap();

    central.start_scan(ScanFilter::default()).await?;

    let mut events = central.events().await?;
    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) = event {
            println!("Device connected {:?}", id);
        }
    }

    Ok(())
}
