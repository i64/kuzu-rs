# KuzuDB

KuzuDB-rs is a Rust wrapper for interacting with the [Kuzu graph database](https://kuzudb.com/). It provides a high-level API for connecting to a Kuzu database, executing queries, and managing transactions.

## However, this library is not the official Rust wrapper for Kuzu. Please check and use the [official library](https://github.com/kuzudb/kuzu/tree/master/tools/rust_api).

## Features

- Connect to a Kuzu database and execute queries.
- Process query results and retrieve values.
- Manage transactions and perform read and write operations.
- Prepare and execute parameterized statements.
- Handle errors and propagate them through the error types provided by the library.

## Example usage

```rust
use kuzu_rs::{connection::Connection, database::Database, error::Error, types::row::Row};

#[derive(FromKuzuRow, Debug)]
struct MyRow {
    str1: String,
    str2: String,
}

fn main() -> Result<(), Error> {
    let database_path = "test2";

    let mut db = Database::builder(database_path)
        .with_log_level(kuzu_rs::database::LogLevel::Debug)
        .build()?;

    let conn = Connection::new(&mut db)?;

    let query = "RETURN 'Зарегистрируйтесь, σπαθιοῦ, Yen [jɛn], kΩ' AS str1, 'abc' as str2;";
    let result = conn.query(query)?;

    for row in result.iter::<Row>()? {
        let str1: String = row.get_val_by_column("str1")?;
        let str2: String = row.get_val_by_column("str2")?;
        println!("str1: {str1}, str2: {str2}");
        // str1: Зарегистрируйтесь, σπαθιοῦ, Yen [jɛn], kΩ, str2: abc
    }

    // let query = "RETURN 'Зарегистрируйтесь, σπαθιοῦ, Yen [jɛn], kΩ' AS str1, 'abc' as str2;";
    // let result = conn.query(query)?;

    // for row in result.iter()? {
    //     let (str1, str2): (String, String) = row;
    //     println!("str1: {str1}, str2: {str2}");
    //     // str1: Зарегистрируйтесь, σπαθιοῦ, Yen [jɛn], kΩ, str2: abc
    // }

    // let query = "RETURN 'Зарегистрируйтесь, σπαθιοῦ, Yen [jɛn], kΩ' AS str1, 'abc' as str2;";
    // let result = conn.query(query)?;

    // for row in result.iter::<MyRow>()? {
    //     println!("str1: {}, str2: {}", row.str1, row.str2);
    //     // str1: Зарегистрируйтесь, σπαθιοῦ, Yen [jɛn], kΩ, str2: abc
    // }

    Ok(())
}
```

## TODO
- [ ] Missing Features
    - [x] named paramaters
    - [x] decoding support
        - [x] primitive types
        - [x] lists
        - [x] structs
    - [ ] custom decoders
        - [ ] petagraph
    - [x] macro support
        - [x] from row
        - [x] into a struct
- [x] Dynamic building & linking
- [ ] Tests
    - [ ] Unit tests
    - [ ] Integrity tests
    - [ ] Fuzzing tests
- [x] Docs
- [ ] Publish to crate
