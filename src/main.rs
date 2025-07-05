use rand;

fn greeter() {
    let welcome_messages = ["Welcome to my totally awesome let's play. Today we'll be commiting tax fraud", ":3", "Im in your walls", "IDK, expecting something else?", "Coded by a bored teenager", "Blåhaj my beloved", "Head empty"];
    let rand_greet: usize = rand::random_range(0..=6);
    println!("{}", welcome_messages[rand_greet]);
    // Will eventually import a list of greating messages that will be printed upon startup, eventually I want to include a weighting to them so that some are more common while others are rarer. But hey it works now.
}

fn main() {
    greeter();
}   