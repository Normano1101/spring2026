//Q1
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

//Q2
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

//Q3
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
    //Q1
    println!("Celcius to Fahrenheit: {:.2}", fahrenheit_to_celsius(89.3));
    println!("Fahrenheit to Celcius: {:.2}", celsius_to_fahrenheit(23.4));

    //Q2
    let nums: [i32; 10] = [4, 7, 9, 10, 15, 18, 21, 22, 25, 30];

    for &n in nums.iter() {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{} -> FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{} -> Fizz", n);
        } else if n % 5 == 0 {
            println!("{} -> Buzz", n);
        } else {
            if is_even(n) {
                println!("{} -> Even", n);
            } else {
                println!("{} -> Odd", n);
            }
        }
    }

    let mut i: usize = 0;
    let mut sum: i32 = 0;

    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum = {}", sum);

    let mut idx: usize = 0;
    let mut largest: i32 = nums[0];

    loop {
        if nums[idx] > largest {
            largest = nums[idx];
        }

        idx += 1;

        if idx >= nums.len() {
            break;
        }
    }
    println!("Largest = {}", largest);

    //Q3 (Option B)
    let secret: i32 = 42;
    let mut guess: i32 = 0;
    let mut attempts: i32 = 0;

    loop {
        attempts += 1;

        if guess < secret {
            guess += 15;
        } else {
            guess = secret; // force correct after passing it
        }

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct!");
            break;
        } else if result == 1 {
            println!("Too high");
        } else {
            println!("Too low");
        }
    }

    println!("Attempts: {}", attempts);
}
