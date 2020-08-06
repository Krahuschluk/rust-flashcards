use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use crate::db::entities::Cat;

pub mod entities {
    #[derive(Debug, Default)]
    pub struct Cat {
        pub name: String,
        pub colour: String,
    }
}

struct Database {

}


pub fn connect_to_db() -> Result<Vec<Cat>> {
    println!("Attempting to connect to the DB");
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

