use std::io;

fn main() {
    loop {
        let mut choose = String::new();

        println!("########## CONVERSIONE °F - °C ##########");
        println!("0. Exit");
        println!("1. From °C to °F");
        println!("2. From °F to °C");
        println!();

        println!("La tua scelta: ");

        io::stdin()
            .read_line(&mut choose)
            .expect("Failed to read line");

        match choose.trim() {
            "0" => {
                println!("Bye bye");

                break;
            },
            "1" => println!("Temperatura: {}°F", from_c_to_f()),
            "2" => println!("Temperatura: {}°C", from_f_to_c()),
            _ => println!("Opzione non valida. Riprova!")
        }
    }
}

fn from_c_to_f() -> i32 {
    loop {
        let mut temperature_c = String::new();
        println!("Inserisci una temperatura in °C: ");

        io::stdin()
            .read_line(&mut temperature_c)
            .expect("Failed to read line");

        let temperature_c: i32 = match temperature_c.trim().parse() {
            Ok(t) => t,
            Err(_) => continue
        };

        return (temperature_c * 9 / 5) + 32;
    }
}

fn from_f_to_c() -> i32 {
    loop {
        let mut temperature_f = String::new();
        println!("Inserisci una temperatura in °F: ");

        io::stdin()
            .read_line(&mut temperature_f)
            .expect("Failed to read line");

        let temperature_f: i32 = match temperature_f.trim().parse() {
            Ok(t) => t,
            Err(_) => continue
        };

        return (temperature_f - 32) * 5 / 9;
    }
}