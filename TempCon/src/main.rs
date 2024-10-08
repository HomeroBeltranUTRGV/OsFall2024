const FAHRENHEIT: f32 = 32.0; // Freezing point of water in Fahrenheit

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let result = (f - FAHRENHEIT as f64) * 5.0 / 9.0;
    return result;
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let result = c * 9.0 / 5.0 + FAHRENHEIT as f64;
    return result;
}

fn main() {
    // Declare a mutable variable with a starting temperature in Fahrenheit
    let mut fahrenheit_temp: f64 = 32.0;
    
    // Convert the starting temperature to Celsius and print the result
    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{:.1}°F is {:.1}°C", fahrenheit_temp, celsius_temp);

    // Loop to convert and print the next 5 integer temperatures
    for _ in 0..5 {
        fahrenheit_temp += 1.0; // Increment Fahrenheit temperature
        let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
        println!("{:.1}°F is {:.1}°C", fahrenheit_temp, celsius_temp);
    }
}
