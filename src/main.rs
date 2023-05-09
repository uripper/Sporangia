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
