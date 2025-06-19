use std::io;
fn main() {
    println!("Enter a positive integer");

    let mut fib = String::new();

    io::stdin()
        .read_line(&mut fib)
        .expect("Failed to read line");

    let fib: u32 = match fib.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input format"),
    };
    
    println!("Fibonacci value for : {} is {}", fib, fibonacci_calculator(fib));
}

fn fibonacci_calculator(fib_num:u32) -> u32{
    if fib_num == 0{
        0
    } else if fib_num == 1 {
        1
    } else {
        fibonacci_calculator(fib_num - 1) + fibonacci_calculator(fib_num -2)
    }
}
