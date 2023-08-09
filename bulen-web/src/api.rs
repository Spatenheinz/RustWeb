use crate::models::*;

use chrono::NaiveDateTime;
use reqwasm::{http::Request, Error};
use reqwasm::http::FormData;

use crate::utils::macros::*;

const BASE_URL: &str = "http://localhost:8080";

pub async fn get_all() -> Result<Vec<FoodClubEntry>, Error> {
    Request::get(&format!("{BASE_URL}/tasks"))
        .send()
        .await?
        .json()
        .await
}

pub async fn create_entry(entry: &FoodClubEntry) -> Result<(), Error> {
    let dt = NaiveDateTime::new(entry.date, entry.time);
    let dt_str = dt.format("%Y-%b-%d-%H:%M").to_string();
    let close = NaiveDateTime::new(entry.date, entry.close);
    let close_str = close.format("%Y-%b-%d-%H:%M").to_string();
    let form = form! {
        "chef".into() => &entry.chef,
        "date".into() => &dt_str,
        "close".into() => &close_str,
    }?;
    if let Some(dish) = entry.dish {
        form.append_with_str("dish".into(), &dish)?;
    }
    if let Some(notes) = entry.notes {
       form.append_with_str("notes".into(), &notes)?;
    }
    if let Some(price) = entry.price {
       form.append_with_str("price".into(), &price.to_string())?;
    }
    Request::post(&format!("{BASE_URL}/task"))
        .body(form)
        .send()
        .await?
        .json()
        .await
}
