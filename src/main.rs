fn main() {
    let fahrenheit = 45.0;
    let celcius = fahrenheit_to_celcius(fahrenheit);
    println!("Converting {fahrenheit}F to celcius is: {celcius}C");
}

// Formula -> (32°F − 32) × 5/9 = 0°C
fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    let celcius: f32;
    celcius = (fahrenheit - 32.0) * (5.0 / 9.0);
    println!("res {celcius}");
    celcius
}
