use std::cmp::Ordering;
use std::{io, io::Write};
use colored::Colorize;


fn main() {
    println!("\nWrite the Mobile Help Hangar emails!\n");

    loop {
        println!("Is this:\n[1] An initial communication, or\n[2] A secondary communication");

        let mut user_choice = String::new();

        print!(">");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        let user_choice: u32 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "That was not a valid input.\n".bold().red());
                continue;
            },
        };


        match user_choice {
            1 => {
                println!("You chose 1."); // Initial Communication Email
                println!("Who is the Site Admin (first name)?");
                let mut site_admin = String::new();
                print!(">");
                let _ = io::stdout().flush();
                io::stdin()
                    .read_line(&mut site_admin)
                    .expect("Failed to read line");
            },
            2 => {
                println!("You chose 2."); // Secondary Communication Email

            },
            _ => {
                println!("{}", "That was not a valid input.\n".bold().red());
                continue;
            }
        };


/* 
        match user_choice.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
                
            }
        }
*/
    }
}
