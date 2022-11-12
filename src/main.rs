use std::io;
use local_ip_address::local_ip;
use regex::Regex;
use colored::Colorize;

fn main() {
    // Name
    let mut firstname;
    loop {
        firstname = String::new();

        println!("Enter your first name: ");
        io::stdin()
            .read_line(&mut firstname)
            .expect("Failed to read first name.");

        let contains_digit = firstname.chars().any(|firstname| firstname.is_ascii_digit());
        if contains_digit {
            println!("{} is not a valid first name.", firstname.trim());
            continue;
        } else {
            println!("Your first name is: {}", firstname.trim());
            break;
        }
    }

    let mut surname;
    loop {
        surname = String::new();

        println!("Enter your surname: ");
        io::stdin()
            .read_line(&mut surname)
            .expect("Failed to read surname.");

        let contains_digit = surname.chars().any(|lastname| lastname.is_ascii_digit());
        if contains_digit {
            println!("{} is not a valid surname.", surname.trim());
            continue;
        } else {
            println!("Your surname is: {}", surname.trim());
            break;
        }
    }

    // Age
    let mut age;
    loop {
        age = String::new();

        println!("Enter your age: ");
        io::stdin()
            .read_line(&mut age)
            .expect("Failed to read age.");

        let age: u32 = match age.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if age > 120 {
            println!("Please enter a valid age.");
            continue;
        } else {
            println!("Your age is: {}", age);
            break;
        }
    }

    // Gender
    let mut gender;
    loop {
        gender = String::new();

        println!("Enter you gender: ");
        io::stdin()
            .read_line(&mut gender)
            .expect("Failed to read gender.");

        match gender.trim().to_lowercase().as_str() {
            "male" => {
                println!("You are a male.");
                break;
            }
            "female" => {
                println!("You are a female.");
                break;
            }
            _ => {
                println!("Failed to enter gender, please enter male or female.");
                continue;
            }
        }
    }

    // Address
    let mut address = String::new();

        println!("Enter your address: ");
        io::stdin()
            .read_line(&mut address)
            .expect("Failed to read address.");

        println!("Your address is: {}", address.trim());

    // Email
    let mut email;
    loop {
        email = String::new();

        println!("Enter your email address: ");
        io::stdin()
            .read_line(&mut email)
            .expect("Failed to read email address.");

        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        if email_regex.is_match(&email) {
            println!("Your email address is: {}", email.trim());
            break;
        } else {
            println!("Invalid address. Please enter a valid email address.");
        }
    }

    // Phone
    let mut phone;
    loop {
        phone = String::new();

        println!("Enter your phone number: ");
        io::stdin()
            .read_line(&mut phone)
            .expect("Failed to read phone number.");

        match phone.trim().parse::<u32>() {
            Ok(_) => {
                println!("Your phone number is: {}", phone.trim());
                break;
            }
            Err(_) => {
                println!("Please enter a valid phone number.");
                continue;
            }
        }
    }

    let local_ip = local_ip().unwrap();
    println!("--------------------------------------------------");
    println!("-  First name: {}", firstname.trim().bright_blue());
    println!("-  Surname: {}", surname.trim().bright_blue());
    println!("-  Age: {}", age.trim().bright_blue());
    println!("-  Gender: {}", gender.trim().bright_blue());
    println!("-  Address: {}", address.trim().bright_blue());
    println!("-  Email: {}", email.trim().bright_blue());
    println!("-  Phone: {}", phone.trim().bright_blue());
    println!("-  Local IP address: {}", local_ip.to_string().bright_blue());
    println!("--------------------------------------------------");
}
