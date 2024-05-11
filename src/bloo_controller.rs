use btleplug::api::Central;
use btleplug::api::CentralEvent;
use btleplug::api::ScanFilter;
use btleplug::platform::Manager;
use btleplug::{api::Manager as _, Result};
use futures::StreamExt;
use log::debug;
use log::warn;
use tokio::sync::oneshot;

pub async fn start_bluetooth_stream() -> Result<()> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.first().unwrap();

    debug!("Started scan");
    central.start_scan(ScanFilter::default()).await?;

    let mut events = central.events().await?;
    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) = event {
            let (tx, _) = oneshot::channel();
            if tx.send(id).is_err() {
                warn!("Sender packet has been dropped");
            };
        }
    }

    Ok(())
}
