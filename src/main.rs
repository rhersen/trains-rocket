#[macro_use]
extern crate rocket;

use chrono;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Error;
use rocket_dyn_templates::Template;
use std::ops::{Add, Sub};
use train_announcement::TrainAnnouncement;
use types::Root;

mod train_announcement;
mod types;

#[get("/text")]
async fn text() -> String {
    match post_xml_data().await {
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
    match post_xml_data().await {
        Ok(announcements) => {
            let mut context = std::collections::HashMap::new();
            context.insert("trains", announcements);
            Template::render("trains", context)
        }

        Err(_e) => Template::render("trains", {}),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![text])
        .mount("/", routes![trains])
        .attach(Template::fairing())
}

async fn post_xml_data() -> Result<Vec<TrainAnnouncement>, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/xml".parse().unwrap());

    let now = chrono::Utc::now();
    let since = now.sub(chrono::Duration::hours(1));
    let until = now.add(chrono::Duration::hours(1));

    let xml_data = format!(
        r#"
    <REQUEST>
        <LOGIN authenticationkey='{}' />
        <QUERY objecttype='TrainAnnouncement' schemaversion='1.6'>
            <FILTER>
                <AND>
                    <EQ name='LocationSignature' value='Tul' />
                    <GT name='AdvertisedTimeAtLocation' value='{}' />
                    <LT name='AdvertisedTimeAtLocation' value='{}' />
                </AND>
            </FILTER>
            <INCLUDE>ActivityType</INCLUDE>
            <INCLUDE>AdvertisedTimeAtLocation</INCLUDE>
            <INCLUDE>AdvertisedTrainIdent</INCLUDE>
            <INCLUDE>TimeAtLocationWithSeconds</INCLUDE>
            <INCLUDE>ToLocation</INCLUDE>
        </QUERY>
    </REQUEST>
"#,
        std::env::var("TRAFIKVERKET_API_KEY").unwrap_or_default(),
        since.to_rfc3339(),
        until.to_rfc3339()
    );
    let res = reqwest::Client::new()
        .post("https://api.trafikinfo.trafikverket.se/v2/data.json")
        .headers(headers)
        .body(xml_data)
        .send()
        .await?;

    println!("Status: {}", res.status());

    let data: Root = res.json().await?;

    let mut vec = data.RESPONSE.RESULT[0].TrainAnnouncement.clone();
    vec.sort_by(|a, b| a.train_ident().cmp(&b.train_ident()));
    Ok(vec)
}
