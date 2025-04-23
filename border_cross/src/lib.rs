#[allow(dead_code)]
pub struct Car<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
}

#[allow(dead_code)]
pub struct Truck<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
    pub load_tons: u32,
}

pub trait Vehicle {
    fn model(&self) -> &str;
    fn year(&self) -> u32;
}

impl Vehicle for Truck<'_> {
    fn model(&self) -> &str {
        self.model
    }

    fn year(&self) -> u32 {
        self.year
    }
}

impl Vehicle for Car<'_> {
    fn model(&self) -> &str {
        self.model
    }

    fn year(&self) -> u32 {
        self.year
    }
}

// Function to get all models from a list of vehicles
fn all_models(list: Vec<&dyn Vehicle>) -> Vec<&str> {
    list.iter().map(|vehicle| vehicle.model()).collect()
}

// fn main() {
//     // List of vehicles (cars and trucks)
//     let vehicles: Vec<&dyn Vehicle> = vec![
//         &Car {
//             plate_nbr: "A3D5C7",
//             model: "Model 3",
//             horse_power: 325,
//             year: 2010,
//         },
//         &Truck {
//             plate_nbr: "V3D5CT",
//             model: "Ranger",
//             horse_power: 325,
//             year: 2010,
//             load_tons: 40,
//         },
//     ];

//     // Call all_models to get the models
//     println!("{:?}", all_models(vehicles)); // Output: ["Model 3", "Ranger"]
// }
