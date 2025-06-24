use disemvowel::disemvowel;

fn main() {
    let myString: &str = "The quick brown fox jumped over the lazy dogs";

    let divowel = disemvowel(myString);

    println!("{}", divowel);

}
