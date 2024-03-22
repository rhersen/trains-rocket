#![allow(non_snake_case)]

use crate::train_announcement::TrainAnnouncement;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub LocationName: String,
    pub Priority: i32,
    pub Order: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TrainInfo {
    pub train_ident: String,
    pub to_location: String,
    pub activity_type: String,
    pub advertised_time: String,
    pub estimated_time: String,
    pub time_at_location: String,
}

#[derive(Deserialize, Debug)]
pub struct Result {
    pub TrainAnnouncement: Vec<TrainAnnouncement>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub RESULT: Vec<Result>,
}

#[derive(Deserialize, Debug)]
pub struct Root {
    pub RESPONSE: Response,
}
