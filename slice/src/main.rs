fn main() {
    let old_s = String::from("I like eating fruits");
    let old_i = old_first_word(&old_s);
    println!("The first word of '{old_s}' is at {old_i}");

    // String slices
    let s1 = String::from("Non è colpa mia");
    let first_s1 = &s1[0..6];
    let second_s1 = &s1[7..16];
    println!("{first_s1} {second_s1}");

    /*
        These are same:
            - [0..2] == [..2]
            - [3..len] == [3..]
            - [0..len] == [..]
     */

    let new_s = String::from("I love you");
    let new_i = new_first_word(&new_s);
    println!("The first word of  is {new_i}");

    /* let mut a = String::from("Cioa cioa cioa");
    let b = new_first_word(&a);
    let c = new_first_word(&a);
    println!("{c}");
    a.clear(); // We can not do this here because we need to wait that the borrowed variables are no longer to use
    println!("{b}");
     */

    // String literals as slices
    let sl = "Addio";
    println!("{sl}");

    // Slices as params
    let ms = String::from("Ciao padre pio");
    let _word = first_word(&ms[0..5]);
    let _word = first_word(&ms[..]);
    let _word = first_word(&ms);

    let msl = "Ciao padre pio";
    let _word = first_word(&msl[0..5]);
    let _word = first_word(&msl[..]);
    let _word = first_word(msl);

    // Other slices
    let arr = [1, 2, 3, 4, 5, 6];
    let arr_slice = &arr[0..4];
    assert_eq!(arr, arr_slice);
}

fn old_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn new_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}