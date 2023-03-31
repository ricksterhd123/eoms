pub mod orders;
pub mod schema;

use std::io::Error;
use rusqlite::Connection;
use self::schema::DB_SCHEMA;

pub fn init(path: String) -> Result<Connection, Error> {
    let open_result = Connection::open(&path);

    if open_result.is_err() {
        panic!(
            "Failed to open database from {}: {:?}",
            path,
            open_result.err()
        );
    }

    let conn = open_result.unwrap();

    match conn.execute(DB_SCHEMA, ()) {
        Err(error) => panic!("Failed to initialize order table: {:?}", error),
        Ok(_) => (),
    };

    Ok(conn)
}

pub fn close(connection: Connection) -> Result<(), Error> {
    let result = connection.close();

    if result.is_err() {
        panic!("Failed to close database connection: {:?}", result.err());
    }

    Ok(())
}
