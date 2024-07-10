use clap::{App, Arg, SubCommand};
use std::fs;
use::serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Todo {
    task: String,
    done: bool,
}

fn main() {
    let matches = App::new("CLI todo")
        .version("1.0")
        .author("IqbalAdi")
        .about("testing app")
        .subcommand(SubCommand::with_name("add")
            .about("added new task")
            .arg(Arg::with_name("TASK")
                .help("the task to add")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("oke")
            .about("complete task")
            .arg(Arg::with_name("idx")
                .help("input index start from 0 to complete task")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("list")
            .about("show list task"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(task) = matches.value_of("TASK") {
            add_task(task.to_string());
        }
    }
    
    if let Some(_) = matches.subcommand_matches("list") {
        list_task();
    }

    if let Some(matches) = matches.subcommand_matches("oke") {
        if let Some(idx) = matches.value_of("idx") {
            match idx.parse::<usize>() {
                Ok(num) => complete_task(num),
                Err(_) => println!("can't cast input")
            }
        }
    }
}

fn load_task() -> Vec<Todo> {
    let data = fs::read_to_string("tasks.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn save_task(tasks: &Vec<Todo>) {
    let data = serde_json::to_string(tasks).unwrap();
    let _ = fs::write("tasks.json", data);
}

fn add_task(task: String) {
    let mut tasks = load_task();
    tasks.push(Todo { task, done: false });
    save_task(&tasks);
}

fn list_task() {
    let tasks = load_task();
    for (i, task) in tasks.iter().enumerate() {
        println!("{}: {} [ {} ]", i + 1, task.task, if task.done {"✔️"} else { "❌" });
    }
}

fn complete_task(i: usize) {
    let mut tasks = load_task();
    tasks[i].done = true;
    save_task(&tasks);
    println!("task completed");
}

