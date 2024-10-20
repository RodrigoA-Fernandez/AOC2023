use core::panic;
use std::{collections::HashMap, fs};

fn main() {
    let input = get_input("./input");
    println!("Solucion Parte 1: {}", sol1(&input));
    println!("Solucion Parte 2: {}", sol2(&input));

    // let test = get_input("./test");
    // println!("Solucion Parte 1: {}", sol1(&test));
    // println!("Solucion Parte 2: {}", sol2(&test));
}

fn get_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect("Error: Entrada incorrecta (el archivo no existe).")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn sol1(input: &[Vec<char>]) -> u32 {
    let mut start: (usize, usize) = (0, 0);

    (1..input.len()).for_each(|i| {
        for j in 1..input[i].len() {
            if input[i][j] == 'S' {
                start = (i, j);
            }
        }
    });

    let mut direccion = primer_paso(input, start);
    let mut posicion: (usize, usize) = start;
    let mut distancia: u32 = 0;

    loop {
        distancia += 1;
        // println!("({0},{1})", direccion, input[posicion.0][posicion.1]);
        match direccion {
            'U' => posicion.0 -= 1,
            'D' => posicion.0 += 1,
            'R' => posicion.1 += 1,
            'L' => posicion.1 -= 1,

            _ => panic!("Dirección de movimiento imposible"),
        }

        // println!("({0},{1})", posicion.0 + 1, posicion.1);
        if posicion == start {
            break;
        }
        direccion = match (direccion, input[posicion.0][posicion.1]) {
            ('R', 'J') | ('U', '|') | ('L', 'L') => 'U',
            ('R', '-') | ('U', 'F') | ('D', 'L') => 'R',
            ('R', '7') | ('D', '|') | ('L', 'F') => 'D',
            ('U', '7') | ('D', 'J') | ('L', '-') => 'L',
            _ => panic!("LOL"),
        };
    }
    distancia / 2
}

fn primer_paso(input: &[Vec<char>], start: (usize, usize)) -> char {
    if let '|' | '7' | 'F' = input[start.0 - 1][start.1] {
        return 'U';
    }

    if let '-' | '7' | 'J' = input[start.0][start.1 + 1] {
        return 'R';
    }

    if let '|' | 'L' | 'J' = input[start.0 + 1][start.1] {
        return 'D';
    }

    if let '-' | 'L' | '7' = input[start.0][start.1 - 1] {
        return 'L';
    }

    panic!("El inicio está aislado");
}

fn sol2(input: &[Vec<char>]) -> u32 {
    let mut start: (usize, usize) = (0, 0);

    //Obtener inicio
    (1..input.len()).for_each(|i| {
        for j in 1..input[i].len() {
            if input[i][j] == 'S' {
                start = (i, j);
            }
        }
    });

    let mut direccion = primer_paso(input, start);
    let mut posicion: (usize, usize) = start;
    let mut min_coord: (usize, usize) = start;
    let mut max_coord: (usize, usize) = start;

    let mut mapa: HashMap<(usize, usize), char> = HashMap::new();

    loop {
        // println!("({0},{1})", direccion, input[posicion.0][posicion.1]);
        match direccion {
            'U' => posicion.0 -= 1,
            'D' => posicion.0 += 1,
            'R' => posicion.1 += 1,
            'L' => posicion.1 -= 1,

            _ => panic!("Dirección de movimiento imposible"),
        }

        //Añadir posición al mapa
        mapa.insert(posicion, input[posicion.0][posicion.1]);

        // Crear Bounding Box
        if posicion.0 < min_coord.0 {
            min_coord.0 = posicion.0;
        }
        if posicion.1 < min_coord.1 {
            min_coord.1 = posicion.1;
        }

        if posicion.0 > max_coord.0 {
            max_coord.0 = posicion.0;
        }
        if posicion.1 > max_coord.1 {
            max_coord.1 = posicion.1;
        }

        // println!("({0},{1})", posicion.0 + 1, posicion.1);
        if posicion == start {
            break;
        }
        direccion = match (direccion, input[posicion.0][posicion.1]) {
            ('R', 'J') | ('U', '|') | ('L', 'L') => 'U',
            ('R', '-') | ('U', 'F') | ('D', 'L') => 'R',
            ('R', '7') | ('D', '|') | ('L', 'F') => 'D',
            ('U', '7') | ('D', 'J') | ('L', '-') => 'L',
            _ => panic!("LOL"),
        };
    }

    // println!(
    //     "({0},{1}):({2},{3})",
    //     min_coord.0 + 1,
    //     min_coord.1 + 1,
    //     max_coord.0 + 1,
    //     max_coord.1 + 1
    // );

    // Raycast

    let mut contador_interiores: u32 = 0;
    let mut interiores: Vec<(usize, usize)> = vec![];
    for i in min_coord.0..=max_coord.0 {
        for j in min_coord.1..=max_coord.1 {
            if !mapa.contains_key(&(i, j)) && interior((i, j), &mapa, min_coord, max_coord, false) {
                contador_interiores += 1;
                interiores.push((i, j));
            }
        }
    }

    map_to_file(input, interiores.clone());
    contador_interiores
}

