#[derive(Clone, Debug)]
pub struct DeviceInfo {
    path: String,
    name: String,
    model: String,
}

impl DeviceInfo {
    pub fn new(path: String, name: String, model: String) -> DeviceInfo {
        DeviceInfo { path, name, model }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }
}
