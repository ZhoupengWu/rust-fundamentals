fn main() {
    // If
    let x = 5;

    if x > 5 {
        println!("Hello world");
    }
    else if x == 5 {
        println!("Sei fortunato oggi");
    }
    else {
        println!("ALL'INFERNO");
    }

    let mut y = if x > 0 { 100 } else { -100 };
    println!("Y: {y}");

    // Loop
    loop {
        println!("Continua a dormire...");

        if y == 110 {
            break;
        }

        y += 1;
    }

    let tax = loop {
        y -= 1;

        if y < 50 {
            break y * 5 + y * 4;
        }
    };
    println!("Tassa: {tax}€");

    // We can choose which loop will use break or continue, using label ex. 'label
    let mut counter = 0;

    'out: loop {
        println!("Counter = {counter}");
        let mut outer = 0;

        loop {
            println!("Outer = {outer}");

            if outer == 7 {
                break;
            }

            if counter == 3 {
                break 'out;
            }

            outer += 1;
        }

        counter += 1;
    }

    // While
    let mut wallet = 1000;

    while wallet > 300 {
        println!("Sei ancora ricco!");

        wallet -= 200;
    }

    println!("Ora sei povero! ahahahahahahaha");

    // For
    let marks = [8, 6, 4, 9, 7, 4];

    for mark in marks {
        println!("Hai preso {mark}");
    }

    for n in (1..=10).rev() {
        println!("{n}");
    }

    println!("GOOOOOOO!!!");
}