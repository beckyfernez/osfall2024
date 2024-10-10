
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // 10 integer numbers
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15];

    for &num in numbers.iter() {

        let even_odd = if is_even(num) { "even" } else { "odd" };
        
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} - FizzBuzz", even_odd);
        } else if num % 3 == 0 {
            println!("{} - Fizz", even_odd);
        } else if num % 5 == 0 {
            println!("{} - Buzz", even_odd);
        } else {
            println!("{}", even_odd);
        } 
    }

    // Find the sum of array
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Find largest number in array
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
