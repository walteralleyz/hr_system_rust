use crate::utils;

pub struct Presenter {
    command: String,
    options: Vec<&'static str>
}

impl Presenter {
    pub fn new() -> Self {
        Presenter {
            command: String::from("INICIO"),
            options: vec!["Sair", "Buscar", "Atualizar", "Cadastrar", "Apagar"]
        }
    }

    pub fn show_menu(&mut self) {
        println!("{}[2J", 27 as char);
        println!("============INICIO");
        
        for (index, option) in self.options.iter().enumerate() {
            println!("|{} - {}", index, option);
        }

        println!("==================");

        let option = utils::get_usize_param("Número da Opção");

        if option > self.options.len()-1 { panic!("Opção inválida!") }

        self.command = String::from(self.options[option]);
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }
}