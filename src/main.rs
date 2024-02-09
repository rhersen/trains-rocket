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
    let result = post_xml_data().await;
    match result {
        Ok(announcements) => format!("{} announcements found", announcements.len()),
        Err(e) => format!("Error: {}", e),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

async fn post_xml_data() -> Result<Vec<TrainAnnouncement>, Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/xml".parse().unwrap());

    let api_key = std::env::var("TRAFIKVERKET_API_KEY").unwrap_or_default();

    let xml_data = format!(
        r#"
    <REQUEST>
        <LOGIN authenticationkey='{}' />
        <QUERY objecttype='TrainAnnouncement' schemaversion='1.6'>
            <FILTER>
                <AND>
                    <EQ name='LocationSignature' value='Tul' />
                    <GT name='AdvertisedTimeAtLocation' value='2024-02-09T13:00:04.137Z' />
                    <LT name='AdvertisedTimeAtLocation' value='2024-02-09T13:59:04.137Z' />
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
        api_key
    );
    let res = client
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
