fn main() {
    /*
        Integer: (default i32)
            - signed i{} or isize
            - unsigned u{} or usize
            - You can use underscore for readibility ex. 4_000_000
            - 0x, 0o, 0b, b'a'
        Floating point: (default f64)
            - f32, f64
        Boolean:
            - bool (false or true)
        Character:
            - char
    */

    let sum = 5 + 3;
    let dif = 42 - 54;
    let product = 92 * 4738;
    let quotient = 54.76 / 18.98;
    let truncated = 199 / 150;
    let remainder = 745 % 3;
    println!("{sum} - {dif} - {product} - {quotient} - {truncated} - {remainder}");

    /*
        Compound types
    */

    // Tuple (like in python) (if is empty it's called unit and when a function doesn't return anything, it returns a unit)
    let private_info: (&'static str, i32, &'static str) = ("Alessia", 2008, "Roma");
    let (name, year, city) = private_info;
    println!("{name} è nato a {city} ed è del {year}");

    let trio = (1, 2, 3);
    let _x = trio.0;
    let _y = trio.1;
    let _z = trio.2;

    // Array
    let _marks = [7, 4, 9, 6, 5];
    let _keke: [f64; 3] = [58.54, 98.23, 57.73]; // 3 elements of type f64
    let ekek = [10; 5]; // 5 elements of the value 10
    println!("First element: {}", ekek[0]);
}