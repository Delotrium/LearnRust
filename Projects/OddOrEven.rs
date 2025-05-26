use std::io;

fn main()
{
    let chosen_number = get_user_input();
    println!("Is number {} even? {}", &chosen_number, odd_or_even(chosen_number));
}

fn odd_or_even(n: i32) -> bool
{
    if n % 2 == 0 {
        true
    } else {
        false
    }
    // alternative (simpler) way
    // n % 2 == 0
}

fn get_user_input() -> i32 {
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Using 0 as default.");
            0
        }
    }
}
