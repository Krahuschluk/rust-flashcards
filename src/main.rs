// Let's get sdl2 into this baby
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use iced::{button, Align};
use iced::{Button, Column, Text, Sandbox, Settings, Element};

fn main() {
    println!("Hello, world!");

    connect_to_db();
    Counter::run(Settings::default());
}

#[derive(Debug)]
struct Cat {
    name: String,
    colour: String,
}

fn connect_to_db() -> Result<()> {
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

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}

#[derive(Default)]
struct Counter {
    value: i32,

    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Kitties")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
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
                Text::new(&self.value.to_string()).size(50),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Previous cat"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}