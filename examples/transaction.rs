use std::error::Error;

use kuzu_rs::connection::{Connection, Transaction, TransactionType};
use kuzu_rs::database::Database;
use kuzu_rs::types::row::Row;

fn create_table(conn: &Connection) -> kuzu_rs::error::Result<()> {
    conn.query("CREATE NODE TABLE User(name STRING, balance FLOAT, PRIMARY KEY (name));")?;
    Ok(())
}

fn insert_data(conn: &Connection) -> kuzu_rs::error::Result<()> {
    conn.query("CREATE (u:User {name: 'Alice', balance: 1000.0})")?;
    conn.query("CREATE (u:User {name: 'Bob', balance: 500.0})")?;
    Ok(())
}

fn update_data(tx: &mut Transaction) -> kuzu_rs::error::Result<()> {
    tx.query("MATCH (u:User {name: 'Alice'}) SET u.balance = u.balance - 100.0")?;
    tx.query("MATCH (u:User {name: 'Bob'}) SET u.balance = u.balance + 100.0")?;

    Ok(())
}

fn print_data(tx: &mut Transaction) -> kuzu_rs::error::Result<()> {
    let res = tx
        .query(
            "MATCH (u:User) WHERE u.name = 'Alice' OR u.name = 'Bob' RETURN u.name, u.balance",
        )?;

    for r in res.iter::<Row>()? {
        let name: String = r.get_val_by_column("u.name")?;
        let balance: f32 = r.get_val_by_column("u.balance")?;

        println!("User: {}, Balance: {}", name, balance);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let database_path = "test2";
    let mut db = Database::builder(database_path)
        .with_log_level(kuzu_rs::database::LogLevel::Debug)
        .build()?;

    let mut conn = Connection::new(&mut db)?;

    create_table(&conn)?;
    insert_data(&conn)?;

    {
        let mut tx = conn.transaction(TransactionType::ReadWrite)?;

        println!("Before:");
        print_data(&mut tx)?;

        println!("After rollback:");
        update_data(&mut tx)?;
        tx.rollback();
        print_data(&mut tx)?;

        println!("After commit:");
        update_data(&mut tx)?;
        tx.commit();
        print_data(&mut tx)?;
    }

    Ok(())
}
