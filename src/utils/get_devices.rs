use std::fs;
use std::fs::OpenOptions;

use evdev_rs::enums::EventCode;
use evdev_rs::enums::EventType;
use evdev_rs::enums::EV_KEY;
use evdev_rs::Device;

use crate::error::NiaServerError;

fn get_device_paths() -> Result<Vec<String>, NiaServerError> {
    let mut result = Vec::new();

    let device_directory = "/dev/input/";
    let iterator = fs::read_dir(device_directory)
        .map_err(|_| NiaServerError::unknown(""))?;

    for entry in iterator {
        let entry = entry.map_err(|_| NiaServerError::unknown(""))?;

        let entry_name = entry.file_name().into_string();

        let entry_name = match entry_name {
            Ok(string) => string,
            _ => continue,
        };

        if entry_name.starts_with("event") {
            result.insert(0, entry_name);
        }
    }

    Ok(result)
}

fn filter_device_paths(
    vector: Vec<String>,
) -> Result<Vec<String>, NiaServerError> {
    let mut result = Vec::new();

    for v in &vector {
        let device_path = format!("/dev/input/{}", v);

        let fd = OpenOptions::new()
            .read(true)
            .open(&device_path)
            .map_err(|_| NiaServerError::unknown(""))?;

        let mut device =
            Device::new().ok_or_else(|| NiaServerError::unknown(""))?;

        device.set_fd(fd).map_err(|_| NiaServerError::unknown(""))?;

        if !device.has_event_type(&EventType::EV_KEY) {
            continue;
        }

        if !device.has_event_code(&EventCode::EV_KEY(EV_KEY::KEY_Q)) {
            continue;
        }

        if device.name() == Some("Nia virtual device") {
            continue;
        }

        result.push(device_path)
    }

    Ok(result)
}

pub fn get_available_devices() -> Result<Vec<String>, NiaServerError> {
    let devices = get_device_paths()?;

    filter_device_paths(devices)
}
