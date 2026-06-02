use std::io;

fn main() {
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    loop {
        let mut n = String::new();

        println!("Inserisci quanti cicli vuoi fare!");

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => continue
        };

        if n == 0 {
            println!("Il valore è 0");

            break;
        }

        for _ in 0..n {
            println!("{a} {b}");
            let c = a + b;
            a = b;
            b = c;
        }

        println!("Il valore è {b}");

        break;
    }
}