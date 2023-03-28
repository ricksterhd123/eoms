use std::io::Error;

pub mod eoms;

fn main() -> Result<(), Error> {
    const PATH: &str = "test.db";

    let conn = eoms::model::init(PATH.to_string())?;
    println!("Connected to database {}", PATH);

    let orders = eoms::model::orders::get(&conn)?;
    println!("Got {} orders", orders.len());

    for order in orders.iter() {
        println!("{:?}", order);
    }

    eoms::model::close(conn)?;
    Ok(())
}
