use std::{cmp::Ordering, fs, path::Path};

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
    cartas: Vec<Carta>,
    tipo: TipoMano,
    apuesta: u64,
}

impl Mano {
    pub fn new(cad: &str) -> Result<Self, ()> {
        let mut cartas: Vec<Carta> = vec![];
        let split: Vec<&str> = cad.split(" ").collect();
        let mut contadores: Vec<(char, u8)> = vec![];

        for c in split[0].chars() {
            cartas.push(Carta::try_from(c)?);
            let mut contenido = false;
            for tupla in &mut contadores {
                if tupla.0 == c {
                    tupla.1 += 1;
                    contenido = true;
                }
            }
            if !contenido {
                contadores.push((c, 1));
            }
        }

        let apuesta: u64 = split[1].parse::<u64>().unwrap();

        contadores.sort_by(|a, b| b.1.cmp(&a.1));

        let tipo = match contadores[0].1 {
            5 => TipoMano::FiveOfAKind,
            4 => TipoMano::FourOfAKind,
            3 => match contadores[1].1 {
                2 => TipoMano::Full,
                1 => TipoMano::ThreeOfAKind,
                _ => return Err(()),
            },
            2 => match contadores[1].1 {
                2 => TipoMano::TwoPair,
                1 => TipoMano::OnePair,
                _ => return Err(()),
            },
            1 => TipoMano::HighCard,
            _ => return Err(()),
        };

        Ok(Mano {
            cartas,
            tipo,
            apuesta,
        })
    }
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

#[derive(PartialEq, Eq, Debug)]
enum Carta {
    Int(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Carta {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_digit(10) {
            Some(i) => match i {
                2 => Ok(Carta::Int(1)),
                3 => Ok(Carta::Int(1)),
                4 => Ok(Carta::Int(1)),
                5 => Ok(Carta::Int(1)),
                6 => Ok(Carta::Int(1)),
                7 => Ok(Carta::Int(1)),
                8 => Ok(Carta::Int(1)),
                9 => Ok(Carta::Int(1)),
                _ => Err(()),
            },
            None => match value {
                'T' => Ok(Carta::T),
                'J' => Ok(Carta::J),
                'Q' => Ok(Carta::Q),
                'K' => Ok(Carta::K),
                'A' => Ok(Carta::A),
                _ => Err(()),
            },
        }
    }
}

impl PartialOrd for Carta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let other_num: u8 = match other {
            Carta::Int(u) => *u,
            Carta::T => 10,
            Carta::J => 11,
            Carta::Q => 12,
            Carta::K => 13,
            Carta::A => 14,
        };

        let self_num: u8 = match self {
            Carta::Int(u) => *u,
            Carta::T => 10,
            Carta::J => 11,
            Carta::Q => 12,
            Carta::K => 13,
            Carta::A => 14,
        };

        u8::partial_cmp(&self_num, &other_num)
    }
}

impl Ord for Carta {
    fn cmp(&self, other: &Self) -> Ordering {
        Carta::partial_cmp(&self, other).expect("No se como narices has llegado aqui.")
    }
}
