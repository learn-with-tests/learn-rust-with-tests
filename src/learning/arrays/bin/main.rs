use std::ops::Add; // note this trait (i think?) needs importing

fn main() {
    println!("{}",sum1([1, 1, 1]))
}

fn sum1(numbers: [i32; 3]) -> i32 {
    let mut total = 0;
    for (_, &item) in numbers.iter().enumerate() {
        total = total + item
    }
    return total;
}

fn sum2(numbers: [i32; 3]) -> i32 {
    numbers
        .iter()
        .enumerate()
        .fold(0, |sum, val| val.1.add(sum)) // this feels wonky but fine
}

fn sum3(numbers: [i32; 3]) -> i32 {
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{sum1, sum2, sum3};

    const NUMBERS: [i32; 3] = [1, 2, 3];
    const EXPECTED_SUM: i32 = 6;

    fn testSum(f: fn([i32; 3]) -> i32) {
        assert_eq!(EXPECTED_SUM, f(NUMBERS))
    }

    #[test]
    fn test_sum1() {
        testSum(sum1)
    }

    #[test]
    fn test_sum2() {
        testSum(sum2)
    }

    #[test]
    fn test_sum3() {
        testSum(sum3);
    }
}

/*
notes

use this crate for reduce https://docs.rs/reduce/0.1.2/reduce/
 */