use std::io;

enum Command {
    Add(String),
    View,
    Remove(usize),
    Quit,
    Invalid,
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input
}

fn parse_command(input: &str) -> Command {
    let trimmed_input = input.trim();
    if trimmed_input.starts_with("add ") {
        let task = trimmed_input.trim_start_matches("add ").trim();
        if task.is_empty() {
            Command::Invalid
        } else {
            Command::Add(task.to_string())
        }
    } else if trimmed_input.starts_with("remove ") {
        let index_str = trimmed_input.trim_start_matches("remove ").trim();
        match index_str.parse::<usize>() {
            Ok(index) => Command::Remove(index),
            Err(_) => Command::Invalid,
        }
    } else if trimmed_input == "view" {
        Command::View
    } else if trimmed_input == "quit" {
        Command::Quit
    } else {
        Command::Invalid
    }
}

fn main() {
    println!("Welcome to the Rust To-Do Manager!");
    println!("----------------------------------");
    let mut tasks: Vec<String> = Vec::new();
    loop {
        let user_input: String = get_input("Enter command (add, view, remove, quit): "); // returned an owned String
        let command = parse_command(&user_input); // Returns a Command Type 

        match command {
            Command::Add(task_description) => {
                println!("Task added: \"{}\"", task_description);
                tasks.push(task_description);
            }
            Command::View => {
                if tasks.is_empty() {
                    println!("Your to-do list is empty.")
                } else {
                    println!("Your to-do list ({} tasks):", tasks.len());
                    for (i, task) in tasks.iter().enumerate() {
                        println!("{}: {}", i + 1, task);
                    }
                    println!("----------------------------------\n");
                }
            }
            Command::Remove(index) => {
                if index > 0 && index <= tasks.len() {
                    let removed_task = tasks.remove(index - 1);
                    println!("Removed task {}: \"{}\"", index, removed_task);
                } else {
                    println!("Error: Index {} is out of bounds.", index);
                }
            }
            Command::Quit => {
                println!("Exiting To-do manager. Goodbye!");
                break;
            }
            Command::Invalid => {
                println!(
                    "Command not recognized. Try 'add <task>', 'view', 'remove <index>', or 'quit'."
                );
            }
        }
    }
}
