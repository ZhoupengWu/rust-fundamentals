fn main() {
    let middle = 25;
    let top_left = middle - 12;
    let top_right = middle + 12;

    for x in 0..=34 {
        let base_start = x - 13;
        let base_end = 50 - x + 13;

        for y in 0..=50 {
            if (x < 13 && (y == top_left - x || y == top_right + x)) || (x < 12 && (y + 1 == top_left + x + 2 || y - 1 == top_right - x - 2)) {
                print!("*");
            }
            else if (x >= 13 && x < 18) && (y == base_start + 1 || y == base_end - 1) {
                print!("*");
            }
            else if (x >= 18 && x < 23) && (y == base_start + 2 || y == base_end - 2) {
                print!("*");
            }
            else if (x >= 23 && x < 28) && (y == base_start + 3 || y == base_end - 3) {
                print!("*");
            }
            else if x >= 28 && (y == base_start + 4 || y == base_end - 4) {
                print!("*");
            }
            else {
                print!(" ");
            }
        }

        println!();
    }
}