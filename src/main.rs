use std::marker::PhantomData;
//https://www.reddit.com/r/rust/comments/5iczah/are_there_overheads_to_the_additional_types_used/

// Mars lander feet vs meters
/*
Could be done with enum but it would pass compilation.


Side not: Try implemention deref for conting with the number;

The distance sensor outputs a float
The throttle takes distance sensor output as input


V1 does not know if its meters or feet


V2 does not at compile time if its meters or feet - check if its true that this adds not runtime overhead.
*/

/*
This solution does not fix compile time checks

enum Unit {
    Meter,
    Feet,
}

struct V1Float64 {
    value: f64,
    unit: Unit,
}
*/

/*
This fixes compile time issues but is a bit more clumsey when it comes to addition/multiplication etc (Make a version of this)
Need to implement duplicates for every combination. (Check this)
Also doesn't scale for addning new types (check this)

struct Float64InMeter(f64);
struct Float64InFeet(f64);
*/

#[derive(Debug)]
struct Meter;

#[derive(Debug)]
struct Feet;

#[derive(Debug)]
struct Millimeter;

#[derive(Debug)]
struct Float64<T> {
    value: f64,
    _phantom_data: PhantomData<T>,
}

impl<T> Float64<T> {
    fn new(value: f64) -> Float64<T> {
        return Float64 {
            value,
            _phantom_data: PhantomData,
        };
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

fn get_current_distance() -> Float64<Feet> {
    Float64::new(10.3)
}

// Add automatic conversion from Feet to Meter with std::convert::From
fn apply_thrust(distance: Float64<Feet>) {
    //Do something with the thrust value
    let thrust_value = distance * Float64::new(0.5);

    println!("Trust value: {:?}", thrust_value);
}

fn main() {
    let a: Float64<Millimeter> = Float64 {
        value: 10.0,
        _phantom_data: PhantomData,
    };

    let b: Float64<Millimeter> = Float64 {
        value: 10.0,
        _phantom_data: PhantomData,
    };

    let c = b * a;
    println!("c: {:?}", c);

    let value: f64 = 10.3;
    let current_distance = get_current_distance();
    println!("Size of: {:?}", std::mem::size_of_val(&value));
    println!("Size of: {:?}", std::mem::size_of_val(&current_distance));
    apply_thrust(current_distance);
}
