use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use evdev_rs::enums::EventCode;
use evdev_rs::enums::EventType;
use evdev_rs::enums::EV_KEY;
use evdev_rs::Device;

use crate::error::NiaServerError;
use crate::protocol::DeviceInfo;

const KEYBOARD_MODELS_DIRECTORY: &'static str = "dist";

fn read_device_info_from_file(
    file_name: &str,
    device_path: &str,
    device_name: &str,
) -> Result<DeviceInfo, NiaServerError> {
    let path = Path::new(KEYBOARD_MODELS_DIRECTORY).join(file_name);
    // let path_str = path.to_str().ok_or_else(|| NiaServerError::unknown(""))?;
    // let path_string = String::from(path_str);

    let string =
        fs::read_to_string(path).map_err(|_| NiaServerError::unknown(""))?;

    let device_info = DeviceInfo::new(
        device_path.to_string(),
        device_name.to_string(),
        string,
    );

    Ok(device_info)
}

fn read_default_device_info(
    device_path: &str,
    device_name: &str,
) -> Result<DeviceInfo, NiaServerError> {
    read_device_info_from_file("default.json", device_path, device_name)
}

fn find_device_info(
    device_name: &str,
    device_path: &str,
) -> Result<Option<DeviceInfo>, NiaServerError> {
    let path = KEYBOARD_MODELS_DIRECTORY;
    let iterator =
        fs::read_dir(path).map_err(|_| NiaServerError::unknown(""))?;

    for entry in iterator {
        let entry = entry.map_err(|_| NiaServerError::unknown(""))?;
        let file_stem_name = entry
            .path()
            .file_stem()
            .ok_or_else(|| NiaServerError::unknown(""))?
            .to_str()
            .ok_or_else(|| NiaServerError::unknown(""))?
            .to_string();

        if device_name.contains(&file_stem_name) {
            let file_name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| NiaServerError::unknown(""))?
                .to_string();

            let device_info = read_device_info_from_file(
                &file_name,
                device_path,
                device_name,
            )?;

            return Ok(Some(device_info));
        }
    }

    Ok(None)
}

pub fn get_device_info(
    device_path: &str,
) -> Result<DeviceInfo, NiaServerError> {
    let fd = OpenOptions::new()
        .read(true)
        .open(device_path)
        .map_err(|_| NiaServerError::unknown(""))?;

    let mut device =
        Device::new().ok_or_else(|| NiaServerError::unknown(""))?;

    device.set_fd(fd).map_err(|_| NiaServerError::unknown(""))?;

    if let Some(device_name) = device.name() {
        if let Some(device_info) = find_device_info(device_name, device_path)? {
            return Ok(device_info);
        }

        return read_default_device_info(device_path, device_name);
    }

    read_default_device_info(device_path, "Unknown")
}
