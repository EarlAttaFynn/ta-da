use std::io::{Write, stdin, stdout};

struct Task {
    name: String,
    desc: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut buffer: String = String::new();

    loop {
        let choice = menu(&mut buffer);
        match choice.as_str() {
            "add" => add_task(&mut tasks, &mut buffer),
            "list" => list_task(&tasks),
            "complete" => complete_task(&mut tasks, &mut buffer),
            "remove" => remove_task(&mut tasks, &mut buffer),
            "quit" => {
                break;
            }
            _ => {}
        }
    }
    println!("Thanks for using TA-DA!!! and have a great day :)");
}

fn remove_task(tasks: &mut Vec<Task>, buffer: &mut String) {
    list_task(tasks);

    if tasks.is_empty() {
        return;
    }

    println!("Which task by index would you like to remove?");
    print!("> ");
    stdout().flush().expect("Flush operation failed!");
    buffer.clear();
    stdin().read_line(buffer).expect("failed to read input");

    let choice = buffer.trim().parse::<usize>();

    match choice {
        Ok(num) => {
            if num > 0 && num <= tasks.len() {
                let removed_task = tasks.remove(num - 1);
                println!("\nSuccessfully removed task: '{}'", removed_task.name);
                list_task(tasks);
            } else {
                println!(
                    "\nInvalid index! Please enter a number from 1 to {}.",
                    tasks.len()
                );
            }
        }
        Err(_) => {
            println!("Invalid input, integer please!");
            remove_task(tasks, buffer);
        }
    }
}

fn menu(buffer: &mut String) -> String {
    buffer.clear();
    println!("TA-DA!!!\nwhat would you like to do (add, remove, list, quit?");
    print!("> ");
    stdout().flush().expect("Flush operation failed!");
    stdin().read_line(buffer).expect("Failed to read input.");

    buffer.trim().to_lowercase()
}

fn complete_task(tasks: &mut Vec<Task>, buffer: &mut String) {
    list_task(tasks);

    println!("Which task by index would you like to complete?");
    print!("> ");
    buffer.clear();
    stdin().read_line(buffer).expect("failed to read input");

    let choice = buffer.trim().parse::<usize>();

    match choice {
        Ok(num) => {
            if let Some(task) = tasks.get_mut(num - 1) {
                task.completed = true;
                list_task(tasks);
            } else {
                println!("Ya dun fucked up!!!");
                list_task(tasks);
            }
        }
        Err(_) => {
            println!("Invalid input, integer please!");
            complete_task(tasks, buffer);
        }
    }
}

// "add": Ask the user for the task description, create a new Task struct, and .push() it into your tasks vector.
fn add_task(tasks: &mut Vec<Task>, buffer: &mut String) {
    println!("What's the name of the task you'd like to add?");
    print!("> ");

    // 1. Clear the buffer to prepare it for new input
    buffer.clear();
    stdin().read_line(buffer).expect("Failed to read name");

    // 2. Trim the buffer to get a clean &str, then create an owned String
    let name = buffer.trim().to_string();

    println!("And what's the description for this task?");

    // 1. Clear the *same* buffer again
    buffer.clear();
    stdin()
        .read_line(buffer)
        .expect("Failed to read description");

    // 2. Trim and create the owned String for the description
    let desc = buffer.trim().to_string();

    // Now, name and desc are clean, owned Strings.
    let new_task = Task {
        name, // `name: name` can be shortened to `name`
        desc, // `desc: desc` can be shortened to `desc`
        completed: false,
    };

    println!(
        "TA-DA!!!\nTask '{}' has been added to your to-do list!",
        new_task.name
    );
    tasks.push(new_task);
}

/// Lists all tasks in the vector with a 1-based index.
fn list_task(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("\nYour to-do list is empty!");
        return;
    }

    println!("\n--- Your To-Do List ---");

    // 2. Use .iter().enumerate() to get (index, &Task)
    for (index, task) in tasks.iter().enumerate() {
        // 3. Print the index + 1 for a user-friendly list
        println!("\n{}. {}", index + 1, task.name);
        println!("   Description: {}", task.desc);

        // 4. Use a simple check to show a friendlier status
        let status = if task.completed { "Done" } else { "Pending" };
        println!("   Status: {}", status);
    }
    println!("\n-------------------------");
}
