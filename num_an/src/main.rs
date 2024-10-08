// Function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers
    let numbers: [i32; 10] = [3, 5, 15, 8, 12, 7, 18, 10, 20, 25];

    // For loop to iterate through the array and print if the number is even or odd
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            if is_even(num) {
                println!("{}: Even", num);
            } else {
                println!("{}: Odd", num);
            }
        }
    }

    // While loop to find and print the sum of all numbers in the array
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Find and print the largest number in the array
    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}

