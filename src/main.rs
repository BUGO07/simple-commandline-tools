use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

mod utils;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u64,
    time: u64,
    text: String,
    done: bool,
}

fn main() {
    let matches = Command::new("std")
        .version("1.0")
        .about("Simple Commandline Tools")
        .subcommand(
            Command::new("todo")
                .about("Todo list manager")
                .subcommand(
                    Command::new("add")
                        .about("Add a new todo")
                        .short_flag('a')
                        .arg(
                            Arg::new("text")
                                .required(true)
                                .num_args(1..)
                                .help("Text of the todo"),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                        .alias("rm")
                        .alias("delete")
                        .short_flag('r')
                        .about("Remove a todo")
                        .arg(Arg::new("id").required(true).help("ID of the todo")),
                )
                .subcommand(
                    Command::new("list")
                        .alias("ls")
                        .short_flag('l')
                        .about("List all todos"),
                )
                .subcommand(
                    Command::new("done")
                        .short_flag('d')
                        .about("Mark a todo as done")
                        .arg(Arg::new("id").required(true).help("ID of the todo")),
                )
                .subcommand(
                    Command::new("undone")
                        .short_flag('u')
                        .about("Unmark a todo as done")
                        .arg(Arg::new("id").required(true).help("ID of the todo")),
                )
                .subcommand(Command::new("organize").about("Organize the IDs of the todo list")),
        )
        .subcommand(
            Command::new("quadratic")
                .alias("qes")
                .about("Solves quadratic equations")
                .arg(
                    Arg::new("a")
                        .required(true)
                        .allow_negative_numbers(true)
                        .help("The coefficient of x^2"),
                )
                .arg(
                    Arg::new("b")
                        .required(true)
                        .allow_negative_numbers(true)
                        .help("The coefficient of x"),
                )
                .arg(
                    Arg::new("c")
                        .required(true)
                        .allow_negative_numbers(true)
                        .help("The constant term"),
                ),
        )
        .subcommand(
            Command::new("palindrome")
                .about("Checks if a string is a palindrome")
                .arg(
                    Arg::new("text")
                        .required(true)
                        .num_args(1..)
                        .help("Text to check"),
                ),
        )
        .subcommand(
            Command::new("reverse").about("Reverses a string").arg(
                Arg::new("text")
                    .required(true)
                    .num_args(1..)
                    .help("Text to reverse"),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("reverse", args)) => {
            let text: String = args
                .get_many::<String>("text")
                .unwrap()
                .cloned()
                .collect::<Vec<_>>()
                .join(" ");
            let reversed: String = text.chars().rev().collect();
            println!("{}", reversed);
        }
        Some(("palindrome", args)) => {
            let text: String = args
                .get_many::<String>("text")
                .unwrap()
                .cloned()
                .collect::<Vec<_>>()
                .join(" ");
            let is_palindrome = text.chars().eq(text.chars().rev());
            println!("{}", if is_palindrome { "Yes" } else { "No" });
        }
        Some(("quadratic", args)) => {
            let a = args.get_one::<String>("a").unwrap().parse().unwrap();
            let b = args.get_one::<String>("b").unwrap().parse().unwrap();
            let c = args.get_one::<String>("c").unwrap().parse().unwrap();
            let (d, x1, x2) = utils::qes(a, b, c);
            println!(
                "{}x^2 {} {}x {} {} = 0",
                a.abs(),
                utils::pos_string(b),
                b.abs(),
                utils::pos_string(c),
                c.abs()
            );
            println!("Discriminant: {}", d);
            println!("Roots: {}, {}", x1, x2);
        }
        Some(("todo", command)) => match command.subcommand() {
            Some(("add", args)) => {
                let text: String = args
                    .get_many::<String>("text")
                    .unwrap()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");
                let mut todos = match utils::read_from_json(&utils::get_todos()) {
                    Ok(todos) => todos,
                    Err(_) => {
                        if std::fs::exists(utils::get_todos()).unwrap_or(false) {
                            eprintln!(
                                "Failed to read the todos file, creating a new one and renaming the old one for safety."
                            );
                            std::fs::rename(
                                utils::get_todos(),
                                dirs::data_dir().unwrap().join("todos.json.bak"),
                            )
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to rename the file: {}", e);
                            });
                        } else {
                            utils::write_to_json(&utils::get_todos(), Vec::<Todo>::new()).unwrap();
                        }
                        Vec::new()
                    }
                };

                todos.sort_by(|a, b| a.id.cmp(&b.id));

                let id = if todos.is_empty() {
                    1
                } else {
                    todos[todos.len() - 1].id + 1
                };

                todos.push(Todo {
                    id,
                    time: SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs(),
                    text: text.clone(),
                    done: false,
                });
                match utils::write_to_json(&utils::get_todos(), todos) {
                    Ok(_) => println!("Added todo: {}", text),
                    Err(e) => eprintln!("Error writing file: {}", e),
                }
            }
            Some(("remove", args)) => {
                let index: usize = args.get_one::<String>("id").unwrap().parse().unwrap();
                let mut todos = match utils::read_from_json(&utils::get_todos()) {
                    Ok(todos) => todos,
                    Err(_) => {
                        eprintln!("Failed to read the todos file");
                        Vec::new()
                    }
                };
                if index > 0 {
                    let mut name = String::new();
                    if let Some(pos) = todos.iter().position(|todo| todo.id == index as u64) {
                        name = todos[pos].text.clone();
                        todos.remove(pos);
                    }
                    match utils::write_to_json(
                        dirs::data_dir()
                            .unwrap()
                            .join("todos.json")
                            .to_str()
                            .unwrap(),
                        todos,
                    ) {
                        Ok(_) => println!("Removed \"{}\" from the todo list", name),
                        Err(e) => eprintln!("Error writing file: {}", e),
                    }
                } else {
                    eprintln!("Invalid index");
                }
            }
            Some(("list", _)) => match utils::read_from_json(&utils::get_todos()) {
                Ok(mut todos) => {
                    if todos.is_empty() {
                        println!("You have no todos yet.");
                        return;
                    }
                    println!("Todo list:");
                    todos.sort_by(|a, b| b.done.cmp(&a.done));
                    for todo in todos.iter() {
                        println!(
                            "{}. [{}] {}",
                            todo.id,
                            if todo.done { "X" } else { " " },
                            todo.text
                        );
                    }
                }
                Err(_) => {
                    if utils::exists(
                        dirs::data_dir()
                            .unwrap()
                            .join("todos.json")
                            .to_str()
                            .unwrap(),
                    ) {
                        eprintln!("Failed to read the todos file");
                    } else {
                        println!("You have no todos yet.");
                    }
                }
            },
            Some(("done", args)) => {
                let index: usize = args.get_one::<String>("id").unwrap().parse().unwrap();
                let mut todos = match utils::read_from_json(&utils::get_todos()) {
                    Ok(todos) => todos,
                    Err(_) => {
                        eprintln!("Failed to read the todos file");
                        Vec::new()
                    }
                };
                if index > 0 {
                    let mut name = String::new();
                    for todo in &mut todos {
                        if todo.id == index as u64 {
                            name = todo.text.clone();
                            todo.done = true;
                            break;
                        }
                    }
                    match utils::write_to_json(
                        dirs::data_dir()
                            .unwrap()
                            .join("todos.json")
                            .to_str()
                            .unwrap(),
                        todos,
                    ) {
                        Ok(_) => println!("Marked \"{}\" as done", name),
                        Err(e) => eprintln!("Error writing file: {}", e),
                    }
                } else {
                    eprintln!("Invalid index");
                }
            }
            Some(("undone", args)) => {
                let index: usize = args.get_one::<String>("id").unwrap().parse().unwrap();
                let mut todos = match utils::read_from_json(&utils::get_todos()) {
                    Ok(todos) => todos,
                    Err(_) => {
                        eprintln!("Failed to read the todos file");
                        Vec::new()
                    }
                };
                if index > 0 && index <= todos.len() {
                    todos[index - 1].done = false;
                    match utils::write_to_json(
                        dirs::data_dir()
                            .unwrap()
                            .join("todos.json")
                            .to_str()
                            .unwrap(),
                        todos,
                    ) {
                        Ok(_) => println!("Marked todo as undone"),
                        Err(e) => eprintln!("Error writing file: {}", e),
                    }
                } else {
                    eprintln!("Invalid index");
                }
            }
            Some(("organize", _)) => {
                let mut todos = match utils::read_from_json(&utils::get_todos()) {
                    Ok(todos) => todos,
                    Err(_) => {
                        eprintln!("Failed to read the todos file");
                        Vec::new()
                    }
                };
                for (i, todo) in todos.iter_mut().enumerate() {
                    todo.id = i as u64 + 1;
                }
                match utils::write_to_json(&utils::get_todos(), todos) {
                    Ok(_) => println!("Organized the todo list"),
                    Err(e) => eprintln!("Error writing file: {}", e),
                }
            }
            _ => eprintln!("Invalid todo command"),
        },
        _ => eprintln!("Invalid command"),
    }
}
