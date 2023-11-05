use std::io;
use std::fs;
use colored::Colorize;

fn completed_logs() {
    println!("this is a incomplete function");
}

fn delete_items() {
    println!("this is a incomplete function");
}

fn add_items() {
    println!("this is a incomplete function");
}

fn view_items() {

    let info = "this is your ongoing items at the moment".blue();
    println!("{}", info);
    
    let content: String = fs::read_to_string("todo.txt")
    .expect("File is missing from the directory!");

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

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use std::process::Command;
//   use std::str;

//   #[test]
//   fn test_menu_list() {
//       // Run the program with '1' as the input
//       let output = Command::new("cargo")
//           .arg("run")
//           .arg("--")
//           .arg("1")
//           .output()
//           .expect("Failed to execute command");

//       // Check the output
//       let output_str = str::from_utf8(&output.stdout).unwrap();
//       assert!(output_str.contains("1. View ongoing stuff"));
//       assert!(output_str.contains("2. Add ongoing stuff"));
//       assert!(output_str.contains("3. Delete ongoing stuff"));
//       assert!(output_str.contains("4. Check completed logs!"));
//       assert!(output_str.contains("5. End the program"));
//       assert!(output_str.contains("this is a incomplete function"));
//   }
// }
