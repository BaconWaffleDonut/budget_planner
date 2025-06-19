use inquire::{InquireError, Select};
fn main() {
    println!("Welcome to my totally awesome let's play! On todays episode we will be committing tax fraud! /j");

    let v1i: Vec<&str>  = vec!["Add", "Withdraw", "Check"];
    let v1o: Result<&str, InquireError> = Select::new("Please select what you want to do!", v1i).prompt();
    match v1o {
        Ok(choice) => println!("Transferring you to {choice}"),
        Err(_) => println!("There was an error, please try again."),
    }
}   