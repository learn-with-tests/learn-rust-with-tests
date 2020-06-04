use std::ops::Add; // note this trait (i think?) needs importing

fn main() {
    println!("{}", sum3([1, 1, 1]))
}

fn sum1(numbers: [i32; 3]) -> i32 {
    let mut total = 0;
    let mut counter = 0;
    loop { // todo: use loop as an expression
        if counter == 3 {
            break
        }
        total = total + numbers[counter];
        counter = counter + 1;
    }
    total
}

fn sum2(numbers: [i32; 3]) -> i32 {
    let mut total = 0;
    let mut counter = 0;
    while counter != 3 {
        total = total + numbers[counter];
        counter = counter + 1;
    }
    total
}

fn sum3(numbers: [i32; 3]) -> i32 {
    let mut total = 0;
    for (_, &item) in numbers.iter().enumerate() { // for prevents bugs with magic number 3 if length is changed, reads better etc
        total = total + item
    }
    total
}

fn sum4(numbers: [i32; 3]) -> i32 {
    numbers
        .iter()
        .enumerate()
        .fold(0, |sum, val| val.1.add(sum)) // this feels wonky but fine
}

fn sum5(numbers: [i32; 3]) -> i32 {
    numbers.iter().sum()
}

#[cfg(test)]
mod tests;

/*
notes

use this crate for reduce https://docs.rs/reduce/0.1.2/reduce/
 */