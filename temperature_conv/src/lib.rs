 pub fn fahrenheit_to_celsius(f: f64) -> f64 {

    let x: f64 = (f-32 as f64)/(9/5) as f64;
    return x

 }

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let x: f64 = (c*(9/5)as f64)+32 as f64;
    return x
}