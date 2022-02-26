use crate::person::Person;
use crate::storage::append_to_storage;

pub struct Repository {
    person: Vec<Person>
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            person: Vec::new()
        }
    }

    pub unsafe fn fill_person(&mut self) {
        self.person = Person::create_person_vector();
    }

    pub fn get_all(&mut self) -> &Vec<Person> {
        &self.person
    }

    pub fn get_by_id(&mut self, id: i16) -> Result<&mut Person, &'static str> {
        for person in &mut self.person {
            if person.get_id() == id {
                return Ok(person);
            }
        }

        Err("Pessoa não encontrada!")
    }

    pub fn update_by_id(target: &mut Person, source: Person) {
        target.set_name(source.get_name().to_string());
        target.set_age(source.get_age());
        target.set_salary(source.get_salary());
        target.set_position(source.get_position().to_string());

        println!("Registro Atualizado!");
    }

    pub unsafe fn delete_by_id(target: &mut Person) {
        target.set_age(0);
        target.set_name("".to_string());
        target.set_salary(0.0);
        target.set_position("".to_string());

        println!("Registro apagado!");
    }

    pub unsafe fn create_person(p: Person) {
        append_to_storage(&p);
    }
}