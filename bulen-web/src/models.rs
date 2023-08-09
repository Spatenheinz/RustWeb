use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

type User = String;
type Text = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoodClubEntry {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub chef: User,
    pub dish: Option<Text>,
    pub close: NaiveTime,
    pub notes: Option<Text>,
    pub price: Option<u32>,
    // pub attendance: Vec<(User, u32)>,
}
