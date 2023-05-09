use std::process::Command;


mod ui;
mod controllers;
#[derive(Debug, Copy, Clone)]
enum State{
SplashState,
MenuState,
Exit,
}
fn main() {
    convert_marshal_to_json("src/test_files/test.rxdata", "src/test_files/test.json");
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

fn convert_marshal_to_json(input_filename: &str, output_filename: &str) {

    let output = Command::new("ruby")
        .arg("rxToJSON.rb")
        .arg(input_filename)
        .arg(output_filename)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Successfully converted file.");
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", s);
    }
}