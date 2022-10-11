use std::time::Duration;
use app::App;
use log::info;
use sport_core::config::{self, Config, KeyMap};
mod app;

fn main() {
    info!("Client started");

    let cfg: Config = config::Config::load().unwrap();
    let keymap = KeyMap::default();

    let mut app = App::new(cfg, keymap);
    app.run(Duration::from_millis(500));
}
