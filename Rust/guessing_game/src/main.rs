extern crate rand;

use std::io; // Standard library to bring in input/output library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number! \n"); //Println macro that prints string
    
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new(); //by default variable is immutable mut makes it mutable

        io::stdin().read_line(&mut guess).expect("Failed to read line");// :: associated function

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //Shadowing
        
        println!("Your guess: {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),  //Ordering is another enum
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
