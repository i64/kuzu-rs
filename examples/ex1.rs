use kuzu_rs::connection::Connection;
use kuzu_rs::database::Database;

fn main() {
    unsafe {
        let database_path = "test2";
        let mut db = Database::builder(database_path)
            .with_log_level(kuzu_rs::database::LogLevel::Debug)
            .build();

        let mut connection = Connection::new(&mut db).unwrap();

        let _ =
            connection.query("CREATE NODE TABLE Person(name STRING, age INT64, isStudent BOOLEAN, PRIMARY KEY(name));");

        let _ = connection.query("CREATE (a:Person {name: 'elma', age: 12, isStudent: false});");

        let res = connection
            .prepare("MATCH (a:Person) WHERE a.isStudent = $1 RETURN a.age")
            .bind(false)
            .execute();

        for r in res.iter() {
            let age: (i64,) = r;
            dbg!(age);
        }
    }
}
