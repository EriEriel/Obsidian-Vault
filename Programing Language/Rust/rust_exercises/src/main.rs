use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Company Directory. Commands:");
    println!("  add [Name] to [Dept]");
    println!("  remove [Name] from [Dept]");
    println!("  list [Dept] | list all");
    println!("  depts | quit\n");

    loop {
        print!("➜ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match parse_command(input) {
            Command::Add { name, dept } => {
                let employees = company.entry(dept.clone()).or_insert_with(Vec::new);
                if employees.contains(&name) {
                    println!("error: {} is already in {}.", name, dept);
                } else {
                    employees.push(name.clone());
                    println!("ok: added {} to {}.", name, dept);
                }
            }
            Command::Remove { name, dept } => {
                match company.get_mut(&dept) {
                    None => println!("error: department \"{}\" does not exist.", dept),
                    Some(employees) => {
                        if let Some(pos) = employees.iter().position(|e| e == &name) {
                            employees.remove(pos);
                            if employees.is_empty() {
                                company.remove(&dept);
                            }
                            println!("ok: removed {} from {}.", name, dept);
                        } else {
                            println!("error: {} not found in {}.", name, dept);
                        }
                    }
                }
            }
            Command::ListDept(dept) => {
                match company.get(&dept) {
                    None => println!("error: department \"{}\" not found.", dept),
                    Some(employees) => {
                        let mut sorted = employees.clone();
                        sorted.sort();
                        println!("\n[ {} ]", dept);
                        for name in &sorted {
                            println!("  {}", name);
                        }
                        println!();
                    }
                }
            }
            Command::ListAll => {
                if company.is_empty() {
                    println!("no employees yet.");
                } else {
                    let mut depts: Vec<&String> = company.keys().collect();
                    depts.sort();
                    println!();
                    for dept in depts {
                        let mut sorted = company[dept].clone();
                        sorted.sort();
                        println!("[ {} ]", dept);
                        for name in &sorted {
                            println!("  {}", name);
                        }
                        println!();
                    }
                }
            }
            Command::Depts => {
                let mut depts: Vec<&String> = company.keys().collect();
                depts.sort();
                if depts.is_empty() {
                    println!("no departments yet.");
                } else {
                    for dept in depts {
                        println!("  {} ({} employees)", dept, company[dept].len());
                    }
                }
            }
            Command::Quit => break,
            Command::Unknown => println!("unknown command. try: add [Name] to [Dept]"),
        }
    }
}

enum Command {
    Add { name: String, dept: String },
    Remove { name: String, dept: String },
    ListDept(String),
    ListAll,
    Depts,
    Quit,
    Unknown,
}

fn parse_command(input: &str) -> Command {
    let lower = input.to_lowercase();

    if let Some(rest) = lower.strip_prefix("add ") {
        if let Some((name_part, dept_part)) = rest.split_once(" to ") {
            let name = name_part.trim().to_string();
            let dept = dept_part.trim().to_string();
            // Preserve original casing from input
            let offset = "add ".len();
            let orig = &input[offset..];
            if let Some((n, d)) = orig.split_once(" to ") {
                return Command::Add {
                    name: n.trim().to_string(),
                    dept: d.trim().to_string(),
                };
            }
            return Command::Add { name, dept };
        }
    }

    if let Some(rest) = lower.strip_prefix("remove ") {
        if let Some((name_part, dept_part)) = rest.split_once(" from ") {
            let offset = "remove ".len();
            let orig = &input[offset..];
            if let Some((n, d)) = orig.split_once(" from ") {
                return Command::Remove {
                    name: n.trim().to_string(),
                    dept: d.trim().to_string(),
                };
            }
            let _ = (name_part, dept_part);
        }
    }

    if let Some(rest) = input.strip_prefix("list ") {
        let target = rest.trim();
        if target.eq_ignore_ascii_case("all") {
            return Command::ListAll;
        }
        return Command::ListDept(target.to_string());
    }

    if lower == "depts"  { return Command::Depts; }
    if lower == "quit" || lower == "exit" { return Command::Quit; }

    Command::Unknown
}
