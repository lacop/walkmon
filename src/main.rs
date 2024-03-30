use anyhow::anyhow;
use btleplug::api::Central as _;
use btleplug::api::Manager as _;
use btleplug::api::Peripheral as _;
use btleplug::api::ScanFilter;
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Scanning for Bluetooth adapters...");
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.into_iter().next().ok_or(anyhow!("No adapters"))?;

    println!("Scanning for WalkingPad...");
    let walkingpad = find_walkingpad(&adapter).await?;
    println!("Discovered device: {:?}", walkingpad.properties().await?);

    println!("Connecting to WalkingPad...");
    walkingpad.connect().await?;
    walkingpad.discover_services().await?;

    let read_characteristic = walkingpad
        .characteristics()
        .into_iter()
        .find(|c| c.uuid.as_fields().0 == 0x0000fe01)
        .ok_or(anyhow!("No read characteristic"))?;
    let write_characteristic = walkingpad
        .characteristics()
        .into_iter()
        .find(|c| c.uuid.as_fields().0 == 0x0000fe02)
        .ok_or(anyhow!("No write characteristic"))?;

    println!("Reading data from WalkingPad...");
    println!("Press Ctrl+C to stop.");

    let mut events = walkingpad.notifications().await?;
    walkingpad.subscribe(&read_characteristic).await?;

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                // TODO: Make nicer constructor.
                walkingpad.write(&write_characteristic, &[0xf7, 0xa2, 0x00, 0x00, 0xa2, 0xfd], btleplug::api::WriteType::WithoutResponse).await?;
            }
            event = events.next() => {
                if let Some(notification) = event {
                    handle_data(&notification.value)?;
                }
            }
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    println!("Stopping...");
    Ok(())
}

async fn find_walkingpad(adapter: &Adapter) -> anyhow::Result<Peripheral> {
    let mut events = adapter.events().await?;
    adapter.start_scan(ScanFilter::default()).await?;

    while let Some(event) = events.next().await {
        match event {
            btleplug::api::CentralEvent::DeviceDiscovered(id) => {
                let peripheral = adapter.peripheral(&id).await?;
                let properties = peripheral.properties().await?;
                let local_name = properties.map(|p| p.local_name).flatten();
                if local_name.as_deref() == Some("WalkingPad") {
                    adapter.stop_scan().await?;
                    return Ok(peripheral);
                }
            }
            _ => {} // Ignore.
        }
    }

    Err(anyhow!("WalkingPad not found"))
}

fn handle_data(data: &[u8]) -> anyhow::Result<()> {
    if data.get(0..2) != Some(&[0xf8, 0xa2]) {
        return Ok(()); // Unknown packet.
    }

    let speed = *data.get(3).ok_or(anyhow!("No speed"))?;
    let time = extract_value(data, 5)?;
    let distance = extract_value(data, 8)?;
    let steps = extract_value(data, 11)?;

    println!(
        "Speed: {:.1} km/h, Time: {}:{:02}:{:02}, Distance: {:.3} m, Steps: {}",
        speed as f32 / 10.,
        time / 3600,
        (time / 60) % 60,
        time % 60,
        distance as f32 / 100.,
        steps
    );

    Ok(())
}

fn extract_value(bytes: &[u8], offset: usize) -> anyhow::Result<u32> {
    // Weird big-endian 24-bit integer.
    bytes
        .get(offset..offset + 3)
        .map(|b| u32::from_be_bytes([0, b[0], b[1], b[2]]))
        .ok_or(anyhow!("No value"))
}
