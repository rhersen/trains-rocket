#![allow(non_snake_case)]

use crate::types::{Location, Root, TrainInfo};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrainAnnouncement {
    ActivityType: String,
    AdvertisedTimeAtLocation: String,
    AdvertisedTrainIdent: String,
    TimeAtLocationWithSeconds: Option<String>,
    ToLocation: Vec<Location>,
}

impl TrainAnnouncement {
    pub(crate) fn activity_type(&self) -> String {
        self.ActivityType[0..3].to_string()
    }

    pub(crate) fn advertised_time(&self) -> String {
        self.AdvertisedTimeAtLocation[11..16].to_string()
    }

    pub(crate) fn to_location(&self) -> String {
        self.ToLocation
            .iter()
            .map(|it| it.LocationName.clone())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub(crate) fn time_at_location(&self) -> String {
        match &self.TimeAtLocationWithSeconds {
            Some(time) => time[11..19].to_string(),
            None => "-".to_string(),
        }
    }

    pub(crate) fn train_ident(&self) -> String {
        self.AdvertisedTrainIdent.to_string()
    }

    pub(crate) fn transform(&self) -> TrainInfo {
        TrainInfo {
            train_ident: self.train_ident(),
            to_location: self.to_location(),
            activity_type: self.activity_type(),
            advertised_time: self.advertised_time(),
            time_at_location: self.time_at_location(),
        }
    }
}

pub(crate) async fn fetch(location_signature: &str) -> Result<Vec<TrainAnnouncement>, Error> {
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
                    <EQ name='LocationSignature' value='{}' />
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
        location_signature,
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
