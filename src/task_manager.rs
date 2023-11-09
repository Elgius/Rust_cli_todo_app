    use colored::Colorize;
    use std::fs;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::BufWriter;
    use std::io::Read;
    use std::io::Write;

pub fn completed_logs() {
    let notice = "welcome to the completed logs menu".blue();
    println!("{}", notice);

    let mut task = String::new();

    println!("please add the task that you have completed:  ");

    io::stdin()
        .read_line(&mut task)
        .expect("there was an issue at input stage");

    let mut file = match File::open("completed.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("error in opening the file: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error reading the file: {}", error);
            return;
        }
    };

    if contents.contains(&task) {
        println!("task is found");
    } else {
        println!("task is not found");
    }
}

pub fn delete_items() {
    let deletor = "cross tasks off the list?".blue();
    println!("{}", deletor);

    println!("what task have you completed that is required to be taken out?");

    let mut task = String::new();

    io::stdin()
        .read_line(&mut task)
        .expect("there has been an error in taking inputs");

    {
        let mut file = File::open("todo.txt").unwrap();
        let mut out_file = File::open("file.txt.temp").unwrap();

        let reader = BufReader::new(&file);
        let mut writer = BufWriter::new(&out_file);

        for (index, line) in reader.lines().enumerate() {
            let line = line.as_ref().unwrap();
            if !line.contains(&task) {
                writeln!(writer, "{}", line);
            }
        }
        writer.flush().unwrap();
        fs::rename("file.txt.temp", "todo.txt").unwrap();
    }
}

pub fn add_items() {
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
        }
        "n" => {
            menu_list();
        }
        _ => {
            println!("invalid choice, sending you back to the main menu!");
            menu_list();
        }
    }
}
pub fn view_items() {
    let info = "this is your ongoing items at the moment".blue();
    println!("{}", info);

    let content: String =
        fs::read_to_string("todo.txt").expect("File is missing from the directory!");

    println!("{content}");
}

pub fn menu_list() {
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
