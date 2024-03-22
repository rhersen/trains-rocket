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

#[get("/station/<location_signature>")]
async fn station(location_signature: &str) -> Template {
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
        .mount("/", routes![index, station])
        .attach(Template::fairing())
}
