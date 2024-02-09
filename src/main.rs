#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    let api_key = std::env::var("TRAFIKVERKET_API_KEY").unwrap_or_default();
    format!("{}, {}", "Hello", api_key)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
