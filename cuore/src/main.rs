fn main() {
    let middle = 25;
    let top_left = middle - 12;
    let top_right = middle + 12;

    for x in 0..=35 {
        for y in 0..=50 {
            if x < 13 && (y == top_left - x || y == top_left + x || y == top_right - x || y == top_right + x) {
                print!("*");
            }
            else if (x >= 13 && x < 20) && (y == x - 13 + 1 || y == 50 - x + 13 - 1) {
                print!("*");
            }
            else if (x >= 20 && x < 27) && (y == x - 13 + 2 || y == 50 - x + 13 - 2) {
                print!("*");
            }
            else if (x >= 27) && (y == x - 13 + 3 || y == 50 - x + 13 - 3) {
                print!("*");
            }
            else {
                print!(" ");
            }
        }

        println!();
    }
}