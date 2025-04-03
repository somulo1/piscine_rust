#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    // Associated function to create a new Light instance
    pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_string(),  // Convert alias to a String
            brightness: 0,             // Initial brightness is 0
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    // Find the light by alias and change its brightness
    if let Some(light) = lights.iter_mut().find(|light| light.alias == alias) {
        light.brightness = value;
    }
}