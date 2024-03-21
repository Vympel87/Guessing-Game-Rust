use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let number = rand::thread_rng().gen_range(1..110);
    println!("The number is: {}", number);

    loop {
        println!("Please input the guess number.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Your guess: {}", guess);
    
        match guess.cmp(&number) {
            Ordering::Less => println!("The number is too small!"),
            Ordering::Greater => println!("The number is too big!"),
            Ordering::Equal => {
                println!("Your guess is right!");
                break;
            }
        }
    }

}
