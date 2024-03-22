#![allow(non_snake_case)]

use crate::types::{Location, TrainInfo};
use serde::{Deserialize, Serialize};

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
