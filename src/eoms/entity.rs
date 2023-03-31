use std::io::Error;

use rusqlite::Connection;

pub mod orders;

pub fn init(path: String) -> Result<Connection, Error> {
    let open_result = Connection::open(&path);

    if open_result.is_err() {
        panic!("Failed to open database from {}: {:?}", path, open_result.err());
    }

    let conn = open_result.unwrap();

    // init all entities
    orders::init(&conn)?;

    Ok(conn)
}

pub fn close(connection: Connection) -> Result<(), Error> {
    let result = connection.close();

    if result.is_err() {
        panic!("Failed to close database connection: {:?}", result.err());
    }

    Ok(())
}
