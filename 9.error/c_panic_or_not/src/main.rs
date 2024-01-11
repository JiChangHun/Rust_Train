use std::net::IpAddr;

fn main() {
    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

        println!("Guess the number! (1 to 1024)");              // macro to show the string

    let secret_number = rand::thread_rng().gen_range(1, 1025);

    loop {
        println!("Please input your guess.");

        // let mut guess = String::new();          // mutable, new String instance

        // io::stdin().read_line(&mut guess)       // Stdin instance, read_line method
        //     .expect("Failed to read line");     // Result=>err, stop

        // let guess: i32 = match guess.trim().parse() {           // shadow guess, trim and parse, check num
        //     Ok(num) => num,
        //     Err(_)  => continue,
        // };

        // if guess < 1 || guess > 1024 {
        //     println!("The secret number will be between 1 and 1024.");
        //     continue;
        // }

        // let username_file_result = File::open("hello3.txt");

        // let mut username_file = match username_file_result {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };
        // let mut username_file = File::open("hello2.txt")?;

        let input: i32 = {  let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input: i32 = match input.trim().parse() {
                                Ok(num) => num,
                                Err(_)  => continue,
                            };
                            input
                        };
        let guess = Guess::new(input);

        println!("You guessed: {:?}", guess.value);

        match guess.value.cmp(&secret_number) {                       // compare values (arm's pattern)
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}

use std::io;                                    // input, show the result
use std::cmp::Ordering;                         // enum like Result (compare Less, Greater, Equal)
use rand;
use rand::Rng;                                  // integer maker

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 1024 {
            panic!("Guess value must be between 1 and 1024, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}