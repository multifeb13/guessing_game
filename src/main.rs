extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);

    let mut guess_count = 0;

    loop{
        println!("Please input your guess.");
        guess_count += 1;

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),       //小さすぎ！
            Ordering::Greater => println!("Too big!"),      //大きすぎ！
            Ordering::Equal => {
                println!("You win at {} time(s)", guess_count );                       //やったね！
                break;
            }
        }
    }
}
