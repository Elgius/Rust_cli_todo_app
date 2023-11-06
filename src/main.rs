use colored::Colorize;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

fn completed_logs() {
    println!("this is a incomplete function");
}

fn delete_items() {
    println!("this is a incomplete function");
}

fn add_items() {
    println!("welcome to the add items menu!");
    let welcome_message = "please add what you want to do today!:   ".blue();
    println!("{}", welcome_message);
    let mut items: String = String::new();
    io::stdin().read_line(&mut items).expect("value missing");
    // file appending function that creates a new file if file is missing. otherwise it add the values directly into txt file
    // works more efficiently than i thought, pretty cool
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.txt")
        .expect("file missing");

    file.write_all(items.as_bytes())
        .expect("error occured at the appending stage!");
    println!("your new tasks have been appended successfully");

    println!("do you want to see the new changes? [y,n]");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("error in getting choices");

    // allows the user to go back to the main menu and check the progress
    // BUG: 'y' and 'n' dont work, so it always go to default value
    match choice.as_str() {
        "y" => {
            view_items();
        },
        "n" => {
            menu_list();
        },
        _ => {
            println!("invalid choice, sending you back to the main menu!");
            menu_list();
        }
    }
}
fn view_items() {
    let info = "this is your ongoing items at the moment".blue();
    println!("{}", info);

    let content: String =
        fs::read_to_string("todo.txt").expect("File is missing from the directory!");

    println!("{content}");
}

fn menu_list() {
    println!("1. View ongoing stuff");
    println!("2. Add ongoing stuff");
    println!("3. Delete ongoing stuff");
    println!("4. Check completed logs!");
    println!("5. End the program");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("There was an issue when trying to get the choice info from user!");

    let choice: char = choice
        .trim()
        .parse()
        .expect("missing value from previous function!");

    match choice {
        '1' => view_items(),
        '2' => add_items(),
        '3' => delete_items(),
        '4' => completed_logs(),
        '5' => println!("thanks for using the system!"),
        _ => println!("you have entered an invalid number, please enter the correct number"),
    }
}

fn main() {
    println!("Hello welcome to the todoifier!");
    println!("what would you like to do today?");
    menu_list();
}

// TODO list:
// 1 - finish all functions
// 2- test all functions
// 3 - integrate DB support (MongoDB)
