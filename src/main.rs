mod shared {
    use std::fmt;
    use std::marker::PhantomData;

    pub struct Meter;
    pub struct Feet;

    pub struct Float64<T> {
        value: f64,
        _phantom_data: PhantomData<T>,
    }

    impl<T> Float64<T> {
        pub fn new(value: f64) -> Float64<T> {
            return Float64 {
                value,
                _phantom_data: PhantomData,
            };
        }
    }

    impl<T> std::fmt::Display for Float64<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl<T> std::ops::Mul for Float64<T> {
        type Output = Self;
        fn mul(self, rhs: Self) -> <Self as std::ops::Mul<Self>>::Output {
            Float64 {
                value: self.value * rhs.value,
                _phantom_data: PhantomData,
            }
        }
    }
}

mod sensor_team {
    use super::shared::{Feet, Float64};

    pub fn get_current_distance() -> Float64<Feet> {
        Float64::new(10.3)
    }
}

mod thrust_team {
    use super::shared::{Float64, Meter};

    fn get_important_thrust_number() -> Float64<Meter> {
        Float64::new(0.5) // This value is in meters
    }

    pub fn apply_thrust(distance: Float64<Meter>) {
        let important_thrust_number = get_important_thrust_number();
        let thrust_value = distance * important_thrust_number;
        println!("Thrust value: {}", thrust_value);
    }
}

mod systems_team {
    use super::sensor_team::get_current_distance;
    use super::thrust_team::apply_thrust;

    pub fn land_spaceship() {
        let current_distance = get_current_distance();
        apply_thrust(current_distance);
    }
}

use systems_team::land_spaceship;
fn main() {
    land_spaceship();
}
