pub fn solution(s: &str) -> String {
    let mut s1 = s.clone();
    let mut s = String::new();

    for (idx, i) in s1.char_indices() {
        if i.is_uppercase() {
            s.push(' ');
            s.push(i);
            continue;
        }
        s.push(i)
    }

    return s
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
