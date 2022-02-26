mod presenter;
mod commander;
mod repository;
mod person;
mod utils;
mod storage;

use crate::presenter::Presenter;
use crate::commander::Commander;

fn main() {
    let mut show = Presenter::new();
    let mut command = Commander::new();

    unsafe {
        loop {
            show.show_menu();

            match show.get_command().as_str() {
                "Sair" => break,
                &_ => command.resolve_command(show.get_command())
            }
        }
    }
    
}