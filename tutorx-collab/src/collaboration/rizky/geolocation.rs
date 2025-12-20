struct Location {
    lat: f64,
    lng: f64,
}

struct Warehouse {
    name: string,
    location: Location,
    radius_km: f64,
}

impl Warehouse {
    fn new(name: &str, lat: f64, lng: f64, radius: f64) -> Self {
        Self {
            name: name.to_string(),
            location: Location { lat, lng },
            radius_km: radius,
        }
    } 

    // Basic euclidean distance formula
    fn calculate_distance(&self, target: &Location) -> f64 {
        let dx = self.location.lat - target.lat;
        let dy = self.location.lng - target.lng;

        return ((dx.powi(2) + dy.powi(2))).sqrt() * 111.0;
    }

    fn can_deliver_to(&self, customer_loc: &Location) -> bool {
        let distance = self.calculate_distance(customer_loc);
        return distance <= self.radius_km;
    }
}

fn main() {
    let warehouse = Warehouse::new("Central", 50.1234, -85.12344, 10.0);

    let customer_loc = Location{
        lat: 45.1344,
        lng: -80.61243,
    },

    if warehouse.can_deliver_to(&customer_loc) {
        print!("Address is within the {}")
    }
}