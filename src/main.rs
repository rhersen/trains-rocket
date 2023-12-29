#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Helo, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
