use std::io;
use passwords::PasswordGenerator;

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
            }
            else if input == "n" {
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
    let mut credentials = Credentials {
        unames: vec!["admin".to_string(), "user".to_string()],
        pwords: vec!["password".to_string(), "password".to_string()],
    };

    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Input failed");
        let input = input.trim();
        let input = input.to_lowercase();

        if input == "exit" || input == "quit" || input == "stop" {
            break;
        } else if input == "login" {
            if login(&credentials) {
                println!("Logged in!");
            } else {
                println!("Incorrect username or password");
            }
        } else if input == "signup" {
            new_user(&mut credentials);
        } else {
            println!("Invalid command");
        }
    }
}
