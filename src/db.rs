use rusqlite::Connection;

pub fn sqlite_connection() -> Connection {
    let conn = Connection::open("./riham.db3").expect("Couldn't connect to db.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS inventory (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    type TEXT NOT NULL,
    color TEXT NOT NULL,
    size INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    price INTEGER NOT NULL
);",
        (),
    )
    .expect("Couldn't create inventory table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sales (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    phone TEXT NOT NULL UNIQUE,
    address TEXT NOT NULL,
    type TEXT NOT NULL,
    color TEXT NOT NULL,
    size TEXT NOT NULL,
    quantity INTEGER NOT NULL
);",
        (),
    )
    .expect("Couldn't create sales table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);",
        (),
    )
    .expect("Couldn't create users table.");

    conn
}
