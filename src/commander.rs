use std::thread::sleep;
use std::time::Duration;
use crate::repository::Repository;
use crate::person::Person;
use crate::utils;
use crate::storage::fill_storage_by_person_vector;

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
        self.repository.fill_person();
        fill_storage_by_person_vector(self.repository.get_all());

        match command.trim() {
            "Buscar" => self.resolve_search_command(),
            "Atualizar" => self.resolve_update_command(),
            "Apagar" => self.resolve_delete_command(),
            "Cadastrar" => self.resolve_create_command(),
            &_ => panic!("Comando não encontrado!")
        };

        sleep(Duration::from_secs(3));
    }

    pub fn resolve_search_command(&mut self) {
        let param = utils::get_i16_param("ID");

        match self.repository.get_by_id(param) {
            Ok(res) => println!("{}", res),
            Err(msg) => println!("{}", msg)
        }
    }

    pub fn resolve_update_command(&mut self) {
        let param = utils::get_i16_param("ID");
        let repo = &mut self.repository;

        match repo.get_by_id(param) {
            Ok(res) => {
                let p = Commander::create_new_person(0);
                Repository::update_by_id(res, p);
            },
            Err(msg) => println!("{}", msg)
        }
    }

    pub unsafe fn resolve_delete_command(&mut self) {
        let param = utils::get_i16_param("ID");

        match self.repository.get_by_id(param) {
            Ok(res) => Repository::delete_by_id(res),
            Err(msg) => println!("{}", msg)
        }
    }

    pub unsafe fn resolve_create_command(&mut self) {
        Repository::create_person(
            Commander::create_new_person(self.repository.get_all().len() as i16));
    }

    fn create_new_person(id: i16) -> Person {
        let name = utils::get_string_param("Nome");
        let age = utils::get_i8_param("Idade");
        let salary = utils::get_f64_param("Salario");
        let position = utils::get_string_param("Função");

        Person::new(
            id,
            name,
            age,
            salary,
            position
        )
    }
}