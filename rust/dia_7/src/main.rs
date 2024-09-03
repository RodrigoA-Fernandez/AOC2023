use std::{fs, path::Path};

fn main() {}

#[derive(Debug)]
struct Partida {
    manos: Vec<Mano>,
}

impl Partida {
    pub fn new(path: &Path) -> Result<Self, String> {
        let contenido = fs::read_to_string(path).expect("No se ha podido leer la entrada");
        let manos: Vec<Mano> = vec![];

        for linea in contenido.lines() {
            let info: Vec<&str> = linea.split(" ").collect();
            if info.len() != 2 || info[0].len() != 5 {
                return Err(String::from("Formato de archivo incorrecto."));
            }

            for c in info[0].chars() {}
        }

        Ok(Partida { manos })
    }
}

#[derive(Debug)]
struct Mano {
    cartas: Vec<u8>,
    tipo: TipoMano,
    apuesta: u64,
}

impl Mano {
    pub fn new(cad)
}

#[derive(Debug)]
enum TipoMano {
    FiveOfAKind,
    FourOfAKind,
    Full,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

enum Carta {
    Int(u8),
    T,
    J,
    Q,
    K,
    A,
}
