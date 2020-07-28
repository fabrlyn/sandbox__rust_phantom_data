//https://www.reddit.com/r/rust/comments/5iczah/are_there_overheads_to_the_additional_types_used/

/*
 Cons with this is
 - that you maybe cant extend the Unit enum type later on because you don't own that type
 - No compile time check that you are giving a function the right data
*/

mod shared {
    pub enum Unit {
        Meter,
        Feet,
    }

    pub struct Float64 {
        pub value: f64,
        pub unit: Unit,
    }
}

mod sensor_team {
    use super::shared::{Float64, Unit};

    pub fn get_current_distance() -> Float64 {
        Float64 {
            value: 10.3,
            unit: Unit::Feet,
        }
    }
}

mod thrust_team {
    use super::shared::{Float64, Unit};

    fn get_important_thrust_number() -> Float64 {
        Float64 {
            value: 0.5,
            unit: Unit::Meter,
        }
    }

    pub fn apply_thrust(distance: Float64) {
        let important_thrust_number = get_important_thrust_number();
        if let (Unit::Meter, Unit::Meter) = (important_thrust_number.unit, distance.unit) {
            let thrust_value = distance.value * important_thrust_number.value;
            println!("Thrust value: {:?}", thrust_value);
        } else {
            println!("Failed to calculate thrust")
        }
    }
}

use sensor_team::get_current_distance;
use thrust_team::apply_thrust;

fn main() {
    let current_distance = get_current_distance();
    apply_thrust(current_distance);
}
