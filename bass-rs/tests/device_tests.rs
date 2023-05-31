use bass_rs::prelude::*;

pub fn device_tests() -> BassResult<()> {
    let all_devices = BassDevice::get_all_devices()?;

    println!("Devices: ");
    for device in all_devices {
        println!("{}", device);
        if device.flags.contains(&DeviceFlags::Init) {
            device.set()?
        }
    }

    Ok(())
}