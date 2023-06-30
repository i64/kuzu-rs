use std::error::Error;

use kuzu_rs::connection::Connection;
use kuzu_rs::database::Database;
use kuzu_rs::error;
use kuzu_rs::types::row::Row;
use kuzu_rs::types::value::{Node, Relation};

fn create_tables(connection: &mut Connection) -> error::Result<()> {
    connection.query("CREATE NODE TABLE User(name STRING, age INT64, PRIMARY KEY (name));")?;
    connection
        .query("CREATE NODE TABLE City(name STRING, population INT64, PRIMARY KEY (name));")?;
    connection.query("CREATE REL TABLE Follows(FROM User TO User, since INT64);")?;
    connection.query("CREATE REL TABLE LivesIn(FROM User TO City);")?;
    Ok(())
}

fn load_data(connection: &mut Connection) -> error::Result<()> {
    connection.query("COPY User FROM \"../test_data/user.csv\";")?;
    connection.query("COPY City FROM \"../test_data/city.csv\";")?;
    connection.query("COPY Follows FROM \"../test_data/follows.csv\";")?;
    connection.query("COPY LivesIn FROM \"../test_data/lives_in.csv\";")?;
    Ok(())
}

fn perform_query(connection: &mut Connection) -> error::Result<()> {
    let res = connection
        .prepare("MATCH (a:User)<-[e:Follows]-(b:User) WHERE a.age > $1 RETURN a, e, b")?
        .bind(1i64)
        .execute()?;

    for r in res.iter::<Row>()? {
        let node_a: Node = r.get_val_by_column("a")?;
        let rel_e: Relation = r.get_val_by_column("e")?;
        let node_b: Node = r.get_val_by_column("b")?;

        println!("User: {:?}", node_a);
        println!("Follows: {:?}", rel_e);
        println!("User: {:?}", node_b);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let database_path = "test2";
    let mut db = Database::builder(database_path)
        .with_log_level(kuzu_rs::database::LogLevel::Debug)
        .build()?;

    let mut connection = Connection::new(&mut db)?;

    create_tables(&mut connection)?;
    load_data(&mut connection)?;
    perform_query(&mut connection)?;

    Ok(())
}
