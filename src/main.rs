mod frontend;
mod backend;
mod database;

use frontend::app::App;

fn main() {
    let app = App::new(55);
    app.run();
}