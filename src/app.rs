
use iced::{button, Align};
use iced::{Button, Column, Text, Element, Application, executor, Command};
use crate::db::Database;
use crate::db::entities::Cat;

// #[derive(Default)]
pub struct FlashcardApp {
    value: usize,
    db: Database,
    cats: Vec<Cat>,

    next_button: button::State,
    previous_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NextPressed,
    PreviousPressed,
}

impl Application for FlashcardApp {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();


    fn new(_flags: ()) -> (FlashcardApp, Command<Self::Message>) {
        let mut db = Database::connect().unwrap();
        let cats = db.load_db().unwrap();


        (FlashcardApp {
            value: 0,
            db,
            cats,
            next_button: Default::default(),
            previous_button: Default::default()
        }, Command::none())

    }

    fn title(&self) -> String {
        String::from("Generic flashcard app title")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::NextPressed => {
                self.value += 1;

                if self.value >= self.cats.len() {
                    self.value -= self.cats.len();
                }

            }
            Message::PreviousPressed => {
                if self.value == 0 {
                    self.value += self.cats.len();
                }
                self.value -= 1;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let mut name = String::new();
        match &self.cats.get(self.value) {
            Some(cat) => {
                name = cat.name.clone();
            },
            None => {
            }
        }


        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.next_button, Text::new("Next cat"))
                    .on_press(Message::NextPressed),
            )
            .push(
                Text::new(name).size(50),
            )
            .push(
                Button::new(&mut self.previous_button, Text::new("Previous cat"))
                    .on_press(Message::PreviousPressed),
            )
            .into()
    }
}
