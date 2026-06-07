fn main() {
    // Reference
    let s = String::from("Camilla beve la camomilla");
    let length = calculate_length(&s);
    println!("{s} = {length}");

    // Reference mutable
    let mut s1 = s.clone();
    change(&mut s1);
    println!("{s1}");

    // We can not do this because we can borrow as mutable once
    /* let sr = &mut s1;
    let sr2 = &mut s1;
    println!("{sr} {sr2}"); */

    // We can not do this because we have mutable and immutable references
    /* let sr = &s1;
    let sr2 = &s1;
    let sr3 = &mut s1;
    println!("{sr} {sr2} {sr3}"); */

    // We can do this
    let sr = &s1;
    let sr2 = &s1;
    println!("{sr} {sr2}"); // From here sr and sr2 are not being used, so we can declare sr3 as mutable

    let sr3 = &mut s1;
    println!("{sr3}");

    // Dangling reference
    let dr = no_dangle();
    println!("{dr}");
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(s: &mut String) {
    s.push_str(" ogni giorno");
}

// Dangling pointers
/* fn dangle() -> &String {
    let s = String::from("hello");
    &s
} */

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}