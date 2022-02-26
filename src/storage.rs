use crate::person::Person;

pub static mut STORAGE: String = String::new();

pub unsafe fn append_to_storage(p: &Person) {
    STORAGE.push_str("===PERSONid:");
    STORAGE.push_str(p.get_id().to_string().as_str());
    STORAGE.push_str(" name:");
    STORAGE.push_str(p.get_name());
    STORAGE.push_str(" age:");
    STORAGE.push_str(p.get_age().to_string().as_str());
    STORAGE.push_str(" job_role:");
    STORAGE.push_str(p.get_salary().to_string().as_str());
    STORAGE.push_str(",");
    STORAGE.push_str(p.get_position());
}

pub unsafe fn erase_storage() {
    STORAGE = String::new();
}

pub unsafe fn fill_storage_by_person_vector(person_vector: &Vec<Person>) {
    erase_storage();

    for person in person_vector {
        append_to_storage(person);
    }
}