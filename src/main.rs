#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

use diesel::prelude::*;
use std::fmt::Write;

use rocket_sync_db_pools::database;

mod schema; // module auto-generated schema from diesel migration
mod models;

use models::*;
#[database("sqlite_data_db")]
struct DataDatabase(diesel::SqliteConnection);

#[get("/auth")]
async fn authenticate(mut conn: DataDatabase) -> String {
    conn.run(|c| load_from_db(c)).await
}

fn load_from_db(conn: &mut diesel::SqliteConnection) -> String {
    use crate::schema::users::dsl::*;

    let user_list = users.load::<User>(conn).expect("Error reading database.");

    let mut s = String::new();

    for user in user_list {
        writeln!(s, "{}\n", user.email).expect("Failed to write to string");
    }

    s
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DataDatabase::fairing())
        .mount("/api", routes![authenticate])
}
