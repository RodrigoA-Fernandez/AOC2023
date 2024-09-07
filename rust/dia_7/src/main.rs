use std::{cmp::Ordering, fmt::Display, fs, path::Path};

fn main() {
    ej1()
}

fn ej1() {
    let mut partida = Partida::new(Path::new("./input/input")).expect("Datos incorrectos.");
    partida.manos.sort();
    for l in partida.manos {
        let mut s = "".to_string();
        for c in l.cartas {
            s.push_str(&c.to_string());
        }
        println!("{}: {}. {}", s, l.apuesta, l.tipo);
    }
}

#[derive(Debug)]
struct Partida {
    manos: Vec<Mano>,
}

impl Partida {
    pub fn new(path: &Path) -> Result<Self, ()> {
        let contenido = fs::read_to_string(path).expect("No se ha podido leer la entrada");
        let mut manos: Vec<Mano> = vec![];

        for linea in contenido.lines() {
            manos.push(Mano::new(linea)?);
        }

        Ok(Partida { manos })
    }
}

#[derive(Debug, PartialEq, Eq)]
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

impl Ord for Mano {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.tipo != other.tipo {
            return isize::cmp(&(self.tipo as isize), &(other.tipo as isize));
        };

        let mut cartas_self = self.cartas.clone();
        let mut cartas_other = other.cartas.clone();

        cartas_self.sort();
        cartas_other.sort();

        for c1 in &cartas_self {
            for c2 in &cartas_other {
                if *c1 != *c2 {
                    return Carta::cmp(c1, c2);
                }
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Mano {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TipoMano {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    Full = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Display for TipoMano {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TipoMano::FiveOfAKind => "FiveOfAKind",
            TipoMano::FourOfAKind => "FourOfAKind",
            TipoMano::Full => "Full",
            TipoMano::ThreeOfAKind => "ThreeOfAKind",
            TipoMano::TwoPair => "TwoPair",
            TipoMano::OnePair => "OnePair",
            TipoMano::HighCard => "HighCard",
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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
                2 => Ok(Carta::Int(2)),
                3 => Ok(Carta::Int(3)),
                4 => Ok(Carta::Int(4)),
                5 => Ok(Carta::Int(5)),
                6 => Ok(Carta::Int(6)),
                7 => Ok(Carta::Int(7)),
                8 => Ok(Carta::Int(8)),
                9 => Ok(Carta::Int(9)),
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

impl Ord for Carta {
    fn cmp(&self, other: &Self) -> Ordering {
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

        u8::cmp(&self_num, &other_num)
    }
}

impl PartialOrd for Carta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Carta::cmp(self, other))
    }
}

impl Display for Carta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Carta::Int(u) => &u.to_string(),
            Carta::T => "T",
            Carta::J => "J",
            Carta::Q => "Q",
            Carta::K => "K",
            Carta::A => "A",
        };
        write!(f, "{}", s)
    }
}
