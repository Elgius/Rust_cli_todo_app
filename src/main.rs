mod mongo_db;

fn main() {
    println!("Hello welcome to the todoifier!");
    println!("what would you like to do today?");
    println!("testing this file!");
    mongo_db::mongo_time();
}

// TODO list:
// 1 - finish all functions [done]
// 2- test all functions [done]
// 3 - integrate DB support (MongoDB)
