use rand;
use inquire::{error::InquireError, Select};

fn greeter() {
    let welcome_messages = ["Welcome to my totally awesome let's play. Today we'll be commiting tax fraud", ":3", "Im in your walls", "IDK, expecting something else?", "Coded by a bored teenager", "Blåhaj my beloved", "Head empty"];
    let rand_greet: usize = rand::random_range(0..=6);
    println!("{}", welcome_messages[rand_greet]);
    // Will eventually import a list of greating messages that will be printed upon startup, eventually I want to include a weighting to them so that some are more common while others are rarer. But hey it works now.
}

fn first_options() {
    let first_options: Vec<&str> = vec!["Dashboard", "Deposit", "Withdrawal"];
    /*
    Dashboard: Overview of monthly expenses and deposits along with goals and other things
    Deposit: Page for any type of income report, ie gift/cash/ etc, how you add money, ie neighbors payed you $10 to mow lawn
    Withdrawal: Any type of deduction from your account, ie you spent $5 at X and now you log it.
     */

    let page: Result<&str, InquireError> = Select::new("Select an option!", first_options).prompt();
    match page {
        Ok(page) => print!("No error detected, choice was: {page}"),
        Err(err) => println!("An error occured.
        Exit code: {}", err)
    }
}

fn main() {
    greeter();
    first_options();
    
}   