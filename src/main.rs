mod app;
mod db;

use iced::{Settings, Application};
use app::FlashcardApp;

fn main() {
    FlashcardApp::run(Settings {
        default_font: Some(include_bytes!("../fonts/BMYEONSUNG_ttf.ttf")),
        ..Settings::default()
    });
}
