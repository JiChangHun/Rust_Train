extern crate rand;                              // external crate (or use rand;)

use std::io;                                    // input, show the result
use std::cmp::Ordering;                         // enum like Result (compare Less, Greater, Equal)
use rand::Rng;                                  // integer maker

fn main() {
    println!("Guess the number! (1 to 1024)");              // macro to show the string

    let secret_number = rand::thread_rng().gen_range(1, 1025);   // get integer maker, make random integer

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();          // mutable, new String instance

        io::stdin().read_line(&mut guess)       // Stdin instance, read_line method
            .expect("Failed to read line");     // Result=>err, stop

        let guess: u32 = match guess.trim().parse() {           // shadow guess, trim and parse, check num
            Ok(num) => num,
            Err(_)  => {
                println!("Your input is not a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {                       // compare values (arm's pattern)
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}