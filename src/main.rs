use rand;
use inquire::{error::InquireError, Select};
use sqlite;


fn greeter() {
    let welcome_messages = ["Welcome to my totally awesome let's play. Today we'll be commiting tax fraud", ":3", "Im in your walls", "IDK, expecting something else?", "Coded by a bored teenager", "Blåhaj my beloved", "Head empty"];
    let rand_greet: usize = rand::random_range(0..=6);
    println!("{}", welcome_messages[rand_greet]);
    // Will eventually import a list of greating messages that will be printed upon startup, eventually I want to include a weighting to them so that some are more common while others are rarer. But hey it works now.
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
        panic!("Unexpected error, no input retrieved)")
    }

    fn deposit_screen(x: bool) {
    if x == true {
        println!("D");
        }
    }
    fn withdrawal_screen(x: bool) {
        if x == true {
            println!("W");
        }
    }

    fn dashboard_screen(x: bool) {
        if x == true {
            println!("-----DASHBOARD-----");
            let connection = sqlite::open("./testing.sqlite3").unwrap();
            let query = "SELECT * FROM withdrawals ORDER BY day DESC LIMIT 15";
            println!("Recent Expeneses:");
            for row in connection
                .prepare(query)
                .unwrap()
                .into_iter()
                .map(|row: Result<sqlite::Row, sqlite::Error>| row.unwrap())
                {
                    println!("Date: {}: ${}", row.read::<&str, _>("day"), row.read::<i64, _>("amount"));
                }
                //find someway to limit to 15 most recent expenses
        }
    }
}   