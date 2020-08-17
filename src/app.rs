
use iced::{button, Align};
use iced::{Button, Column, Text, Element, Application, executor, Command, Container, Row, Length};
use crate::db::Database;
use crate::db::entities::Cat;

// #[derive(Default)]
pub struct FlashcardApp {
    index: usize,
    db: Database,
    cats: Vec<Cat>,

    next_button: button::State,
    previous_button: button::State,
    give_up_button: button::State,
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
            index: 0,
            db,
            cats,
            next_button: Default::default(),
            previous_button: Default::default(),
            give_up_button: Default::default(),
        }, Command::none())

    }

    fn title(&self) -> String {
        String::from("Generic flashcard app title")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::NextPressed => {
                self.index += 1;

                if self.index >= self.cats.len() {
                    self.index -= self.cats.len();
                }

            }
            Message::PreviousPressed => {
                if self.index == 0 {
                    self.index += self.cats.len();
                }
                self.index -= 1;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let mut name = String::new();
        match &self.cats.get(self.index) {
            Some(cat) => {
                name = cat.name.clone();
            },
            None => {
            }
        }

        let next_button = Button::new(&mut self.next_button, Text::new("Next cat"))
            .on_press(Message::NextPressed);

        let previous_button = Button::new(&mut self.previous_button, Text::new("Previous cat"))
            .on_press(Message::PreviousPressed);

        let content = Column::new()
            .spacing(100)
            .align_items(Align::Center)
            .push(next_button)
            .push(Text::new(name).size(100))
            .push(previous_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

    }
}
