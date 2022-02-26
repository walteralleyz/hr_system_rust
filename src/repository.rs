use crate::person::Person;
use crate::person::append_to_storage;

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

    pub fn get_by_id(&self, id: String) -> &Person {
        for person in &self.person {
            if person.get_id().to_string() == id.trim() {
                return &person;
            }
        }

        panic!("Pessoa não encontrada!");
    }

    pub unsafe fn create_person(&mut self, p: Person) {
        append_to_storage(&p);
    }

    pub fn get_person_size(&self) -> usize {
        self.person.len()
    }
}