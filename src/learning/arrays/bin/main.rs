use std::ops::Add;

fn main() {
    println!("{}",sum([1, 1, 1]))
}

fn sum(numbers: [i32; 3]) -> i32 {
    let mut total = 0;
    for (_, &item) in numbers.iter().enumerate() {
        total = total + item
    }
    return total;
}

fn sum2(numbers: [i32; 3]) -> i32 {
    // coerce the array into a slice
    let iter = numbers.iter();

    let enumerate = iter.enumerate();
    enumerate.fold(0, |sum, val| val.1.add(sum))
}

#[cfg(test)]
mod tests {
    use crate::{sum, sum2};

    #[test]
    fn test_sum1() {
        assert_eq!(6, sum([1, 2, 3]));
    }

    #[test]
    fn test_sum2() {
        assert_eq!(6, sum2([1, 2, 3]));
    }
}

/*
notes

use this crate for reduce https://docs.rs/reduce/0.1.2/reduce/
 */