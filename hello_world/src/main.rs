use std::io;

const FREEZE: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let celsius_num = (f - 32.0) * 5.0/9.0;

    celsius_num
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let f_num = (9.0/5.0) * c + 32.0;

    f_num
}


fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        println!("{} is even", n);
        true
    }
     else {
         println!("{} is odd", n);
        false
     }
 }


 fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 //guess is correct
    }
    else if guess > secret {
        1 //guess is too high
    }
    else {
        -1 //guess is too low
    }
}




fn main() {
    //problem 1
   let mut fahrenheit_num: f64 = 32.0; //this would be in fahrenheit

   //convert to celsius with function
    println!("Temperature in Celsius is: {}", fahrenheit_to_celsius(fahrenheit_num));
    
    //convert to fahrenheit
    println!("Temperature in Fahrenheit is: {}", celsius_to_fahrenheit(0.0));

    //loop
    loop {
        println!("Fahrenheit {} to Celsius {}", fahrenheit_num, fahrenheit_to_celsius(fahrenheit_num));
        fahrenheit_num += 1.0;

        if fahrenheit_num == 38.0 {
            break;
        }
    }


    //problem 2
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for num in arr.iter() {
         //println!("{}", is_even(*num));
         if num % 3 == 0 {
             println!("Fizz");
         }
         else if num % 5 == 0 {
             println!("Buzz");
         }
         else if num % 3 == 0 && num % 5 == 0 {
             println!("FizzBuzz");
         }
         else {
             println!("{}", is_even(*num));
         }
     }
 
     let mut sum = 0;
     let mut counter = 0;
     while counter != 10 {
         sum += arr[counter];
         counter += 1;
 
         if counter == 9 {
             println!("Sum of array: {}", sum);
         }
     }
 
     let mut large_num = 0;
     for num in arr.iter() {
         if num > &large_num {
             large_num = *num;
         }
     }
     println!("The largest number is : {}",  large_num);


     //problem 3
    let secret_number = 20;
    let mut count = 0;

    loop {
        let mut guess = String::new(); //creates a mutable value to get input
        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut guess) //the input is in a mutable var
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //removes newline from input

        //println!("You guessed: {}", guess);
        let result = check_guess(guess as i32, secret_number);

        if result == 1 {
            println!("number is too high");
            count += 1;
        }
        if result == -1 {
            println!("number is too low");
            count += 1;

        }
        if result == 0 {
            println!("winner");
            count += 1;
            break;
        }
    }

    println!("Number of guesses was  {}", count);
}