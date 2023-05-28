#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate log;

use simplelog::*;
use diesel::prelude::*;
use std::fmt::Write;

use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder},
    Response,
};
use rocket_sync_db_pools::database;

const LOG_PATH: &str = "vaultify.log";

mod schema; // module auto-generated schema from diesel migration
mod models;
mod endpoints;
mod guards;

use endpoints::*;
use guards::*;

/// An error type which is returned when there is an internal error or bad request
#[derive(Debug, Clone, Copy)]
pub enum ApiError {
    AuthKeyMissing,
    AuthKeyInvalid,
    EmailMissing,
    EmailInvalid,
    VaultMissing,
    VaultInvalid,
    UserExists,
    DatabaseRead,
    DatabaseWrite,
    InternalError,
    UserNoExists,
}

impl From<ApiError> for String {
    fn from(e: ApiError) -> Self {
        String::from(match e {
            ApiError::AuthKeyMissing => "Authentication key missing",
            ApiError::AuthKeyInvalid => "Authentication key invalid",
            ApiError::EmailMissing => "Email missing",
            ApiError::EmailInvalid => "Email invalid",
            ApiError::VaultMissing => "Vault missing",
            ApiError::VaultInvalid => "Vauld invalid",
            ApiError::UserExists => "User already exists in database",
            ApiError::DatabaseRead => "Failed to read database",
            ApiError::DatabaseWrite => "Failed to write to database",
            ApiError::InternalError => "Internal server error",
            ApiError::UserNoExists => "User does not exist in database",
        })
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let s: String = self.into();

        Response::build_from(s.respond_to(req)?)
            .status(Status::BadRequest)
            .header(ContentType::Text)
            .ok()
    }
}

#[catch(400)]
fn bad_request() -> &'static str {
    "Bad request"
}

use models::*;
#[database("sqlite_data_db")]
pub struct DataDatabase(diesel::SqliteConnection);

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
    let log_file = std::fs::File::options()
        .append(true)
        .create(true)
        .open(LOG_PATH)
        .expect("Failed to open logging file");

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
    ])
    .expect("Failed to initialize logging system");

    rocket::build()
        .attach(DataDatabase::fairing())
        .register("/api", catchers![bad_request])
        .mount(
            "/api",
            routes![
                authenticate,
                register_user,
            ],
        )
}
