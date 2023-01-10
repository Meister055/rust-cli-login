use passwords::PasswordGenerator;
use std::io;

fn login(credentials: &Credentials) -> bool {
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter your username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Enter your password: ");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    username = username.trim().to_string();
    password = password.trim().to_string();

    if let Some(index) = credentials.unames.iter().position(|x| *x == username) {
        if credentials.pwords[index] == password {
            return true;
        }
    }
    return false;
}

fn new_user(credentials: &mut Credentials) {
    let mut username = String::new();
    let mut password = String::new();

    loop {
        println!("Choose a username: ");
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");
        if let Some(index) = credentials.unames.iter().position(|x| *x == username) {
            println!("Username already exists");
            continue;
        } else {
            let username = username.trim().to_string();
            println!("Would you like to generate a password? (y/n)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input: String = input.trim().to_string();
            if input == "y" || input == "" {
                println!("How long would you like your password to be?");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input: String = input.trim().to_string();
                let input: usize = match input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input");
                        continue;
                    }
                };
                let password = PasswordGenerator::new()
                    .length(input)
                    .numbers(true)
                    .symbols(true)
                    .generate_one();
                let password = password.unwrap();
                println!("Your password is {}", password);

                credentials.unames.push(username);
                credentials.pwords.push(password);
                break;
            } else if input == "n" {
                println!("Choose a password: ");
                io::stdin()
                    .read_line(&mut password)
                    .expect("Failed to read line");
                let password = password.trim().to_string();
                credentials.unames.push(username);
                credentials.pwords.push(password);
                break;
            }
        }
    }
    return;
}

struct Credentials {
    unames: Vec<String>,
    pwords: Vec<String>,
}

fn main() {
    const COMMANDS: &str = "login: Logs in a user.
    signup: Creates a new user.
    logout: Signs out the current user.
    exit: Kills the current window.
    help: Shows this command.";

    let mut logged_in: bool = false;

    let mut credentials = Credentials {
        unames: vec!["admin".to_string(), "user".to_string()],
        pwords: vec!["password".to_string(), "password".to_string()],
    };

    println!("Meister Console (v1.0.0) (type 'help' for commands)\nCopyright (C) 2022 Joshua Siefert. All rights reserved.");

    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Input failed");
        let input = input.trim();
        let input = input.to_lowercase();

        if input == "exit" || input == "quit" || input == "stop" {
            break;
        } else if input == "login" {
            logged_in = login(&credentials);
            if logged_in {
                println!("Logged in!");
            } else {
                println!("Incorrect login, or you are already logged in.");
            }
        } else if input == "signup" {
            new_user(&mut credentials);
        } else if input == "help" {
            println!("Commands:\n {}", &COMMANDS);
        } else if input == "logout" {
            if logged_in {
                logged_in = false;
                println!("Logged out");
            } else {
                println!("You are not logged in.");
            }
        } else if input == "secret" {
            if logged_in {
                println!("The secret is: 42");
            } else {
                println!("You are not logged in.");
            }
        } else {
            println!("Invalid command");
        }
    }
}
