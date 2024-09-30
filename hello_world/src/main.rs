use std::io::{self, Write};
use std::fs::{File, OpenOptions};


struct Car {
    model: String,
    year: u32,
}

fn reading_from_console() {
    let mut buffer = String::new(); 

    print!("What is your car model? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("How year is your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    let car = Car { model, year };
    //println!("Your car is a {} and the year is {}", car.model, car.year); 
    let mut file = OpenOptions::new().append(true).create(true).open("user_info.txt").unwrap();
    writeln!(file, "Your car is a {} and the year is {}", car.model, car.year).unwrap();
}



fn main() {
    reading_from_console();
}