mod presenter;
mod presenter_set;
mod commander;
mod repository;
mod person;

fn main() {
    let mut show = presenter::Presenter::new();
    let mut command = commander::Commander::new();

    unsafe {
        loop {
            show.show_menu();

            match show.get_command().as_str() {
                "Sair" => break,
                "Voltar" => show.go_back_page(),
                &_ => command.resolve_command(show.get_command())
            }
        }
    }
    
}