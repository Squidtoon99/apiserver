use rocket::fs::{relative, FileServer};

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Userscripts", |rocket| async {
        rocket.mount("/scripts", FileServer::from(relative!("static")))
    })
}
