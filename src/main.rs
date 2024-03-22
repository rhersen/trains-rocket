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

const LOCATION_SIGNATURE: &'static str = "Tul";

#[get("/text")]
async fn text() -> String {
    match train_announcement::fetch(LOCATION_SIGNATURE).await {
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

#[get("/trains")]
async fn trains() -> Template {
    match train_announcement::fetch(LOCATION_SIGNATURE).await {
        Ok(announcements) => {
            let mut trains: Vec<TrainInfo> =
                announcements.iter().map(|it| it.transform()).collect();
            trains.sort_by(|a, b| a.advertised_time.cmp(&b.advertised_time));

            Template::render(
                "trains",
                context! {location_signature: LOCATION_SIGNATURE, trains: trains},
            )
        }

        Err(_e) => Template::render("trains", {}),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, text, trains])
        .attach(Template::fairing())
}
