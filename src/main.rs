use inquire::{InquireError, Select};

fn main() {
    println!("Welcome to my totally awesome let's play! On todays episode we will be committing tax fraud! /j");

    let v1i: Vec<&str>  = vec!["Add", "Withdraw", "Check"];
    let v1o: Result<&str, InquireError> = Select::new("Please select what you want to do!", v1i).prompt();
    //find some way to remove inquireerror from v1o PLEASE FUTURE ME, in like two minutes that is....... mjaybe i just find other way to handle input 

    //println!("test: {choice}");
}   