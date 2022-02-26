use std::io::stdin;

pub fn get_string_param(field: &str) -> String {
    println!("Digite o {}:", field);
    let mut param = String::new();

    stdin().read_line(&mut param).expect("Problema ao coletar entrada");

    param
}

pub fn get_i8_param(field: &str) -> i8 {
    match get_string_param(field).trim().parse() {
        Ok(res) => res,
        Err(_) => 0
    }
}

pub fn get_i16_param(field: &str) -> i16 {
    match get_string_param(field).trim().parse() {
        Ok(res) => res,
        Err(_) => 0
    }
}

pub fn get_f64_param(field: &str) -> f64 {
    match get_string_param(field).trim().parse() {
        Ok(res) => res,
        Err(_) => 0.0
    }
}

pub fn get_usize_param(field: &str) -> usize {
    match get_string_param(field).trim().parse() {
        Ok(res) => res,
        Err(_) => 0
    }
}