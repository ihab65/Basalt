use std::fs;

const TODO: &str = "- [ ]";
const DONE: &str = "- [x]";
const TAB:  &str = "\t";

fn parse (contents: String) {
    let todos: Vec<&str> = contents.lines()
        .filter(|s| s.starts_with(TODO))
        .collect();
    
    let sub_tasks: Vec<&str> = contents.lines()
        .filter(|s| s.starts_with(&(TAB.to_owned() + TODO)))
        .collect();

    let dones: Vec<&str> = contents.lines()
        .filter(|s| s.starts_with(DONE))
        .collect();


    println!("tasks:");
    for todo in todos {
        println!("\t{}", todo)    
    }

    println!("done :");
    for done in dones {
        println!("\t{}", done)
    }
}

fn main() {
    let path: &str = "../../Documents/obsidian.main/To-do.md";
    let tasks: Result<String, std::io::Error> = fs::read_to_string(path);

    parse(tasks.unwrap())
}
