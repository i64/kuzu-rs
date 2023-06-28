use kuzu_rs::connection::Connection;
use kuzu_rs::database::Database;
use kuzu_rs::types::custom_types::node::{InternalId, Node};
use kuzu_rs::types::custom_types::rel::Relation;
use kuzu_rs::types::row::Row;

fn main() {
    unsafe {
        let database_path = "test2";
        let mut db = Database::builder(database_path)
            .with_log_level(kuzu_rs::database::LogLevel::Debug)
            .build();

        let mut connection = Connection::new(&mut db).unwrap();

        connection.query("CREATE NODE TABLE User(name STRING, age INT64, PRIMARY KEY (name));");
        connection
            .query("CREATE NODE TABLE City(name STRING, population INT64, PRIMARY KEY (name));");

        connection.query("CREATE REL TABLE Follows(FROM User TO User, since INT64);");
        connection.query("CREATE REL TABLE LivesIn(FROM User TO City);");

        connection.query("COPY User FROM \"../test_data/user.csv\";");
        connection.query("COPY City FROM \"../test_data/city.csv\";");
        connection.query("COPY Follows FROM \"../test_data/follows.csv\";");
        connection.query("COPY LivesIn FROM \"../test_data/lives_in.csv\";");

        let res = connection.query("MATCH (a:User)<-[e:Follows]-(b:User) RETURN a, e, b");

        for r in res.iter::<Row>() {
            let a: Node = r.get(0).unwrap().into();
            dbg!(&a);
            let e: Relation = r.get(1).unwrap().into();
            dbg!(&e);
            let b: Node = r.get(2).unwrap().into();
            dbg!(&b);
        }
    }
}
