mod task_manager;


fn main() {
    println!("Hello welcome to the todoifier!");
    println!("what would you like to do today?");
    task_manager::menu_list();
}

// TODO list:
// 1 - finish all functions [done]
// 2- test all functions [done]
// 3 - integrate DB support (MongoDB)
