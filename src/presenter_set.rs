pub struct Set {
    title: String,
    options: Vec<Option>
}

pub struct Option {
    index: i16,
    text: &'static str
}

impl Set {

    pub fn get_sets() -> Vec<Self> {
        let mut vec_set: Vec<Set> = Vec::new();
        let mut count = 0;

        for title in Set::title_list() {
            let mut vec_option: Vec<Option> = Vec::new();
            let mut index = 0;

            for text in &Set::option_list()[count] {
                vec_option.push(Option {
                    index,
                    text
                });

                index += 1;
            }

            vec_set.push(Set {
                title: title.to_string(),
                options: vec_option
            });

            count += 1;
        }

        vec_set
    }

    pub fn option_list() -> Vec<Vec<&'static str>> {
        vec![
            vec!["Sair", "Buscar", "Atualizar", "Apagar", "Cadastrar"],
            vec!["Voltar", "Buscar Por ID", "Buscar Por nome", "Buscar Por idade", "Buscar Por Cargo"],
            vec!["Voltar", "Atualizar Por ID", "Atualizar Por nome", "Atualizar Por idade"],
            vec!["Voltar", "Apagar Por ID"],
            vec!["Voltar", "Cadastrar Funcionário", "Cadastrar Gestor"]
        ]
    }

    pub fn title_list() -> Vec<&'static str> {
        vec!["Start", "Buscar", "Atualizar", "Apagar", "Cadastrar"]
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_options(&self) -> &Vec<Option> {
        &self.options
    }
}

impl Option {
    pub fn get_index(&self) -> i16 {
        self.index
    }

    pub fn get_text(&self) -> String {
        String::from(self.text)
    }
}