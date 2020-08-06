// Let's get sdl2 into this baby
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use iced::{button, Align};
use iced::{Button, Column, Text, Settings, Element, Application, executor, Command};
use std::ops::Index;

fn main() {
    Counter::run(Settings::default());
}

#[derive(Debug, Default)]
struct Cat {
    name: String,
    colour: String,
}

fn connect_to_db() -> Result<Vec<Cat>> {
    println!("Let's try connecting to the DB");
    let conn = Connection::open("test.db")?;

    let mut statement = conn.prepare(
        "SELECT name, colour FROM cats"
    )?;

    let cats = statement.query_map(NO_PARAMS, |row| {
        Ok(Cat {
            name: row.get(0)?,
            colour: row.get(1)?,
        })
    })?;

    let mut cat_vector = Vec::new();

    for cat in cats {
        println!("Found cat {:?}", cat);
        cat_vector.push(cat.unwrap());
    }

    Ok(cat_vector)
}

#[derive(Default)]
struct Counter {
    value: usize,
    cats: Vec<Cat>,

    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();


    fn new(_flags: ()) -> (Counter, Command<Self::Message>) {
        (Counter {
            value: 0,
            cats: connect_to_db().unwrap(),
            increment_button: Default::default(),
            decrement_button: Default::default()
        }, Command::none())

    }

    fn title(&self) -> String {
        String::from("Kitties")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;

                if self.value >= self.cats.len() {
                    self.value -= self.cats.len();
                }
            }
            Message::DecrementPressed => {
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
                Button::new(&mut self.increment_button, Text::new("Next cat"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
                Text::new(&self.cats.index(self.value).name).size(50),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Previous cat"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}

