#[derive(Debug)]

struct Edificio {
    indirizzo: String,
    piani: i32,
    anno_costruzione: i32,
}

impl Edificio {
    fn descrizione(&self) {
        println!("indirizzo: {}, piani: {}, anno costruzione: {}",
        self.indirizzo, self.piani, self.anno_costruzione)
    }

    fn nuovo(indirizzo: String, piani: i32, anno_costruzione: i32) -> Self {
        Self {
            indirizzo: indirizzo,
            piani: piani,
            anno_costruzione: anno_costruzione,
        }
    }

}

fn main() {
    let ed = Edificio::nuovo(
        String::from("Via Tuscolana 1026, Roma"),
        5,
        1960,
    );
    ed.descrizione();
}
