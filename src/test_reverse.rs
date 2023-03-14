use std::io;

fn main() {
    // Read input string from the user
    let mut input = String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove any trailing whitespace or newline characters
    input = input.trim().to_string();

    // Reverse the input string and print it
    let reversed = input.chars().rev().collect::<String>();
    println!("Reversed string: {}", reversed);
}