#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate image;
extern crate img_hash;
extern crate reqwest;
//extern crate leptess;
mod imagemanip;
mod membean;
mod userscripts;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(imagemanip::stage())
        .attach(membean::stage())
        .attach(userscripts::stage())
}
