//https://www.reddit.com/r/rust/comments/5iczah/are_there_overheads_to_the_additional_types_used/

// Mars lander feet vs meters

/*
This fixes compile time issues but is a bit more clumsey when it comes to addition/multiplication etc (Make a version of this)
Need to implement duplicates for every combination. (Check this)
Also doesn't scale for addning new types (check this)
*/

mod shared {
    pub struct Float64InFeet {
        pub value: f64,
    }

    pub struct Float64InMeter {
        pub value: f64,
    }
}

mod sensor_team {
    use super::shared::Float64InFeet;

    pub fn get_current_distance() -> Float64InFeet {
        Float64InFeet { value: 10.3 }
    }
}

mod thrust_team {
    use super::shared::Float64InMeter;

    fn get_important_thrust_number() -> Float64InMeter {
        Float64InMeter { value: 0.5 }
    }

    pub fn apply_thrust(distance: Float64InMeter) {
        let important_thrust_number = get_important_thrust_number();
        let thrust_value = distance.value * important_thrust_number.value;
        println!("Thrust value: {:?}", thrust_value);
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
