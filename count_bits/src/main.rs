

fn main() {
    let n: i32 = 1234;
    let n_binary: String = format!("{:b}", n);
    
    let sum_binary: u32 = n_binary
        .chars()
        .filter_map(|b| b.to_digit(10))
        .sum();

    println!("{}", sum_binary);
    
}
