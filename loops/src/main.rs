use std::io;

fn main() {
    println!("Enter number:");

    let input = get_int_input();

    println!("Input: {}", input);
}

fn get_int_input() -> u32 {
    let mut int_input: String;
    loop {
        println!("Enter n value:");

        int_input = read_input();

        let int_input: u32 = match int_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
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
