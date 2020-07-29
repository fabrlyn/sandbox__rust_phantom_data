//https://www.reddit.com/r/rust/comments/5iczah/are_there_overheads_to_the_additional_types_used/

// Mars lander feet vs meters

mod sensor_team {
    pub fn get_current_distance() -> f64 {
        10.3 // This value is in feet
    }
}

mod thrust_team {
    fn get_important_thrust_number() -> f64 {
        0.5 // This value is in meters
    }

    pub fn apply_thrust(distance: f64) {
        let important_thrust_number = get_important_thrust_number();
        let thrust_value = distance * important_thrust_number;
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
