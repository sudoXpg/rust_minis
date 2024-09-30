use std::io;                                                                // input outputr library
use std::cmp::Ordering;                                                     // to get ordering
use rand::Rng;                                                              // random num **crate**
fn main() {
    println!("GUESSING GAME ");                      
    let secret_number = rand::thread_rng().gen_range(1..=100);        // thread_rng() -> seeded by system | gen_range -> start..=end == [start,end]
    //println!("The secret number is {secret_number}");
    loop{
        println!("Input your guess : ");
        let mut guess = String::new();                                // let used to create variables | variables are mutable by def. 
        io::stdin()                                                     // for taing user input
            .read_line(&mut guess)                  // result has 2 parameter OK and Err 
            .expect("Failed to read line");                             // err msg
        let guess:u32 = match guess.trim().parse() {                        // trim removes trailing whitespaces/new line | parse does type conversion 
            Ok(num) => num,     // if num then continue
            Err(_) => {
                println!("Enter a number !");
                continue;
            },     // if error then inp again '_' means match all errors (if any)
        };                                                                  // parse returns -> result type 

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Equal =>{ 
                        println!("Equal \nYou win !");
                        break;
                        },
            Ordering::Greater => println!("Too big"),
        }
    }


}