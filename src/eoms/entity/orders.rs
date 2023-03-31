use std::io::Error;

use rusqlite::Connection;

const ORDER_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS orders (id INTEGER PRIMARY KEY AUTOINCREMENT, ref VARCHAR(255))";

#[derive(Debug)]
pub struct Order {
    pub id: Option<i32>,
    pub reference: String,
}

pub fn init(conn: &Connection) -> Result<(), Error> {
    match conn.execute(ORDER_TABLE, ()) {
        Err(error) => panic!("Failed to initialize order table: {:?}", error),
        Ok(_) => Ok(()),
    }
}

pub fn get(conn: &Connection) -> Result<Vec<Order>, Error> {
    let mut stmt = match conn.prepare("SELECT * FROM orders") {
        Err(error) => panic!("Failed to get orders: {:?}", error),
        Ok(stmt) => stmt,
    };

    // best way to turn MappedRows iterator into Vec<Order>?
    let orders_iter = match stmt.query_map([], |row| {
        Ok(Order {
            id: row.get(0)?,
            reference: row.get(1)?,
        })
    }) {
        Err(error) => panic!("{:?}", error),
        Ok(iter) => iter,
    };

    let mut orders = Vec::<Order>::new();

    for order_result in orders_iter {
        let order = match order_result {
            Ok(order) => order,
            Err(error) => panic!("Failed to retrieve order from row: {:?}", error),
        };

        orders.push(order);
    }

    Ok(orders)
}

pub fn create(conn: &Connection, order: Order) -> Result<i32, Error> {
    match conn.execute(
        "INSERT INTO orders (id, ref) VALUES (NULL, ?1)",
        [order.reference.to_string()],
    ) {
        Err(error) => panic!("Failed to create order {:?}: {:?}", order, error),
        Ok(_) => 0
    };

    // Now attempt to get the last row id inserted
    let mut stmt = match conn.prepare("SELECT last_insert_rowid()") {
        Err(error) => panic!("Failed to get last_insert_rowid: {:?}", error),
        Ok(result) => result
    };

    let last_row_id = match stmt.query_map([], |row| -> Result<i32, _> {
        Ok(row.get(0)?)
    }) {
        Err(error) => panic!("Failed to map last_insert_rowid result: {:?}", error),
        Ok(mut results) => results.next().unwrap().unwrap()
    };

    Ok(last_row_id)
}
