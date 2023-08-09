use std::{collections::BTreeMap, sync::Arc};

use crate::{prelude::W, utils::macros::map};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::{
    sql::{thing, Array, Object, Value},
    Response,
};

use rocket::form::FromForm;

use crate::utils::dates::TimeForm;

type User = String;
type Text = String;

type Res<T> = Result<T, crate::error::Error>;


#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct FoodClubEntry {
    pub date: TimeForm,
    pub chef: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dish: Option<Text>,
    pub close: TimeForm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<u32>,
    // pub attendance: Vec<(User, u32)>,
}

impl From<FoodClubEntry> for Value {
    fn from(entry: FoodClubEntry) -> Self {
        let mut m = map![
            "date".into() => entry.date.into(),
            "chef".into() => entry.chef,
            "close".into() => entry.close.into(),
        ];
        if let Some(dish) = entry.dish {
            m.insert("dish".into(), dish);
        }
        if let Some(notes) = entry.notes {
            m.insert("notes".into(), notes);
        }
        if let Some(price) = entry.price {
            m.insert("price".into(), price.into());
        }
        m.into()
    }
}

impl Createable for FoodClubEntry {}

pub trait Createable: Into<Value> {}


#[derive(Clone)]
pub struct DB<C : surrealdb::Connection>(&'static Surreal<C>);

impl<C : surrealdb::Connection> DB<C> {

    pub fn new(conn: &'static Surreal<C>) -> Self {
        Self(conn)
    }

    pub async fn add_entry(&self, entry: FoodClubEntry) -> Res<FoodClubEntry> {
        let rec: FoodClubEntry = self.0
            .create("foodclub")
            .content(entry)
            .await?;
        Ok(rec)
    }

    pub async fn get_all(&self) -> Res<Vec<Object>> {
        let sql = "SELECT * FROM foodclub ORDER BY date DESC";
        let res = self.0.query(sql).await?;
        println!("Got response: {:?}", res);
        println!("num response: {:?}", res.num_statements());
        todo!()
        // let first_res = res.into_iter().next().expect("Did not get a response");

        // let array: Array = W(first_res.result?).try_into()?;

        // array.into_iter().map(|value| W(value).try_into()).collect()
    }

}
