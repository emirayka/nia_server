use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use evdev_rs::enums::EventCode;
use evdev_rs::enums::EventType;
use evdev_rs::enums::EV_KEY;
use evdev_rs::Device;

use crate::error::NiaServerError;
use crate::protocol::{DeviceInfo, DeviceModel};

const KEYBOARD_MODELS_DIRECTORY: &'static str = "keyboard_models";

fn read_device_info_from_file(
    device_id: i32,
    device_path: &str,
    device_name: &str,
    file_name: &str,
) -> Result<DeviceInfo, NiaServerError> {
    let path = Path::new(KEYBOARD_MODELS_DIRECTORY).join(file_name);

    let string =
        fs::read_to_string(path).map_err(|_| NiaServerError::unknown(""))?;

    let device_info = DeviceInfo::new(
        device_id,
        false,
        device_path.to_string(),
        device_name.to_string(),
        DeviceModel::from_string(string)?,
    );

    Ok(device_info)
}

fn read_default_device_info(
    device_id: i32,
    device_path: &str,
    device_name: &str,
) -> Result<DeviceInfo, NiaServerError> {
    read_device_info_from_file(
        device_id,
        device_path,
        device_name,
        "default.kbm",
    )
}

fn find_device_info(
    device_id: i32,
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
                device_id,
                device_path,
                device_name,
                &file_name,
            )?;

            return Ok(Some(device_info));
        }
    }

    Ok(None)
}

fn get_device_info(
    device_id: i32,
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
        if let Some(device_info) =
            find_device_info(device_id, device_name, device_path)?
        {
            return Ok(device_info);
        }

        return read_default_device_info(device_id, device_path, device_name);
    }

    read_default_device_info(device_id, device_path, "Unknown")
}

pub fn get_devices_info(
    device_paths: &Vec<String>,
) -> Result<Vec<DeviceInfo>, NiaServerError> {
    let mut result = Vec::new();

    for (id, device_path) in device_paths.iter().enumerate() {
        let device_info = get_device_info(id as i32, device_path)?;

        result.push(device_info)
    }

    Ok(result)
}
