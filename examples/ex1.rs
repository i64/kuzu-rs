use kuzu_rs::connection::Connection;
use kuzu_rs::database::{Database, DatabaseBuilder, SystemConfig};

fn main() {
    unsafe {
        dbg!(std::mem::size_of::<Database>());
        let database_path = "test";
        let mut db = Database::new(database_path);

        println!("maiin {:p}", &db);
        println!("maiin.0.0 {:p}", db.0 .0.as_ptr());

        let mut connection = Connection::new(&mut db);
        //dbg!(2);
        let res =
            connection.query("CREATE NODE TABLE User(name STRING, age INT64, PRIMARY KEY (name))");

        //dbg!((*res).is_success());
        //dbg!((*res).get_error_message());
        // let _db = Database::new(db_path);
        // let _db = Database::new_custom(db_path, SystemConfig::default());
    }
}
