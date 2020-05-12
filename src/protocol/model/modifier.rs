#[derive(Clone, Debug)]
pub struct Modifier {
    keyboard_path: String,
    key_code: i32,
    modifier_alias: String,
}

impl Modifier {
    pub fn new(
        keyboard_path: String,
        key_code: i32,
        modifier_alias: String,
    ) -> Modifier {
        Modifier {
            keyboard_path,
            key_code,
            modifier_alias,
        }
    }

    pub fn get_keyboard_path(&self) -> &str {
        &self.keyboard_path
    }

    pub fn get_key_code(&self) -> &str {
        &self.keyboard_path
    }

    pub fn get_modifier_alias(&self) -> &str {
        &self.keyboard_path
    }

    pub fn into_tuple(self) -> (String, i32, String) {
        (self.keyboard_path, self.key_code, self.modifier_alias)
    }
}
