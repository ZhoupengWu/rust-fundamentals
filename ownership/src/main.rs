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
}