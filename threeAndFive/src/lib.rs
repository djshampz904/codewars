pub fn solution(num: i32) -> i32 {
    (0..num)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

mod tests {
    use super::solution;

    #[test]
    fn sample_test() {
        assertion(23, 10);
        assertion(33, 11);
        assertion(225, 33);
        assertion(8, 6);
        assertion(3420, 123);
        assertion(543, 50);
        assertion(0, 0);
        assertion(0, -204);
        assertion(25719750, 10500);
    }

     fn assertion(expected : i32, input : i32) {
        let actual = solution(input);
        
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input
        );
    }
}



