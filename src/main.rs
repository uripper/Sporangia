mod frontend;
mod backend;
mod database;

use frontend::app::App;

fn main() {
    let app = App::new();
    app.run();
}