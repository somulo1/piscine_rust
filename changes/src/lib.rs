#[derive(Debug, Eq, PartialEq, Clone)]
<<<<<<< HEAD
=======

>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
<<<<<<< HEAD
    // Associated function to create a new Light instance
    pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_string(),  // Convert alias to a String
            brightness: 0,             // Initial brightness is 0
=======
    pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_string(),
            brightness: 0,
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
<<<<<<< HEAD
    // Find the light by alias and change its brightness
    if let Some(light) = lights.iter_mut().find(|light| light.alias == alias) {
        light.brightness = value;
    }
}
=======
    for light in lights.iter_mut() {
        if light.alias == alias {
            light.brightness = value;
            break;
        }
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {


    let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);
    let _result =  change_brightness(&mut lights, "living_room", 200);
    //assert_eq!(result, Some(200));
    assert_eq!(lights[0].brightness, 200);
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
