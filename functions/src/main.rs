fn main() {
    println!("Hello, world!");

    verso_paradiso();

    println!("Area del rettangolo: {}", area_rect(67.67, 76.76));

    // Rust is an expression-based language
    let x = {
        let y = 0;
        y + 1 * 5 // Expression doesn't need semicolon because otherwise it becomes a statement and returns unit
    };

    println!("X: {x}");
}

fn verso_paradiso() {
    println!("Sei all'inferno");
}

fn area_rect(b: f64, h: f64) -> f64 {
    b * h
}