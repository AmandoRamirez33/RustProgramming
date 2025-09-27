//Assignment 1
const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f -FREEZING_POINT ) * 5.0 / 9.0
}
fn celsius_to_fahrenheiht(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT
}
fn assignment1(){
    let temp_f: f64 = 32.0;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.1}°F is {:.1}°C", temp_f, temp_c);

    for i in 1..=5{
        let new_temp_f = temp_f + i as f64;
        let new_temp_c = fahrenheit_to_celsius(new_temp_f);
        println!("{:.1}°F is {:.1}°C", new_temp_f, new_temp_c);
    }
    let celsius_value = 0.0;
    let converted_f = celsius_to_fahrenheiht(celsius_value);
    println!("{:.1}°C is {:.1}°F", celsius_value, converted_f);
}
//Assignment 2
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn assignment2() {
let numbers = [3, 10,15,22,9,30,7,5,12,18];
for n in numbers.iter(){
    if *n % 3 == 0 && *n % 5 == 0 {
        println!("{} -> FizzBuzz", n);
    } else if *n % 3 == 0 {
        println!("{} -> Fizz", n);
    } else if *n % 5 == 0 {
        println!("{} -> Buzz", n);
    } else if is_even(*n) {
        println!("{} -> Even", n);
    } else {
        println!("{} -> Odd", n);
    }
}
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len(){
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    let mut largest = numbers[0];
    let mut i = 1;
    loop {
        if i >= numbers.len(){
            break;
        }
        if numbers[i] > largest{
            largest = numbers[i];
        }
        i += 1;
    }
    println!("Largest number: {}", largest);
}
//Assignment 3
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret{
        1
    } else {
        -1
    }
}
fn assignment3(){
    let secret = 42;
    let mut guess = 30;
    let mut attempts = 0;
    loop{
        attempts += 1;
        let result = check_guess(guess, secret);
        if result == 0{
            println!("Correct. Secret number is {}", guess);
            break;
        } else if result == 1{
            println!("{} is too high!", guess);
            guess -= 1;
        } else {
            println!("{} is too low!", guess);
            guess += 1;
        }
    }
    println!("It took {} guess to find the secret.", attempts);
}


fn main(){
    assignment1();
    assignment2();
    assignment3();
}