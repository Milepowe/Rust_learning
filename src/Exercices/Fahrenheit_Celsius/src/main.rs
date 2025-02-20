use std::io;

fn main() {
    let mut fahrenheit = String::new();

    println!("Please input your fahrenheit: ");
    io::stdin()
        .read_line(&mut  fahrenheit)
        .expect("Error reading from stdin");

    let fahreinet: f32 = match fahrenheit.trim().parse() {
        Ok(fahrenheit) => fahrenheit,
        Err(_) => panic!("Error parsing fahreinet number"),
    };
    println!("{} fahrenheit is equal to  {} celsisus", fahreinet, fahrenheit_to_celsius(fahreinet));
}
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32
{
    (fahrenheit - 32.0) * 5.0/9.0
}
