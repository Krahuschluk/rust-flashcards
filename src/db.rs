use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use crate::db::entities::{WordPair};

pub mod entities {
    // #[derive(Debug, Default)]
    // pub struct Cat {
    //     pub name: String,
    //     pub colour: String,
    // }

    #[derive(Debug, Default)]
    pub struct WordPair {
        pub english: String,
        pub korean: String,
    }
}

#[derive(Debug)]
pub struct Database {
    connection: Connection,
}

impl Database {

    pub fn connect() -> Result<Self> {
        println!("Attempting to connect to the DB");
        let connection = Connection::open("test.db")?;

        Ok(Database {
            connection,
        })
    }

    // pub fn load_db(&mut self) -> Result<Vec<Cat>> {
    //     let mut statement = self.connection.prepare(
    //         "SELECT name, colour FROM cats"
    //     )?;
    //
    //     let cats = statement.query_map(NO_PARAMS, |row| {
    //         Ok(Cat {
    //             name: row.get(0)?,
    //             colour: row.get(1)?,
    //         })
    //     })?;
    //
    //     let mut cat_vector = Vec::new();
    //
    //     for cat in cats {
    //         println!("Found cat {:?}", cat);
    //         cat_vector.push(cat.unwrap());
    //     }
    //
    //     Ok(cat_vector)
    // }

    pub fn load_words(&mut self) -> Result<Vec<WordPair>> {
        let mut statement = self.connection.prepare(
            "SELECT english, korean FROM words"
        )?;

        let words = statement.query_map(NO_PARAMS, |row| {
            Ok(WordPair {
                english: row.get(0)?,
                korean: row.get(1)?,
            })
        })?;

        let mut word_vector = Vec::new();

        for pair in words {
            println!("Found word {:?}", pair);
            word_vector.push(pair.unwrap());
        }

        Ok(word_vector)
    }
}
