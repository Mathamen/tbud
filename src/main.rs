use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

const FILE: &str = "commands.txt";

fn save_command(name: String) {
    println!("Enter your command between '::start' and '::end':");

    let stdin = io::stdin();
    let mut lines = Vec::new();
    let mut capture = false;

    for line in stdin.lock().lines().flatten() {
        let trimmed = line.trim();
        if trimmed == "::start" {
            capture = true;
            continue;
        }
        if trimmed == "::end" {
            break;
        }
        if capture {
            lines.push(line);
        }
    }

    let content = lines.join("\n");

    if content.trim().is_empty() {
        println!("Command is empty. Nothing saved.");
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE)
        .expect("Couldn't open file");

    writeln!(file, "{}:::{}", name, content.replace("\r", "")).expect("Failed to write");
    println!("Saved '{}'", name);
}

fn list_commands() {
    if let Ok(file) = OpenOptions::new().read(true).open(FILE) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some((name, _)) = line.split_once(":::") {
                println!("{}", name);
            }
        }
    } else {
        println!("No saved commands.");
    }
}

fn delete_command(name: String) {
    if let Ok(file) = OpenOptions::new().read(true).open(FILE) {
        let lines: Vec<String> = BufReader::new(file)
            .lines()
            .flatten()
            .filter(|line| !line.starts_with(&(name.clone() + ":::")))
            .collect();

        let mut file = fs::File::create(FILE).expect("Failed to recreate file");
        for line in lines {
            writeln!(file, "{}", line).expect("Write failed");
        }

        println!("Deleted '{}'", name);
    } else {
        println!("Nothing to delete.");
    }
}

fn run_command(name: String) {
    if let Ok(file) = OpenOptions::new().read(true).open(FILE) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some((n, c)) = line.split_once(":::") {
                if n == name {
                    println!("Running '{}':", name);
                    let output = Command::new("cmd")
                        .args(&["/C", &c])
                        .output()
                        .expect("Failed to run");

                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                    return;
                }
            }
        }
        println!("Command '{}' not found", name);
    } else {
        println!("No commands file found.");
    }
}

fn print_help() {
    println!(
        "Usage:
  tbot add <name>     | a <name>
  tbot list           | ls
  tbot delete <name>  | d <name>
  tbot run <name>     | r <name>"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" | "a" => {
            if args.len() < 3 {
                println!("Usage: tbot add <name>");
            } else {
                save_command(args[2].clone());
            }
        }
        "list" | "ls" => list_commands(),
        "delete" | "d" => {
            if args.len() < 3 {
                println!("Usage: tbot delete <name>");
            } else {
                delete_command(args[2].clone());
            }
        }
        "run" | "r" => {
            if args.len() < 3 {
                println!("Usage: tbot run <name>");
            } else {
                run_command(args[2].clone());
            }
        }
        _ => print_help(),
    }
}
