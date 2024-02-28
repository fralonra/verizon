mod app;

use app::App;

pub fn run() {
    let mut app = App::new();

    match app.init() {
        Ok(()) => {
            if let Err(err) = app::run(&app) {
                log::error!("Failed to run the application: {}", err);
            }
        }
        Err(err) => {
            log::error!("Failed to initialize the application: {}", err);
        }
    };
}
