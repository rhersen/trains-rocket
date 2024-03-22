#[macro_use]
extern crate rocket;

use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use types::TrainInfo;

mod train_announcement;
mod types;

#[get("/index")]
async fn index() -> Template {
    Template::render("index", {})
}

#[get("/text/<location_signature>")]
async fn text(location_signature: &str) -> String {
    match train_announcement::fetch(location_signature).await {
        Ok(announcements) => announcements
            .iter()
            .map(|it| {
                format!(
                    "{}\t{}\t{}\t{} {}",
                    it.train_ident(),
                    it.to_location(),
                    it.activity_type(),
                    it.advertised_time(),
                    it.time_at_location()
                )
            })
            .collect::<Vec<String>>()
            .join("\n"),
        Err(e) => format!("Error: {}", e),
    }
}

#[get("/trains/<location_signature>")]
async fn trains(location_signature: &str) -> Template {
    match train_announcement::fetch(location_signature).await {
        Ok(announcements) => {
            let mut trains: Vec<TrainInfo> =
                announcements.iter().map(|it| it.transform()).collect();
            trains.sort_by(|a, b| a.advertised_time.cmp(&b.advertised_time));

            Template::render(
                "trains",
                context! {location_signature: location_signature, trains: trains},
            )
        }

        Err(e) => Template::render("error", context! {error: e.to_string()}),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, text, trains])
        .attach(Template::fairing())
}
