use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Partida {
    pasos: Vec<Direccion>,
    mapa: HashMap<String, Direccion>,
}

impl Partida {
    fn new(path: String) -> Partida {
        Partida {
            pasos: vec![],
            mapa: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum Direccion {
    L,
    R,
}

#[derive(Debug)]
struct Interseccion {
    left: String,
    right: String,
}
