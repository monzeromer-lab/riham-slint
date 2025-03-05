mod db;
use std::sync::{Arc, Mutex};

use slint::{ComponentHandle, ModelRc};

slint::include_modules!();

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
}

fn main() -> Result<(), slint::PlatformError> {
    let riham = Riham::new().unwrap();
    let database = Arc::new(Mutex::new(db::sqlite_connection()));

    let weak = riham.as_weak();
    riham.on_login(move |username, password| {
        let riham = weak.unwrap();
        if username.as_str() == "admin" && password.as_str() == "admin" {
            riham.set_currentPage(slint::SharedString::from("dashboard"));
            return riham.set_loggedin(true);
        }
        riham.invoke_loginError();
    });

    let db_clone = Arc::clone(&database);
    riham.on_deleteUser(move |id| {
        let db = db_clone.lock().unwrap();
        db.execute(
            "DELETE FROM users WHERE id = ?;
",
            [id],
        );
    });

    let db_clone = Arc::clone(&database);
    riham.on_createUser(move |username, password| {
        let db = db_clone.lock().unwrap();
        db.execute(
            "INSERT INTO users (username, password) VALUES (?, ?);",
            [username.to_string(), password.to_string()],
        );
    });

    let db_clone = Arc::clone(&database);
    let mut stmt = db_clone
        .lock()
        .unwrap()
        .prepare(
            "SELECT * FROM users;
",
        )
        .expect("Couldn't prepare statement.");

    let mut users: Vec<slint_generatedRiham::User>;
    let users_iter = stmt
        .query_map([], |row| {
            users.push(slint_generatedRiham::User {
                id: row.get(0).expect("no id"),
                name: slint::SharedString::from(row.get(1).expect("no name")),
            });
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .expect("Couldn't query users.");
    riham.set_users(ModelRc::from(users));
    riham.run()
}
