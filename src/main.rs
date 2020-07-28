//https://www.reddit.com/r/rust/comments/5iczah/are_there_overheads_to_the_additional_types_used/

/*
 Cons with this is
 - that you maybe cant extend the Unit enum type later on because you don't own that type
 - No compile time check that you are giving a function the right data
*/

mod shared {
    pub enum Float64 {
        Meter(f64),
        Feet(f64),
    }
}

mod sensor_team {
    use super::shared::Float64;

    pub fn get_current_distance() -> Float64 {
        Float64::Feet(10.3)
    }
}

mod thrust_team {
    use super::shared::Float64;

    fn get_important_thrust_number() -> Float64 {
        Float64::Meter(0.5)
    }

    pub fn apply_thrust(distance: Float64) {
        let important_thrust_number = get_important_thrust_number();
        if let (Float64::Meter(t), Float64::Meter(d)) = (important_thrust_number, distance) {
            let thrust_value = d * t;
            println!("Thrust value: {:?}", thrust_value);
        } else {
            println!("Failed to calculate thrust")
        }
    }
}

mod systems_team {
    use super::sensor_team::get_current_distance;
    use super::thrust_team::apply_thrust;

    pub fn land_vehicle() {
        let current_distance = get_current_distance();
        apply_thrust(current_distance);
    }
}

use systems_team::land_vehicle;
fn main() {
    land_vehicle();
}
