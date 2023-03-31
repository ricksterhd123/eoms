use std::io::Error;

use rusqlite::{Row};

use crate::eoms::entity::orders::Order;
pub mod eoms;

fn main() -> Result<(), Error> {
    const PATH: &str = "test.db";

    let conn = eoms::entity::init(PATH.to_string())?;

    println!("{}", eoms::entity::orders::create(
        &conn,
        Order {
            id: None,
            reference: "hello".to_string(),
        },
    )?);

    // let orders = eoms::entity::orders::get(&conn)?;

    // for order in orders {
    //     println!("{:?}", order);
    // }

    eoms::entity::close(conn)?;
    Ok(())
}
