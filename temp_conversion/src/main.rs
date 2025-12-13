use std::io;

fn main() {
    println!("Enter temperature in Farenheit");

    let mut temperature_farenheit = String::new();

    io::stdin()
        .read_line(&mut temperature_farenheit)
        .expect("Invalid temperature");

    let temperature_farenheit: f32 = temperature_farenheit.trim().parse().expect("Invalid temperature");

    let temperature_celcius = (temperature_farenheit - 32.0) / 1.8;
    let temperature_celcius_rounded = temperature_celcius as i32;

    println!("{temperature_farenheit} degrees farenheit is {temperature_celcius_rounded} degrees celcius!");
}
