const FARENHEIT:f32 = 32.0;


fn farenheit_to_celsius(f:f64) -> f64 {
    let far = FARENHEIT as f64;
    let result = (f -far)* 5.0/9.0;
    return result;
}


fn celsius_to_fahrenheit(c: f64) -> f64 {
    let far = FARENHEIT as f64;
    let result = (c * 9.0/5.0) + far;
    return result;
}

fn main() {
    let x = farenheit_to_celsius(104.0);
    let y = celsius_to_fahrenheit(0.0);
    println!("{}", x);
    println!("{}", y);

    // Add the loop for the next 5 integer temperatures
    let mut start_temp = 32.0;
    println!("\nConverting the next 5 integer temperatures starting from {}°F:", start_temp);
    for _ in 0..5 {
        let celsius = farenheit_to_celsius(start_temp);
        println!("{:.1}°F is equal to {:.2}°C", start_temp, celsius);
        start_temp += 1.0;
    }
}