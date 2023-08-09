use chrono::{DateTime, Utc, NaiveDateTime, TimeZone, NaiveTime, NaiveDate};
use chrono_tz::Europe::Copenhagen;

use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use rocket::http::RawStr;
use rocket::form::{FromFormField, ValueField};
use rocket::form;

use surrealdb::sql::{Value, Datetime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeForm(DateTime<Utc>);

impl<'a> FromFormField<'a> for TimeForm {
    fn from_value(field: ValueField<'a>) -> form::Result<'a, Self> {
        let decoded = field.value;
        if let Ok(datetime) = NaiveDateTime::parse_from_str(decoded, "%Y-%b-%d-%H:%M") {
            let cph = Copenhagen.from_local_datetime(&datetime).unwrap();
            return Ok(TimeForm(cph.with_timezone(&Utc)));
        }
        // if let Ok(date) = NaiveDate::parse_from_str(decoded, "%Y-%b-%d") {
        //     let time = NaiveTime::from_hms_opt(19, 0, 0);
        //     let datetime = NaiveDateTime::new(date, time.unwrap());
        //     let cph = Copenhagen.from_local_datetime(&datetime).unwrap();
        //     return Ok(TimeForm(cph.with_timezone(&Utc)));
        // }
        Err(form::Error::validation(format!("could not parse : {}", decoded)))?
    }
}

impl From <TimeForm> for Value {
    fn from(form: TimeForm) -> Self {
        form.0.into()
    }
}

// impl From<NaiveDateForm> for Value {
//     fn from(date: NaiveDateForm) -> Self {
//         Value::Strand(date.0)
//     }
// }

// impl<'a> FromFormField<'a> for NaiveTimeForm {
//     fn from_value(field: ValueField<'a>) -> form::Result<'a, Self> {
//         let decoded = field.value;
//         if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M:%S%.3f") {
//             return Ok(NaiveTimeForm(time));
//         }
//         if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M") {
//             return Ok(NaiveTimeForm(time));
//         }
//         Err(form::Error::validation(format!("could not parse : {}", decoded)))?
//     }
// }

// impl From <NaiveTimeForm> for NaiveTime {
//     fn from(time: NaiveTimeForm) -> Self {
//         time.0
//     }
// }
