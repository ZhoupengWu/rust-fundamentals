fn main() {
    let middle = 25;
    let top_left = middle - 12;
    let top_right = middle + 12;
    let mut counter = 0;

    for x in 0..25 {
        for y in 0..50 {
            if (x == 15) && (y == top_left || y == top_right) {
                print!("*");

                continue;
            }

            if x < 13 && (y == top_left - x || y == top_left + x || y == top_right - x || y == top_right + x) {
                print!("*");

                continue;
            }
            else {
                print!("#");
            }
        }

        println!();
    }
}