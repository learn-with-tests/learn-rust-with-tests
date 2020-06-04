use crate::{sum3, sum4, sum5, sum1, sum2};

const NUMBERS: [i32; 3] = [1, 2, 3];
const EXPECTED_SUM: i32 = 6;

fn test_sum(f: fn([i32; 3]) -> i32) {
    assert_eq!(EXPECTED_SUM, f(NUMBERS))
}

#[test]
fn test_sums() {
    test_sum(sum1);
    test_sum(sum2);
    test_sum(sum3);
    test_sum(sum4);
    test_sum(sum5);
}
