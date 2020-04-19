use std::fs::OpenOptions;

use evdev_rs::Device;
use evdev_rs::enums::EventType;
use evdev_rs::enums::EventCode;
use evdev_rs::enums::EV_KEY;

use crate::error::Error;
use std::path::Path;
use std::fs;

const KEYBOARD_MODELS_DIRECTORY: &'static str = "dist";

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    name: String,
    model: String,
}

impl DeviceInfo {
    pub fn new(name: String, model: String) -> DeviceInfo {
        DeviceInfo {
            name,
            model,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }
}

fn read_device_info_from_file(file_name: &str, device_name: &str) -> Result<DeviceInfo, Error> {
    let path = Path::new(KEYBOARD_MODELS_DIRECTORY).join(file_name);

    let string = fs::read_to_string(path)
        .map_err(|_| Error::unknown())?;

    let device_info = DeviceInfo::new(
        device_name.to_string(),
        string,
    );

    Ok(device_info)
}

fn read_default_device_info(device_name: &str) -> Result<DeviceInfo, Error> {
    read_device_info_from_file("default.json", device_name)
}

fn find_device_info(device_name: &str) -> Result<Option<DeviceInfo>, Error> {
    let path = KEYBOARD_MODELS_DIRECTORY;
    let iterator = fs::read_dir(path)
        .map_err(|_| Error::unknown())?;

    for entry in iterator {
        let entry = entry.map_err(|_| Error::unknown())?;
        let file_stem_name = entry.path()
            .file_stem().ok_or_else(|| Error::unknown())?
            .to_str().ok_or_else(|| Error::unknown())?
            .to_string();

        if device_name.contains(&file_stem_name) {
            let file_name = entry.file_name()
                .to_str().ok_or_else(|| Error::unknown())?
                .to_string();

            let device_info = read_device_info_from_file(&file_name, device_name)?;

            return Ok(Some(device_info));
        }
    }

    Ok(None)
}

pub fn get_device_info(device_path: &str) -> Result<DeviceInfo, Error> {
    let fd = OpenOptions::new()
        .read(true)
        .open(device_path)
        .map_err(|_| Error::unknown())?;

    let mut device = Device::new()
        .ok_or_else(|| Error::unknown())?;

    device.set_fd(fd)
        .map_err(|_| Error::unknown())?;

    if let Some(device_name) = device.name() {
        if let Some(device_info) = find_device_info(device_name)? {
            return Ok(device_info);
        }

        return read_default_device_info(device_name);
    }

    read_default_device_info(device_path)
}
