<<<<<<< HEAD
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,  // Center of the circle, which is a Point
    pub radius: f64,    // The radius of the circle
}

impl Circle {
    // Associated function to create a new Circle
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y), // Initialize the center as a Point
            radius,              // Set the radius
        }
    }

    // Method to calculate the diameter of the circle
=======
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}


impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius,
        }
    }
    
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

<<<<<<< HEAD
    // Method to calculate the area of the circle
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // Method to check if two circles intersect
    pub fn intersect(&self, other: Circle) -> bool {
        let distance = self.center.distance(other.center); // Get the distance between the centers
        distance < (self.radius + other.radius)            // Check if the distance is smaller than the sum of the radii
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);  // Tuple struct to hold x and y coordinates

impl Point {
    // Method to calculate the distance between two points
    pub fn distance(&self, other: Point) -> f64 {
        let dx = self.0 - other.0;  // x2 - x1
        let dy = self.1 - other.1;  // y2 - y1
        (dx.powi(2) + dy.powi(2)).sqrt()  // Euclidean distance formula
    }
}
=======
    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) <= (self.radius + other.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point(80.0, 115.0),
            radius: 30.0,
        };
        let point_a = Point(1.0, 1.0);
        let point_b = Point(0.0, 0.0);  
        assert_eq!(circle.area(), 70685.83470577035);
        assert_eq!(circle.diameter(), 300.0);
        assert_eq!(circle1.diameter(), 60.0);
        assert_eq!(circle.intersect(circle1), false);
        assert_eq!(point_a.distance(point_b), 1.4142135623730951);
    }
}
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
