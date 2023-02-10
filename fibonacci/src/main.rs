// use std::io;

fn main() {
    println!("Find the nth fibonacci number");
    println!("Enter a number:");
    
    // let mut number = String::new();

    // io::stdin()
    //     .read_line(&mut number)
    //     .expect("Failed to read line");

    // let number: i128 = match number.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => return,
    // };

    let number = 55;

    let fib = fibonacci(number);

    println!("The {number} fibonacci number is {fib}")
}

fn fibonacci(number: i128) -> i128 {
    if number < 2 {
        return number;
    }

    let mut i = 0;
    let mut f = 0;
    let mut l = 1;
    while i < number {
        i += 1;
        f = f + l;
        l = f - l;
    }

    f
}

