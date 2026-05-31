// Const must have a type
const MAX_IQ: u32 = 200;

fn main() {
    let immutable_variable = 5;
    let mut mutable_variable = 10;

    println!("Variabile immutabile: {immutable_variable}");
    println!("Variabile mutabile: {mutable_variable}");

    mutable_variable = 15;
    println!("Variabile mutabile: {mutable_variable}");

    println!("MAX IQ: {MAX_IQ}");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inside bracket: {x}");
    }

    println!("Outside bracket: {x}");

    /*
        // Different types
        let mut spaces = "       ";
        spaces = spaces.len();

        // Right method
        let space = "           ";
        let space = space.len();
    */
}