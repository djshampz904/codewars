fn digital_root(n: i64) -> i64 {
   (n - 1) % 9 + 1

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_root() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);
    }
}

