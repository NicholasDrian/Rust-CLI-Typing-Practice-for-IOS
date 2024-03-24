mod app;

use app::App;

use std::env; // cli args

fn main() {
    let app = App::new(env::args().collect());
    app.run()
}
