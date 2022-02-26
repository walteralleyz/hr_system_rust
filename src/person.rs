use std::fmt;

static mut STORAGE: String = String::new();

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

pub struct Person {
    id: i16,
    name: String,
    age: i8,
    job_role: Worker
}

pub struct Worker {
    salary: f64,
    position: String
}

impl Person {
    pub fn new(id: i16, name: String, age: i8, salary: f64, position: String) -> Self {
        Person {
            id, name, age, job_role: Worker { salary, position }
        }
    }

    pub fn create_by_content(content: &'static str) -> Self {
        let data: Vec<&str> = content.split(" ").collect();
        let mut id = 0;
        let mut name = String::new();
        let mut age = 0;
        let mut job_role = Worker {
            salary: 0.0,
            position: String::new()
        };

        for info in data {
            let tags: Vec<&str> = info.trim().split(":").collect();

            if tags[0] == "id" {
                id = match tags[1].trim().parse() {
                    Ok(res) => res,
                    Err(_) => 0
                }
            }

            else if tags[0] == "name" {
                name = String::from(tags[1]);
            }

            else if tags[0] == "age" {
                age = match tags[1].trim().parse() {
                    Ok(res) => res,
                    Err(_) => 0
                };
            }

            else if tags[0] == "job_role" {
                let job_info: Vec<&str> = tags[1].split(",").collect();

                job_role = Worker {
                    salary: match job_info[0].trim().parse() {
                        Ok(res) => res,
                        Err(_) => 0.0
                    },

                    position: String::from(job_info[1])
                }
            }
        }

        Person { id, name, age, job_role }
    }

    pub unsafe fn create_person_vector() -> Vec<Person> {
        let data: Vec<&str> = STORAGE.split("===PERSON").collect();
        let mut person_vector = Vec::new();

        if !data.is_empty() {
            for info in data {
                let temp: &'static str = info.trim();

                if !temp.is_empty() {
                    person_vector.push(Person::create_by_content(temp));
                }
                
            }
        }

        person_vector
    }

    pub fn get_id(&self) -> i16 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_age(&self) -> i8 {
        self.age
    }

    pub fn get_salary(&self) -> f64 {
        self.job_role.salary
    }

    pub fn get_position(&self) -> &String {
        &self.job_role.position
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "===DADOS FUNCIONARIO\nid: {},\nname: {},\nage: {},\nsalary: {},\nposition: {}", 
        self.id, self.name, self.age, self.job_role.salary, self.job_role.position)
    }
}