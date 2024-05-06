use crate::bloo_tui::EventSubscriber;
use btleplug::api::Central;
use btleplug::api::CentralEvent;
use btleplug::api::ScanFilter;
use btleplug::platform::Manager;
use btleplug::{api::Manager as _, Result};
use futures::StreamExt;
use log::debug;

pub async fn start_bluetooth_stream(event_sub: &mut dyn EventSubscriber) -> Result<()> {
    let manager = Manager::new().await?;

    let adapters = manager.adapters().await?;
    let central = adapters.first().unwrap();

    central.start_scan(ScanFilter::default()).await?;
    event_sub.scan_started();

    let mut events = central.events().await?;
    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) = event {
            event_sub.add_device(id)
        }
    }

    Ok(())
}
