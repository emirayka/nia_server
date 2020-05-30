use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

use evdev_rs::enums::EventCode;
use evdev_rs::enums::EventType;
use evdev_rs::enums::EV_KEY;
use evdev_rs::Device;

use crate::error::{NiaServerError, NiaServerResult};
use crate::protocol::{DeviceInfo, DeviceModel};

const KEYBOARD_MODELS_DIRECTORY: &'static str = "keyboard_models";

pub fn read_device_model_from_path(
    path: PathBuf,
) -> NiaServerResult<DeviceModel> {
    let string =
        fs::read_to_string(&path).map_err(|_| NiaServerError::unknown(""))?;

    DeviceModel::from_string(string).map_err(|_| {
        NiaServerError::unknown(format!(
            "Invalid keyboard model file: {:?}.",
            path
        ))
    })
}

pub fn try_read_device_model(
    device_name: &str,
) -> NiaServerResult<DeviceModel> {
    let path = KEYBOARD_MODELS_DIRECTORY;
    let iterator = fs::read_dir(path).map_err(|_| {
        NiaServerError::unknown("Cannot open keyboard model directory.")
    })?;

    for entry in iterator {
        let entry = entry.map_err(|_| {
            NiaServerError::unknown(
                "Cannot read keyboard model directory contents.",
            )
        })?;
        let file_stem_name = entry
            .path()
            .file_stem()
            .ok_or_else(|| {
                NiaServerError::unknown(
                    "Cannot read keyboard model directory contents.",
                )
            })?
            .to_str()
            .ok_or_else(|| {
                NiaServerError::unknown(
                    "Cannot read keyboard model directory contents.",
                )
            })?
            .to_string();

        if device_name.contains(&file_stem_name) {
            let file_name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| {
                    NiaServerError::unknown(
                        "Cannot read keyboard model directory file",
                    )
                })?
                .to_string();

            let path = Path::new(KEYBOARD_MODELS_DIRECTORY).join(file_name);

            return read_device_model_from_path(path);
        }
    }

    NiaServerError::unknown("Cannot find device model from disk.").into()
}

pub fn get_device_model(device_name: &str) -> DeviceModel {
    match try_read_device_model(device_name) {
        Ok(device_model) => device_model,
        Err(_) => DeviceModel::default(),
    }
}

pub fn get_device_info(
    device_id: usize,
    device_path: &str,
) -> NiaServerResult<DeviceInfo> {
    let fd = OpenOptions::new()
        .read(true)
        .open(device_path)
        .map_err(|_| {
            NiaServerError::unknown(format!(
                "Cannot open file {} for reading",
                device_path
            ))
        })?;

    let mut device = Device::new()
        .ok_or_else(|| NiaServerError::unknown("Cannot create device"))?;

    device.set_fd(fd).map_err(|_| {
        NiaServerError::unknown(format!(
            "Cannot read from file {}.",
            device_path
        ))
    })?;

    let device_name = match device.name() {
        Some(name) => name.to_string(),
        None => String::from("Unknown"),
    };

    let device_model = get_device_model(&device_name);

    let device_info = DeviceInfo::new(
        device_id as i32,
        false,
        device_path,
        &device_name,
        device_model,
    );

    Ok(device_info)
}

pub fn get_devices_info(
    device_paths: &Vec<String>,
) -> NiaServerResult<Vec<DeviceInfo>> {
    let mut devices_info = Vec::new();

    for (device_id, device_path) in device_paths.iter().enumerate() {
        let device_info = get_device_info(device_id, device_path)?;

        devices_info.push(device_info);
    }

    Ok(devices_info)
}
