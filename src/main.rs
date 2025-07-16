use rand;
use inquire::{error::InquireError, Select};
use rusqlite::{Connection, Result};
use std::{io};

fn greeter() {
    let welcome_messages = ["Welcome to my totally awesome let's play. Today we'll be commiting tax fraud", ":3", "Im in your walls", "IDK, expecting something else?", "Coded by a bored teenager", "Blåhaj my beloved", "Head empty"];
    let rand_greet: usize = rand::random_range(0..=6);
    println!("{}", welcome_messages[rand_greet]);
    // Will eventually import a list of greating messages that will be printed upon startup, eventually I want to include a weighting to them so that some are more common while others are rarer. But hey it works now.
}

struct Savings {
    amount: f32,
    date: Option<Vec<u8>>
}

fn main() {
    greeter();
    
    //Handles first user selection
    let first_options: Vec<&str> = vec!["Dashboard", "Deposit", "Withdrawal"];
    /*
    Dashboard: Overview of monthly expenses and deposits along with goals and other things
    Deposit: Page for any type of income report, ie gift/cash/ etc, how you add money, ie neighbors payed you $10 to mow lawn
    Withdrawal: Any type of deduction from your account, ie you spent $5 at X and now you log it.
     */

    let page: Result<&str, InquireError> = Select::new("Select an option!", first_options).prompt();
    let choice: &str = page.expect("Failed to select option");
    println!("{choice}");

    if choice == "Deposit" {
        deposit_screen(true);
    } else if choice == "Withdrawal" {
        withdrawal_screen(true);
    } else if choice == "Dashboard" {
        dashboard_screen(true);
    } else {
        panic!("Unexpected error, no input retrieved")
    }

    fn deposit_screen(x: bool) {
    /*
    Example:
        |Deposit|

    |Savings|   |Checkings|     |Auto|
    add directly to savings or checkings account
    or automatically distro based on set goals, ie 75% to savings, 20% to checkings, 5% to fun or something,
    this is definitely going to be a lot more complicated because i need to add a way to create custome goals based off of the total amount of liquidity

    Pretty much I want to be able to load this up and the dashboard should show how close to goals I am instead of expense/income;
    It should show like:
    Savings--------------------------
    ${Running Total} / ({Goal})
        Goal 1: {Amount}    {Possible complete by date}     {followed by how much dedicated per month required to meet/current alloted} 
        Goal 2: {Amount}
        ...

        Total Needed: {Sum of All Goals}
    Expenses-----------------
    $(Running Total) / ({Budgeted Amount})                                    Like lets say that you expect/want to spend only $5K this month 
        Goal 1: {Amount Spent}/{Amount Alloted}    {Yearly/Monthly}                                  Lets say you want to cut back on how much you spend on fast food, this will show you how much you want to spend at most along with how much you have spent so far in selected time frame
    
    Debts:
        Debt 1: {Amount} {Pay back by: Y/M/D}                   Might have this be a part, track how much you owe, ie credit card purchases that havent been payed off yet or loans from a friend

    Are you on track to hitting your goals: {1-10 system based on fractions of above, ie running<budgeted=good, give motivation if good and advice if bad, idk}

    all of the above fields should be selectable, as a row, so that you can open a new tab/window/thing that allows you to see more information such as monthly payemnts towards goal or current percent allocated, etc, along with a more detailed descirption fo what it is for or what you bought/spent money on, or who you owe money to and why
     */
    if x == true {
        let conn = Connection::open("./testing.sqlite3").unwrap();
        let deposit_options: Vec<&str> = vec!["Savings", "Checking", "Auto"];
        let page: Result<&str, InquireError> = Select::new("Select an option!", deposit_options).prompt();
        let choice: &str = page.expect("Failed to select option");
        println!("{choice}");
        
        if choice == "Savings" {
            loop {
            println!("Enter amount to add");
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read line");
            let amount: f32 = match amount.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Unexpected error during processing")               
            };
            
            let test = Savings {
                amount: amount,
                date: None,
            };
            let confirm_prompt: Result<&str, InquireError> = Select::new("Is this the right amount", vec!["Yes", "No"]).prompt();
            let confirm_state: &str = confirm_prompt.expect("Failed to confirm.");
            if confirm_state == "Yes" {
                println!("Confirmed");
                let _ = conn.execute("
                    INSERT INTO savings(amount, date) VALUES(?1, ?2)",
                    (&test.amount, &test.date),
                );
                break
            } else if confirm_state == "No" {
                println!("Denied");
                continue
            }
        }
        } else if choice == "Checking" {
            loop {
            println!("Enter amount to add");
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read line");
            let amount: f32 = match amount.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Unexpected error during processing")               
            };
            
            let test = Savings {
                amount: amount,
                date: None,
            };
            let confirm_prompt: Result<&str, InquireError> = Select::new("Is this the right amount", vec!["Yes", "No"]).prompt();
            let confirm_state: &str = confirm_prompt.expect("Failed to confirm.");
            if confirm_state == "Yes" {
                println!("Confirmed");
                let _ = conn.execute("
                    INSERT INTO checking(amount, date) VALUES(?1, ?2)",
                    (&test.amount, &test.date),
                );
                break
            } else if confirm_state == "No" {
                println!("Denied");
                continue
            }
        }
        } else if choice == "Auto" {
            println!("This is a problem for tommorow me...")
        } else {
            panic!("Unexpected error, no input retrieved.")
        }
        }
    }
    fn withdrawal_screen(x: bool) {
        /*
        should have at least two options,
        paid which will subtract from liquidity
        and unpaid which will add to debt

        for example:
            Withdrawal

            |USER INPUT| needs to validate input as f64, return error on non-f64/more than 2 decimals

            |PAID|  |UNPAID| bool t\f

            |CONFIRM|  |CANCEL| for confirm requires both input and P\UP to be valid, 
            for example, (230.94 PAID) this could be logged in sql in the format of DATE | AMOUNT | STATUS
            that way you can see when you purchased something and for what amount
            i might be able to learn more sql and do something on the db side to then check if the status is paid to then subtract that amount from an account
            speaking of i need to add a system for multiple accounts, ie credit/debit/cash

            I can create a trigger when inserting into the withdrawal table to then perf a math op in balance table based on bool state

        */
        if x == true {
            println!("W");
        }
    }

    fn dashboard_screen(x: bool) {
        if x == true {
            println!("-----DASHBOARD-----");
        }
    }
}   