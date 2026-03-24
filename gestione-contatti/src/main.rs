use std::io;

// Struct - defines features of a contact
struct Contatto {
    nome: String,
    email: String,
    telefono: String,
}

impl Contatto {
    // Constructor
    fn nuovo(nome: String, email: String, telefono: String) -> Contatto {
        Contatto { nome, email, telefono }
    }

    // Takes immutable reference - doesn't edit
    fn stampa(&self) {
        println!("Nome:     {}", self.nome);
        println!("Email:    {}", self.email);
        println!("Telefono: {}", self.telefono);
        println!("----------");
    }
}

fn main() {
    let mut rubrica: Vec<Contatto> = Vec::new();

    loop {
        println!("\n=== GEstione Contatti ===");
        println!("1. Aggiungi contatto");
        println!("2. Cerca contatto");
        println!("3. Mostra tutti");
        println!("4. Elimina contatto");
        println!("5. Esci");
        println!("Scelta: ");

        let mut scelta = String::new();
        io::stdin()
            .read_line(&mut scelta)
            .expect("Errore nella lettura");

        match scelta.trim() {
            "1" => aggiungi_contatto(&mut rubrica),
            "2" => cerca_contatto(&rubrica),
            "3" => mostra_tutti(&rubrica),
            "4" => elimina_contatto(&mut rubrica),
            "5" => {
                println!("Arrivederci!");
                break;
            }
            _ => println!("Scelta non valida"),
        }
    }
}

fn leggi_input(messaggio: &str) -> String {
    println!("{}", messaggio);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura");
    input.trim().to_string()
}

fn aggiungi_contatto(rubrica: &mut Vec<Contatto>) {
    let nome = leggi_input("Nome:");
    let email = leggi_input("Email:");
    let telefono = leggi_input("Telefono:");

    if nome.is_empty() {
        println!("Il nome non può essere vuoto");
        return;
    }

    rubrica.push(Contatto::nuovo(nome, email, telefono));
    println!("Contatto aggiunto!");
}

fn cerca_contatto(rubrica: &Vec<Contatto>) {
    if rubrica.is_empty() {
        println!("La rubrica è vuota");
        return;
    }

    let termine = leggi_input("Cerca per nome:");

    // iter() gives back immutable reference - borrowing vs ownership
    let risultati: Vec<&Contatto> = rubrica
        .iter()
        .filter(|c| c.nome.to_lowercase().contains (&termine.to_lowercase()))
        .collect();

    if risultati.is_empty() {
        println!("Nessun risultato trovato");
        return;
    }

    println!("\nRisultati:");
    for contatto in risultati {
        contatto.stampa();       
    }
}

fn mostra_tutti(rubrica: &Vec<Contatto>) {
    if rubrica.is_empty() {
        println!("La rubrica è vuota");
        return;
    }

    println!("\nTutti i contatti ({})", rubrica.len());
    for (i, contatto) in rubrica.iter().enumerate() {
        println!("\n[{}]", i + 1);
        contatto.stampa();
    }
}

fn elimina_contatto(rubrica: &mut Vec<Contatto>) {
    if rubrica.is_empty() {
        println!("La rubrica è vuota");
        return;
    }

    mostra_tutti(rubrica);
    let input = leggi_input("Numero contatto da eliminare:");

    match input.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= rubrica.len() => {
            let rimosso = rubrica.remove(n - 1);
            println!("'{}' eliminato", rimosso.nome);
        }
        _ => println!("Numero non valido"),
    }
}