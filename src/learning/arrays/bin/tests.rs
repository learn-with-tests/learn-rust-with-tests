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
