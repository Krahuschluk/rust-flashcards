mod app;
mod db;

use iced::{Settings, Application};
use app::FlashcardApp;

fn main() {
    FlashcardApp::run(Settings::default());
}



