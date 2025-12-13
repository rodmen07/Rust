use std::io;

// Create a function that generates the nth fibonacci number.
// The nth fibonacci number is given by (n-1) + (n-2) for n > 2.
fn main() {
    println!("Please enter a positive integer, n.");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Invalid user input");

    let user_input: i32 = user_input
        .trim()
        .parse()
        .expect("Did not enter a positive integer!");

    let mut counter = 2;

    let mut nth_output = 3;
    let mut nth_minus_one = 2;
    let mut nth_minus_two = 1;

    if user_input <= 0 {
        println!("Did not enter a positive integer!");
    } else if user_input == 1 {
        println!("The nth fibonacci for n = 1 is {nth_minus_two}!");
    } else if user_input == 2 {
        println!("The nth fibonacci for n = 2 is {nth_minus_one}!");
    } else {
        while counter < user_input {
            nth_output = nth_minus_one + nth_minus_two;
            nth_minus_two = nth_minus_one;
            nth_minus_one = nth_output;

            counter += 1;
            println!("{nth_output}, {nth_minus_one}, {nth_minus_two}, {counter}");
        }
        println!("The nth fibonacci for n = {user_input} is {nth_output}!")
    }
}
