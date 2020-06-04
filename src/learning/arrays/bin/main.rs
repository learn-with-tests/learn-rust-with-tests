use std::ops::Add; // note this trait (i think?) needs importing

fn main() {
    println!("{}", sum2([1, 1, 1]))
}

fn sum2(numbers: [i32; 3]) -> i32 {
    let mut total = 0;
    for (_, &item) in numbers.iter().enumerate() {
        total = total + item
    }
    total
}

fn sum1(numbers: [i32; 3]) -> i32 {
    let mut total = 0;
    let mut counter = 0;
    loop { // todo: use loop as an expression
        if counter == numbers.len() {
            break
        }
        total = total + numbers[counter];
        counter = counter + 1;
    }
    total
}

fn sum3(numbers: [i32; 3]) -> i32 {
    numbers
        .iter()
        .enumerate()
        .fold(0, |sum, val| val.1.add(sum)) // this feels wonky but fine
}

fn sum4(numbers: [i32; 3]) -> i32 {
    numbers.iter().sum()
}

#[cfg(test)]
mod tests;

/*
notes

use this crate for reduce https://docs.rs/reduce/0.1.2/reduce/
 */