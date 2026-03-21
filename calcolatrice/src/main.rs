use std::io;

fn main() {
    println!("===Calcolatrice===");

    let a = leggi_numero("Inserisci il primo numero: ");
    let operatore = leggi_operatore();
    let b = leggi_numero("Inserisci il secondo numero: ");

    let risultato = match operatore {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0.0 {
                println!("Errore: divisione per zero");
                return;
            }
            a / b
        }
        _ => {
            println!("Operatore non valido");
            return;
        }
    };

    println!("{} {} {} = {}", a, operatore, b, risultato);
}

fn leggi_numero(messaggio: &str) -> f64 {
    loop {
        println!("{}", messaggio);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Errore nella lettura");

        match input.trim().parse::<f64>() {
            Ok(numero) => return numero,
            Err(_) => println!("Inserisci un numero valido"),
        }
    }
}

fn leggi_operatore() -> char {
    loop {
        println!("Inserisci operatore (+, -, *, /): ");
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Errore nella lettura");

        let carattere = input.trim().chars().next();

        match carattere {
            Some(c) if "+-*/".contains(c) => return c,
            _  => println!("Operatore non valido, riprova"),
        }
    }
}
