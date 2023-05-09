use std::fs::File;
use std::path::Path;

mod ui;
mod controllers;
#[derive(Debug, Copy, Clone)]
enum State{
SplashState,
MenuState,
Exit,
}
fn main() {
    let mut state = State::SplashState;
    loop {
        state = state_handler(state);
    }
}
fn state_handler(state: State) -> State {
    let mut new_state = state.clone();
    match new_state {
        State::SplashState => {
            ui::components::splash::run();
            new_state = State::MenuState;
        }
        State::MenuState => {
            ui::components::menu::run();
            new_state = State::Exit;
        }
        State::Exit => {
            std::process::exit(0);
        }
    }
    new_state
}

fn read_test_file() {
    let file_path = Path::new("test_files/test.RXDATA");
    let file = File::open(&file_path).expect("Could not open file");

    let mut reader = controllers::reader::Reader::new(file);

    match reader.parse() {
        Ok(_) => println!("File parsed successfully"),
        Err(e) => println!("Error: {}", e),
    }

}