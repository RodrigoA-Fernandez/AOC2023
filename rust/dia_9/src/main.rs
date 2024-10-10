use std::fs;

fn main() {
    let vector = get_input("./input".to_string());
    println!("Solucion Parte 1: {0}", sol1(vector.clone()));
    println!("Solucion Parte 2: {0}", sol2(vector.clone()));
}

fn get_input(input: String) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(input).expect("Error: Input incorrecto.");

    input
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|n| n.parse::<i32>().expect("Error: Input Incorrecto"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn sol1(vector: Vec<Vec<i32>>) -> i32 {
    let mut estimaciones: Vec<i32> = vec![];
    for v in vector {
        estimaciones.push(estimar(&v));
    }

    estimaciones.iter().sum()
}

fn estimar(vector: &[i32]) -> i32 {
    let mut nuevo_vector = vec![];

    for v in vector {
        // print!("{v} ");
    }
    // println!();

    for i in 0..(vector.len() - 1) {
        nuevo_vector.push(vector[i + 1] - vector[i]);
    }

    let mut band = true;
    for n in &nuevo_vector {
        if *n != 0 {
            band = false;
            break;
        }
    }

    // println!("{}", vector.last().unwrap());
    if band {
        // println!("{0}", *nuevo_vector.last().expect("Vacio?"));
        return *vector.last().expect("Vacio?");
    }

    vector.last().expect("Vacío?") + estimar(&nuevo_vector)
}

fn sol2(vector: Vec<Vec<i32>>) -> i32 {
    let mut estimaciones: Vec<i32> = vec![];
    for v in vector {
        estimaciones.push(estimar_hacia_atras(&v));
    }

    estimaciones.iter().sum()
}

fn estimar_hacia_atras(vector: &[i32]) -> i32 {
    let mut nuevo_vector = vec![];

    for v in vector {
        // print!("{v} ");
    }
    // println!();

    for i in 0..(vector.len() - 1) {
        nuevo_vector.push(vector[i + 1] - vector[i]);
    }

    let mut band = true;
    for n in &nuevo_vector {
        if *n != 0 {
            band = false;
            break;
        }
    }

    // println!("{}", vector.last().unwrap());
    if band {
        // println!("{0}", *nuevo_vector.last().expect("Vacio?"));
        return *vector.first().expect("Vacio?");
    }

    vector.first().expect("Vacío?") - estimar_hacia_atras(&nuevo_vector)
}
