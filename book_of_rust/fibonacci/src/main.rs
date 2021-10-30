use std::io;

fn main() {
    let n = get_int_input();
    if n == 1 || n == 2 {
        println!("Fib({}) is: {}", n, n - 1);
    } else {
        let mut seq: [u128; 2] = [0, 1];

        for _ in 3..n+1 {
            seq[1] = seq[0] + seq[1];
            seq[0] = seq[1] - seq[0];
        }
        println!("\nFib({}) is: {}\n", n, seq[1]);
    }    
}

// repeatadly calls read_input() until an int value is given
fn get_int_input() -> u128 {
    let mut int_input: String;
    loop {
        println!("Enter n value:");
        int_input = read_input();
        let int_input: u128 = match int_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nNot a valid entry, must be a posisitive integer!\n");
                continue
            },
        }; 
        return int_input
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line.");
    input
}
