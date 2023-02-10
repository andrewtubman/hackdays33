use std::io;

fn main() {
    println!("Convert form Fahrenheit to Celsius");
    println!("Enter a temperature in F:");
    
    let mut temperature_f = String::new();

    io::stdin()
        .read_line(&mut temperature_f)
        .expect("Failed to read line");

    let temperature_f: f32 = match temperature_f.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let temperature_c: f32 = (temperature_f - 32.0) * 5.0/9.0;

    println!("{temperature_f:.2} F is {temperature_c:.2} C")
}
