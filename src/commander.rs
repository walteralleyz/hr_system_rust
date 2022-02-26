use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;
use crate::repository::Repository;
use crate::person::Person;

pub struct Commander {
    repository: Repository
}

impl Commander {
    pub fn new() -> Self {
        Commander {
            repository: Repository::new()
        }
    }

    pub unsafe fn resolve_command(&mut self, command: &str) {
        let first_word: &str = command.split(" ").collect::<Vec<&str>>()[0];

        self.repository.fill_person();

        match first_word.trim() {
            "Buscar" => self.resolve_search_command(command),
            "Atualizar" => self.resolve_update_command(command),
            "Apagar" => self.resolve_delete_command(command),
            "Cadastrar" => self.resolve_create_command(command),
            &_ => panic!("Comando não encontrado!")
        }
    }

    pub fn resolve_search_command(&self, command: &str) {
        let by: Vec<&str> = command.split(" ").collect();

        if by.contains(&"ID") {
            let param = Commander::get_string_param("ID");

            println!("{}", self.repository.get_by_id(param));
            sleep(Duration::from_secs(3));
        }
    }

    pub fn resolve_update_command(&self, command: &str) {

    }

    pub fn resolve_delete_command(&self, command: &str) {

    }

    pub unsafe fn resolve_create_command(&mut self, command: &str) {
        let by: Vec<&str> = command.split(" ").collect();

        if by.contains(&"Funcionário") || by.contains(&"Gestor") {
            let name = Commander::get_string_param("Nome");
            let age = Commander::get_i8_param("Idade");
            let salary = Commander::get_f64_param("Salario");

            self.repository.create_person(Person::new(
                self.repository.get_person_size() as i16,
                name,
                age,
                salary,
                if command == "Cadastrar Funcionário" { String::from("FUNCIONARIO") } 
                else { String::from("GESTOR") } 
            ))
        }
    }

    fn get_string_param(field: &str) -> String {
        println!("Digite o {}:", field);
        let mut param = String::new();

        stdin().read_line(&mut param).expect("Problema ao coletar entrada");

        param
    }

    fn get_i8_param(field: &str) -> i8 {
        match Commander::get_string_param(field).trim().parse() {
            Ok(res) => res,
            Err(_) => 0
        }
    }

    fn get_f64_param(field: &str) -> f64 {
        match Commander::get_string_param(field).trim().parse() {
            Ok(res) => res,
            Err(_) => 0.0
        }
    }
}