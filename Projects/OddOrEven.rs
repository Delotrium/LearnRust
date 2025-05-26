use std::io;

fn main()
{
    let chosen_number = get_user_input();
    println!("Is number {} even? {}", &chosen_number, odd_or_even(&chosen_number));
}

fn odd_or_even(n: &i32) -> bool
{
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

fn get_user_input() -> i32
{
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Please enter a valid number")
}
