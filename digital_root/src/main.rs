

fn main() {
    let num: String = format!("{}", 493193);
    let mut mynum = num.clone();

    loop {
        let sum_num: u32 = mynum
            .chars()
            .filter_map(|b| b.to_digit(10))
            .sum();

        if sum_num < 10 {
            println!("{}", sum_num);
            break;
        }

        mynum = sum_num.to_string();
    }


}
