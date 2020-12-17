use crate::db::entities::WordPair;
use crate::db::Database;
use iced::{button, Align};
use iced::{executor, Application, Button, Column, Command, Container, Element, Length, Row, Text};

// #[derive(Default)]
pub enum FlashcardApp {
    Menu,
    Flashcard(FlashcardState),
    Database,
}

struct FlashcardState {
    index: usize,
    db: Database,
    words: Vec<WordPair>,

    next_button: button::State,
    previous_button: button::State,
    show_button: button::State,

    show_flag: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NextPressed,
    PreviousPressed,
    ShowButtonPressed,
    HideButtonPressed,
}

impl Application for FlashcardApp {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (FlashcardApp, Command<Self::Message>) {
        let mut db = Database::connect().unwrap();
        let words = db.load_words().unwrap();

        (
            FlashcardApp::Flashcard(FlashcardState {
                index: 0,
                db,
                words,
                next_button: Default::default(),
                previous_button: Default::default(),
                show_button: Default::default(),

                show_flag: false,
            }),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Generic flashcard app title")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match self {
            FlashcardApp::Flashcard(state) => match message {
                Message::NextPressed => {
                    state.index += 1;
                    if state.index >= state.words.len() {
                        state.index -= state.words.len();
                    }
                    state.show_flag = false;
                }
                Message::PreviousPressed => {
                    if state.index == 0 {
                        state.index += state.words.len();
                    }
                    state.index -= 1;
                    state.show_flag = false;
                }
                Message::ShowButtonPressed => {
                    state.show_flag = true;
                }

                Message::HideButtonPressed => {
                    state.show_flag = false;
                }
            },

            default => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            FlashcardApp::Flashcard(state) => {
                let mut english = String::new();
                let mut korean = String::new();
                match &state.words.get(state.index) {
                    Some(word) => {
                        english = word.english.clone();
                        korean = word.korean.clone();
                    }
                    None => {
                        // There should be some error handling here or just unwrap this baby
                    }
                }

                let next_button = Button::new(&mut state.next_button, Text::new("Next word"))
                    .on_press(Message::NextPressed);

                let previous_button =
                    Button::new(&mut state.previous_button, Text::new("Previous word"))
                        .on_press(Message::PreviousPressed);

                let english_text = Text::new(english).size(100);
                let korean_text = if state.show_flag {
                    Text::new(korean).size(100)
                } else {
                    Text::new("").size(100)
                };

                let show_button = if state.show_flag {
                    Button::new(&mut state.show_button, Text::new("Hide answer"))
                        .on_press(Message::HideButtonPressed)
                } else {
                    Button::new(&mut state.show_button, Text::new("Show answer"))
                        .on_press(Message::ShowButtonPressed)
                };

                let mut content = Column::new()
                    .spacing(100)
                    .align_items(Align::Center)
                    .push(english_text)
                    .push(korean_text)
                    .push(
                        Row::new()
                            .spacing(20)
                            .push(previous_button)
                            .push(next_button),
                    )
                    .push(show_button);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }

            default => Container::new(Column::new()).into(),
        }
    }
}
