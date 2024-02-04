use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    clear_terminal();

    loop {
        print_welcome();
        print_commands();

        let mut command: String = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error getting input");

        let command = command.trim().to_lowercase();

        match &command[..] {
            "d" => add_department(&mut directory),
            "e" => add_employee(&mut directory),
            "l" => list_employees(&directory),
            "q" => break,
            _ => {
                println!("Unknown command");
                continue;
            }
        }
    }
}

fn add_department(directory: &mut HashMap<String, Vec<String>>) {
    print!("What is the name of this new department? ");
    io::stdout().flush().unwrap();

    loop {
        let mut new_dir: String = String::new();

        io::stdin()
            .read_line(&mut new_dir)
            .expect("Error getting input");

        let new_dir = new_dir.trim().to_string();

        match directory.get(&new_dir) {
            Some(_) => {
                println!("Department already exists! Please enter another name");
                continue;
            }
            None => (),
        }

        directory.insert(new_dir.clone(), vec![]);

        clear_terminal();
        println!("Department added: {}\n", new_dir);
        break;
    }
}

fn add_employee(directory: &mut HashMap<String, Vec<String>>) {
    // TODO find a better way to do this without clone(). This isn't optimal
    let department: String = choose_department(directory.clone().into_keys().collect());

    loop {
        let mut new_emp: String = String::new();

        print!("Employee name: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut new_emp)
            .expect("Error getting input");

        let new_emp: String = new_emp.trim().to_string();
        let department_vec: Option<&mut Vec<String>> = directory.get_mut(&department);

        match department_vec {
            Some(_) => {
                department_vec.unwrap().push(new_emp.clone());
            }
            None => {
                clear_terminal();
                println!("Error, returning to main menu");
                return;
            }
        }

        clear_terminal();
        println!("Employee added to {}: {}\n", &department, new_emp);
        break;
    }
}

fn list_employees(directory: &HashMap<String, Vec<String>>) {
    let department: String = choose_department(directory.clone().into_keys().collect());

    let department_vec: Option<&Vec<String>> = directory.get(&department);

    match department_vec {
        Some(_) => {
            clear_terminal();
            for name in department_vec.unwrap() {
                println!("{} Employee(s):", department);
                println!("{}", name);
            }
            println!();
        }
        None => {
            clear_terminal();
            println!("Error, returning to main menu");
            return;
        }
    }
}

fn choose_department(departments: Vec<String>) -> String {
    loop {
        println!("Choose a department:");

        for dep in &departments {
            print!("  {}  |", dep);
        }

        print!("\nSelection: ");
        io::stdout().flush().unwrap();

        let mut dep: String = String::new();

        io::stdin()
            .read_line(&mut dep)
            .expect("Error getting input");

        let dep = dep.trim().to_string();

        match departments.contains(&dep) {
            true => return dep,
            false => println!("Couldn't find that department, please try again"),
        }
    }
}

fn print_welcome() {
    println!("======|| CRAB CORP. EMPLOYEE DIRECTORY ||======");
    println!(
        "                   __       __
                  / <`     '> \\
                 (  / @   @ \\  )
                  \\(_ _\\_/_ _)/
                (\\ `-/     \\-' /)
                ''===\\     /===''
                  .==')___(`==.
                 ' .='     `=."
    );
    println!();
}

fn print_commands() {
    println!("[D] Add a new department");
    println!("[E] Add a new employee to a department");
    println!("[L] List all employees in a department");
    println!("[Q]uit");
}

fn clear_terminal() {
    Command::new("clear")
        .status()
        .expect("Error clearing console");
}
