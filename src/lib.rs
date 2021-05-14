use chrono::prelude::*;
use reqwest::{blocking, header::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("serde_json error")]
    Serde(#[from] serde_json::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

/// An Alert, possibly indicating many affected areas.
/// The `date` field will most likely be a unique identifier. The API documentation does not exist, and therefore does not provide any gurantees however it is likely.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Alert {
    #[serde(rename = "data")]
    pub areas: Vec<String>,
    #[serde(rename = "id", with = "date_format")]
    pub date: DateTime<Utc>,
    pub title: String,
}

impl Alert {
    pub fn get() -> Result<Option<Self>> {
        let client = blocking::Client::new();
        let resp = client
            .get("http://localhost:3000")
            .header(REFERER, "https://www.oref.org.il/12481-en/Pakar.aspx")
            .header("X-Requested-With", "XMLHttpRequest")
            .header(
                USER_AGENT,
                "oref-red-alert https://github.com/ronthecookie/oref-red-alert",
            )
            .send()
            .map_err(|e| Error::Reqwest(e))?
            .text()
            .map_err(|e| Error::Reqwest(e))?;

        Ok(if resp.len() > 0 {
            Some(serde_json::from_str(&resp).map_err(|e| Error::Serde(e))?)
        } else {
            None
        })
    }
}

mod date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let num = date.timestamp_millis();
        serializer.serialize_i64(num)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i64::deserialize(deserializer)?;
        Ok(Utc.timestamp_millis(s))
    }
}

mod test {

    #[test]
    fn basic_get() {
        use super::Alert;
        Alert::get().unwrap();
    }

    // Run this for logging output: cargo test -- --nocapture
    #[test]
    fn readme_example() {
        use super::Alert;
        let alert = Alert::get().unwrap();
        if alert.is_some() {
            eprintln!("Affected cities: {}", alert.unwrap().areas.len())
        } else {
            eprintln!("There is no alert available");
        }
    }

    #[test]
    fn consistent_serde() {
        use super::Alert;
        use chrono::{TimeZone, Utc};
        let title = String::new();
        let now = Utc.timestamp_millis(1337);
        let alert = Alert {
            areas: vec![],
            date: now,
            title,
        };
        eprintln!("{:?}", serde_json::to_string(&alert));
        let new = serde_json::from_str::<Alert>(&serde_json::to_string(&alert).unwrap()).unwrap();
        assert!(alert == new);
    }
}
