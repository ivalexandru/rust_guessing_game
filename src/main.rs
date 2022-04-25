use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //  generate random nr 
    let secret_number = rand::thread_rng().gen_range(0..10);    
    println!("The secret number is: {}", secret_number);


    loop {

    println!("Guess the number!");
    let mut guess = String::new();


    // ask for user input 
    // & indicates that this argument is a reference
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); 

    // convert String to number, ignoring wrong types
    // let guess: u32 = guess.trim().parse().expect("type a number");   
    let guess: u32 = match guess
                .trim()
                .parse() {
        Ok(ceva) => ceva,
        Err(_) => continue, 
    }; 



    // how is the nr user typed compared to the random gen nr?
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("you won!");
            break;
        } 
    }
}

}