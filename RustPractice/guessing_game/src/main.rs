fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 23; // Hard-coded secret number
    let mut guess_count = 0;
    let mut low = 1;
    let mut high = 100;

    println!("I will guess a secret number from 1 - 100.");

    loop {
        let guess = (low + high) / 2;
        guess_count += 1;
        
        println!("{}", guess);

        match check_guess(guess, secret) {
            0 => {
                println!("Correct! The secret number is {}.", secret);
                break;
            }
            1 => {
                println!("Too high.");
                high = guess - 1;
            }
            -1 => {
                println!("Too low.");
                low = guess + 1;
            }
            _ => unreachable!(),
        }
    }

    println!("It took {} tries to guess the secret number.", guess_count);
}