pub fn disemvowel(s: &str) -> String {
    let mut disvowel: String = String::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    for i in s.chars() {
        if vowels.contains(&i){
            continue
        } else {
            disvowel.push(i);
        }
    }

    disvowel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"),
                    "Ths wbst s fr lsrs LL!");
    }
}
