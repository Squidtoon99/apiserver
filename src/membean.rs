use rocket::{
    fairing::AdHoc,
    fairing::{Fairing, Info, Kind},
    http::Header,
    response::{status::Created, Debug},
    serde::{
        json::{json, Json, Value},
        Deserialize, Serialize,
    },
    Build, Request, Response, Rocket,
};
use rocket_sync_db_pools::diesel;

#[derive(Default)]
pub struct CORS();

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[allow(unused_imports)]
use self::diesel::prelude::*;

#[database("postgres")]
pub struct Db(diesel::PgConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "membean"]
struct Question {
    question: String,
    answer: String,
}

table! {
    membean (question) {
        question -> Varchar,
        answer -> Varchar,
    }
}

#[post("/", data = "<question>")]
async fn create(db: Db, question: Json<Question>) -> Result<Created<Json<Question>>> {
    let question_value = question.clone();
    db.run(move |conn| {
        diesel::insert_into(membean::table)
            .values(question_value)
            .execute(conn)
    })
    .await?;

    Ok(Created::new("/").body(question))
}

#[get("/<question>")]
async fn read(db: Db, question: String) -> Option<Json<Question>> {
    db.run(move |conn| {
        membean::table
            .filter(membean::question.eq(question))
            .first(conn)
    })
    .await
    .map(Json)
    .ok()
}

#[options("/")]
async fn show_cors() -> &'static str {
    "Success!"
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Question not found"
    })
}

#[catch(500)]
fn duplicate() -> Value {
    json!({
        "status": "error",
        "reason": "Question answer already exists"
    })
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(CORS::default())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount("/membean", routes![create, read, show_cors])
            .register("/membean", catchers![not_found, duplicate])
    })
}
