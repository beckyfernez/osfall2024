
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers
    let numbers = [15, 7, 3, 9, 12, 5, 20, 25, 8, 1];

    // For loop to iterate through the array
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }

    // While loop to find the sum of all numbers
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Loop to find the largest number
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
