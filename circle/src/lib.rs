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
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

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