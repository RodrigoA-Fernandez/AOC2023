use std::{collections::HashMap, fs, io::BufRead};

fn main() {
    let input = parse_input("./input");
    println!("Parte 1: {}", sol_1(&mut input.clone()));
    println!("Parte 2: {}", sol_2(&mut input.clone()));

    let test = parse_input("./test");
    println!("Parte 1: {}", sol_1(&mut test.clone()));
    println!("Parte 2: {}", sol_2(&mut test.clone()));
}

fn parse_input(path: &str) -> Vec<Vec<char>> {
    fs::read(path)
        .expect("Archivo de entrada inválido")
        .lines()
        .map(|l| {
            l.expect("Formato de entrada incorrecto")
                .chars()
                .collect::<Vec<char>>()
        })
        .collect()
}

fn sol_1(input: &mut Vec<Vec<char>>) -> i64 {
    computar_expansion(input, 1);
    let mut mapa: Vec<(i64, i64)> = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '#' {
                mapa.push((i as i64, j as i64));
            }
        }
    }
    // for g in &mapa {
    //     println!("({},{})", g.0 + 1, g.1 + 1);
    // }

    let mut sum = 0;
    for i in 0..mapa.len() {
        for j in (i + 1)..mapa.len() {
            sum += distancia_manhattan(mapa[i], mapa[j]);
        }
    }

    sum
}

fn sol_2(input: &mut Vec<Vec<char>>) -> i64 {
    let (filas_vacias, columnas_vacias) = expansion_extrema(input);
    let lambda = 10;
    let mut mapa: Vec<(i64, i64)> = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '#' {
                mapa.push((i as i64, j as i64));
            }
        }
    }

    println!("{}", filas_vacias.len());
    println!("{}", columnas_vacias.len());

    let mut sum = 0;
    for i in 0..mapa.len() {
        for j in (i + 1)..mapa.len() {
            sum += distancia_manhattan_extrema(
                mapa[i],
                mapa[j],
                &columnas_vacias,
                &filas_vacias,
                lambda,
            );
        }
    }

    sum
}

fn expansion_extrema(input: &Vec<Vec<char>>) -> (std::vec::Vec<i64>, std::vec::Vec<i64>) {
    let mut columnas_vacias: Vec<bool> = vec![true; input.len()];
    let mut filas_vacias: Vec<bool> = vec![true; input.len()];
    for i in 0..input.len() {
        (0..input[0].len()).for_each(|j| {
            if input[i][j] != '.' {
                if columnas_vacias[j] {
                    columnas_vacias[j] = false;
                }
                if filas_vacias[i] {
                    filas_vacias[i] = false;
                }
            }
        });
    }

    // println!(
    //     "{}",
    //     filas_vacias
    //         .clone()
    //         .into_iter()
    //         .filter(|f| *f == true)
    //         .count()
    // );

    let mut cols: Vec<i64> = vec![];
    let mut filas: Vec<i64> = vec![];

    (0..columnas_vacias.len()).for_each(|i| {
        if columnas_vacias[i] {
            cols.push(i as i64);
        }
    });
    (0..filas_vacias.len()).for_each(|i| {
        if filas_vacias[i] {
            filas.push(i as i64);
        }
    });

    (filas, cols)
}

fn computar_expansion(input: &mut Vec<Vec<char>>, lambda: usize) {
    let mut columnas_vacias: Vec<bool> = vec![true; input.len()];
    let mut filas_vacias: Vec<bool> = vec![true; input.len()];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] != '.' {
                if columnas_vacias[j] {
                    columnas_vacias[j] = false;
                }
                if filas_vacias[i] {
                    filas_vacias[i] = false;
                }
            }
        }
    }
    for i in (0..input[0].len()).rev() {
        if columnas_vacias[i] {
            for v in &mut *input {
                for _ in 0..lambda {
                    v.insert(i, '.')
                }
            }
        }
    }

    for i in (0..input.len()).rev() {
        if filas_vacias[i] {
            for _ in 0..lambda {
                input.insert(i, vec!['.'; input[0].len()])
            }
        }
    }
}

fn distancia_manhattan(x: (i64, i64), y: (i64, i64)) -> i64 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}
fn distancia_manhattan_extrema(
    x: (i64, i64),
    y: (i64, i64),
    columnas_expandidas: &[i64],
    filas_expandidas: &Vec<i64>,
    lambda: i64,
) -> i64 {
    let (col_min, col_max) = (x.0.min(y.0), x.0.max(y.0));
    let (row_min, row_max) = (x.1.min(y.1), x.1.max(y.1));

    let delta_col = col_max - col_min;
    let delta_row = row_max - row_min;

    let col_exp = columnas_expandidas
        .iter()
        .filter(|&col| *col >= col_min && *col <= col_max)
        .count() as i64;
    let row_exp = filas_expandidas
        .iter()
        .filter(|&row| *row >= row_min && *row <= row_max)
        .count() as i64;
    // println!("{},{}", col_exp, row_exp);

    delta_col + col_exp * (lambda - 1) + delta_row + row_exp * (lambda - 1)
}
