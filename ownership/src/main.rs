fn main() {
    // String type
    let mut s = String::from("Hello");
    s.push_str(" world");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("{x} {y}");

    // Move from a variable to another variable

    // We can't do this
    /* let s2 = String::from("ciao");
    let s3 = s2;
    println!("{s2} {s3}"); */

    // We can do this
    let mut s2 = String::from("ciao");
    let s3 = s2.clone(); // Deep copy data
    println!("{s2} {s3}");
    s2 = String::from("buona sera");
    println!("{s2} {s3}");

    // Ownership and functions
    let secret = String::from("ghsifidsbfhbsiufisdgsjgiusi");
    lose_ownership(secret);
    // We can not do this
    /* println!("{secret}"); */

    let id = 14875973;
    make_copy(id);
    println!("Main id: {id}");

    // Return
    let value = give_ownership();
    println!("{value}");
    let value2 = String::from("Padre pio");
    let value3 = lose_and_give_back(value2);
    println!("{value3}");

    let word = String::from("Alla sera vedi una rag");
    let (length, word2) = calculate_length(word);
    println!("The word '{word2}' has length of {length}");

}

fn lose_ownership(s: String) {
    println!("Your secret: {s}");
}

fn make_copy(id: i32) {
    println!("Your id: {id}");
}

fn give_ownership() -> String {
    let s = String::from("Zio pera");
    s
}

fn lose_and_give_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (usize, String) {
    let length = s.len();
    (length, s)
}