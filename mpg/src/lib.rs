fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    distance_to_pump <= mpg * gallons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {

        assert_eq!(zero_fuel(50, 25, 2), true);
        assert_eq!(zero_fuel(100, 50, 1), false);
    }
}
