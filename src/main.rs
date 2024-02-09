#[macro_use]
extern crate rocket;

use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Error;
use train_announcement::TrainAnnouncement;
use types::Root;

mod train_announcement;
mod types;

#[get("/")]
async fn index() -> String {
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

async fn post_xml_data() -> Result<Vec<TrainAnnouncement>, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/xml".parse().unwrap());

    let xml_data = format!(
        r#"
    <REQUEST>
        <LOGIN authenticationkey='{}' />
        <QUERY objecttype='TrainAnnouncement' schemaversion='1.6'>
            <FILTER>
                <AND>
                    <EQ name='LocationSignature' value='Tul' />
                    <GT name='AdvertisedTimeAtLocation' value='2024-02-09T14:00:04.137Z' />
                    <LT name='AdvertisedTimeAtLocation' value='2024-02-09T14:59:04.137Z' />
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
        std::env::var("TRAFIKVERKET_API_KEY").unwrap_or_default()
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
