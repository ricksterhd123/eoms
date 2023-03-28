use std::io::Error;

use rusqlite::Connection;

const ORDER_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS orders (id INTEGER PRIMARY KEY AUTOINCREMENT, ref VARCHAR(255))";

#[derive(Debug)]
pub struct Order {
    pub id: i32,
    pub reference: String,
}

pub fn init(conn: &Connection) -> Result<(), Error> {
    let result = conn.execute(ORDER_TABLE, ());

    if result.is_err() {
        panic!("Failed to initialize order table: {:?}", result.err());
    }

    Ok(())
}

pub fn get(conn: &Connection) -> Result<Vec<Order>, Error> {
    let mut stmt = conn.prepare("SELECT * FROM orders").unwrap();
    
    // best way to turn MappedRows iterator into Vec<Order>?

    let orders_iter = stmt.query_map([], |row| {
        Ok(Order { id: row.get(0)?, reference: row.get(1)? })
    }).unwrap();

    let mut orders = Vec::<Order>::new();

    for order in orders_iter {
        orders.push(order.unwrap());
    }

    Ok(orders)
}
