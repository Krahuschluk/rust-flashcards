
use iced::{button, Align};
use iced::{Button, Column, Text, Settings, Element, Application, executor, Command};
use std::ops::Index;
use crate::db::connect_to_db;
use crate::db::entities::Cat;

#[derive(Default)]
pub struct FlashcardApp {
    value: usize,
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
        (FlashcardApp {
            value: 0,
            cats: connect_to_db().unwrap(),
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
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.next_button, Text::new("Next cat"))
                    .on_press(Message::NextPressed),
            )
            .push(
                Text::new(&self.cats.index(self.value).name).size(50),
            )
            .push(
                Button::new(&mut self.previous_button, Text::new("Previous cat"))
                    .on_press(Message::PreviousPressed),
            )
            .into()
    }
}
