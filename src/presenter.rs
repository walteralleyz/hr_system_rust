use std::io::stdin;
use crate::presenter_set::Set;

pub struct Presenter {
    page: usize,
    command: String,
    sets: Vec<Set>
}

impl Presenter {
    pub fn new() -> Self {
        Presenter {
            page: 0,
            command: String::from("Start"),
            sets: Set::get_sets()
        }
    }

    pub fn show_menu(&mut self) {
        print!("{}[2J", 27 as char);
        println!("==================");
        println!("|{}", self.sets[self.page].get_title());

        for option in self.sets[self.page].get_options() {
            println!("|{0} - {1}", option.get_index(), option.get_text());
        }

        println!("==================");

        self.change_page_by_user_option();
    }

    pub fn change_page_by_user_option(&mut self) {
        let mut user_page = String::new();

        stdin().read_line(&mut user_page).expect("Deve ser um valor numérico");

        let option = self.get_string_as_usize(user_page);

        self.command = self.sets[self.page].get_options()[option].get_text();
        self.page = option;
    }

    pub fn get_string_as_usize(&self, val: String) -> usize {
        match val.trim().parse() {
            Ok(res) => res,
            Err(_) => 0
        }
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn go_back_page(&mut self) {
        self.page = 0;
    }
}