//Eta funcion devuelve true si el punto es interior y falso en caso de que sea interior. Funciona
//usando un ray cast.
fn interior(
    pos: (usize, usize),
    mapa: &HashMap<(usize, usize), char>,
    min_cood: (usize, usize),
    max_coord: (usize, usize),
    invertir: bool,
) -> bool {
    // Por defecto la dirección es hacia arriba
    let mut pos_ray: (usize, usize) = pos;
    let mut tmp_char: Option<char> = None;
    let mut cruces: usize = 0;

    while pos_ray.0 >= min_cood.0 && pos_ray.0 <= max_coord.0 {
        if invertir {
            pos_ray.0 -= 1
        } else {
            pos_ray.0 += 1
        }
        match mapa.get(&pos_ray) {
            Some(c) => match c {
                '|' => {}
                'L' => {
                    // println!("{}", tmp_char.unwrap());

                    if invertir {
                        tmp_char = Some('L');
                    } else if let Some('7') = tmp_char {
                        cruces += 1;
                        tmp_char = None;
                    }
                }
                '7' => {
                    if invertir {
                        if let Some('L') = tmp_char {
                            cruces += 1;
                            tmp_char = None
                        }
                    } else {
                        tmp_char = Some('7');
                    }
                }
                'J' => {
                    if invertir {
                        tmp_char = Some('J');
                    } else if let Some('F') = tmp_char {
                        cruces += 1;
                        tmp_char = None;
                    }
                }
                'F' => {
                    if invertir {
                        if let Some('J') = tmp_char {
                            cruces += 1;
                            tmp_char = None;
                        }
                    } else {
                        tmp_char = Some('F');
                    }
                }
                'S' => {
                    tmp_char = None;
                    return interior(pos, mapa, min_cood, max_coord, true);
                }
                '-' => {
                    cruces += 1;
                    tmp_char = None;
                }
                c => panic!("Valor inválido, {c}"),
            },
            None => {
                tmp_char = None;
                continue;
            }
        }
    }

    (cruces % 2 != 0) && cruces != 0
}

fn map_to_file(input: &[Vec<char>], interiores: Vec<(usize, usize)>) {
    let mut start: (usize, usize) = (0, 0);

    //Obtener inicio
    for i in 1..input.len() {
        for j in 1..input[i].len() {
            if input[i][j] == 'S' {
                start = (i, j);
            }
        }
    }
    let mut direccion = primer_paso(input, start);
    let mut posicion: (usize, usize) = start;
    let mut min_coord: (usize, usize) = start;
    let mut max_coord: (usize, usize) = start;

    let mut mapa: HashMap<(usize, usize), char> = HashMap::new();

    loop {
        // println!("({0},{1})", direccion, input[posicion.0][posicion.1]);
        match direccion {
            'U' => posicion.0 -= 1,
            'D' => posicion.0 += 1,
            'R' => posicion.1 += 1,
            'L' => posicion.1 -= 1,

            _ => panic!("Dirección de movimiento imposible"),
        }

        //Añadir posición al mapa
        mapa.insert(posicion, input[posicion.0][posicion.1]);

        // Crear Bounding Box
        if posicion.0 < min_coord.0 {
            min_coord.0 = posicion.0;
        }
        if posicion.1 < min_coord.1 {
            min_coord.1 = posicion.1;
        }

        if posicion.0 > max_coord.0 {
            max_coord.0 = posicion.0;
        }
        if posicion.1 > max_coord.1 {
            max_coord.1 = posicion.1;
        }

        // println!("({0},{1})", posicion.0 + 1, posicion.1);
        if posicion == start {
            break;
        }
        direccion = match (direccion, input[posicion.0][posicion.1]) {
            ('R', 'J') | ('U', '|') | ('L', 'L') => 'U',
            ('R', '-') | ('U', 'F') | ('D', 'L') => 'R',
            ('R', '7') | ('D', '|') | ('L', 'F') => 'D',
            ('U', '7') | ('D', 'J') | ('L', '-') => 'L',
            _ => panic!("LOL"),
        };
    }

    let mut cadena_mapa = String::new();

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            cadena_mapa += match mapa.get(&(i, j)) {
                Some('J') => "┘",
                Some('-') => "─",
                Some('|') => "│",
                Some('L') => "└",
                Some('7') => "┐",
                Some('F') => "┌",
                Some('S') => "S",
                None => {
                    if interiores.contains(&(i, j)) {
                        "X"
                    } else {
                        " "
                    }
                }
                Some(_) => panic!("No deberias estar aquí."),
            };
        }
        cadena_mapa += "\n";
    }
    match fs::write("./mapa", cadena_mapa) {
        Ok(_) => println!("Escrito"),
        Err(_) => println!("No escrito"),
    };
}
