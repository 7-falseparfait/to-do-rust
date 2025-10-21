use std::io;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input
}

fn main() {
    println!("Welcome to the Rust To-Do Manager!");
    println!("----------------------------------");
    let mut tasks: Vec<String> = Vec::new();
    loop {
        let command_input = get_input("Enter command (add, view, remove, quit): ");

        let command_trimmed = command_input.trim();

        if command_trimmed == "quit" {
            println!("Existing To-do manager. Goodbye");
            break;
        } else if command_trimmed.is_empty() {
            continue;
        } else if command_trimmed.starts_with("add ") {
            let task_description = command_trimmed.trim_start_matches("add ").trim();
            let new_task = task_description.to_string();
            tasks.push(new_task);
            println!("Task added: \"{}\"", task_description);
        } else if command_trimmed == "view" {
            if tasks.is_empty() {
                println!("Your to-do list is empty")
            } else {
                println!("Your to-do list {}", tasks.len());
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}:{}", i + 1, task)
                }
                println!("----------------------------------\n");
            }
        } else if command_trimmed.starts_with("remove ") {
            let index_str = command_trimmed.trim_start_matches("remove ").trim();
            match index_str.parse::<usize>() {
                Ok(index) => {
                    let actual_index = index - 1;
                    if actual_index < tasks.len() {
                        let removed_task = tasks.remove(actual_index);
                        println!("Removed task {}: \"{}\"", index, removed_task);
                    }
                }
                Err(_) => {
                    println!("Error: Index is out of bounds")
                }
            }
        } else {
            println!(
                "Command '{}' not recognized. Try 'add', 'view', 'remove <index>', or 'quit'.",
                command_trimmed
            );
        }
    }
}
