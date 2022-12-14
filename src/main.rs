use std::io;

fn login(list_unames: Vec<String>, list_pwords: Vec<String>) -> bool {
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

    if list_unames.contains(&username) && list_pwords.contains(&password) {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut unames: Vec<String> = Vec::new();
    let mut pwords: Vec<String> = Vec::new();

    unames.push("admin".to_string());
    pwords.push("password".to_string());

    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Input failed");
loop {
    let input = input.trim();
    let input = input.to_lowercase();

    let current_unamelist: Vec<String> = unames.clone();
    let current_pwordlist: Vec<String> = pwords.clone();

    if input == "exit" {
        break;
    } else
    if input == "login" {
        if login(current_unamelist, current_pwordlist) {
            println!("Logged in!");
        } else {
            println!("Incorrect username or password");
        }
    } else {
        println!("Invalid command");
    }
}
}
