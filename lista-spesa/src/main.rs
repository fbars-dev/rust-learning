use std::io;

fn main() {
    let mut lista: Vec<String> = Vec::new();

    loop {
        println!("\n=== Lista della Spesa ===");
        println!("1. Aggiungi elemento");
        println!("2. Rimuovi elemento");
        println!("3. Mostra lista");
        println!("4. Esci");
        println!("Scelta: ");

        let mut scelta = String::new();
        io::stdin()
            .read_line(&mut scelta)
            .expect("Errore nella lettura");

        match scelta.trim() {
            "1" => aggiungi(&mut lista),
            "2" => rimuovi(&mut lista),
            "3" => mostra(&lista),
            "4" => {
                println!("Arrivederci!");
                break;
            }
            _ => println!("Scelta non valida"),
        }
    }
}

fn aggiungi(lista: &mut Vec<String>) {
    println!("Cosa vuoi aggiungere?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura");

    let elemento = input.trim().to_string();

    if elemento.is_empty() {
        println!("Non puoi aggiungere un elemento vuoto");
        return
    }

    lista.push(elemento);
    println!("Aggiunto!");
}

fn rimuovi(lista: &mut Vec<String>) {
    if lista.is_empty() {
        println!("La lista è vuota!");
        return;
    }

    mostra(lista);
    println!("Numero elemento da rimuovere: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura");

    match input.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= lista.len() => {
            let rimosso = lista.remove(n -1);
            println!("'{}' rimosso", rimosso);
        }
        _ => println!("Numero non valido"),
    }
}

fn mostra(lista: &Vec<String>) {
    if lista.is_empty() {
        println!("La lista è vuota");
        return;
    }

    println!("\nLa tua lista:");
    for (i, elemento) in lista.iter().enumerate() {
        println!("{}. {}", i + 1, elemento);
    }
}
