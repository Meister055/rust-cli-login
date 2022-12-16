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

struct Credentials {
    unames: Vec<String>,
    pwords: Vec<String>,
}

fn main() {
    let credentials = Credentials {
        unames: vec!["admin".to_string(), "user".to_string()],
        pwords: vec!["password".to_string(), "password".to_string()],
    };

    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Input failed");
loop {
    let input = input.trim();
    let input = input.to_lowercase();

    if input == "exit"  || input == "quit" || input == "stop" {
        break;
    } else
    if input == "login"{
        if login(&credentials) {
            println!("Logged in!");
        } else {
            println!("Incorrect username or password");
        }
    } else {
        println!("Invalid command");
    }
}
}
