#![feature(test)]
extern crate test;
#[macro_use]
extern crate rocket;
use chrono::{DateTime, Utc};
use rand::{self, Rng};
use rocket::{
    request::FromParam,
    tokio::time::{sleep, Duration},
};
use std::{fmt::Display, ops::Deref};

struct Event {
    id: String,
    name: String,
    date: Timestamp,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.id, self.name, &*self.date.to_rfc3339())
    }
}

struct Timestamp(DateTime<Utc>);

impl Deref for Timestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement FromParam for Timestamp/DateTime<Utc>
impl<'r> FromParam<'r> for Timestamp {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let date = param.parse::<DateTime<Utc>>().map_err(|_| "Invalid date")?;
        Ok(Timestamp(date))
    }
}

// Generate random 4 character ID using uppercase letters.
fn random_id() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const ID_LENGTH: usize = 4;
    let mut rng = rand::thread_rng();

    let id: String = (0..ID_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    id
}

#[get("/new-event/<name>/<date>")]
fn new_event(name: String, date: Timestamp) -> String {
    let event = Event {
        id: random_id(),
        name,
        date,
    };
    event.to_string()
}

#[get("/")]
fn index() -> String {
    // Get current time
    let now = Utc::now();
    format!("Hello, world! {}", now.to_rfc3339())
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![new_event, delay, index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use test::Bencher;

    #[bench]
    fn bench_new_event(b: &mut Bencher) {
        b.iter(|| {
            let name = "Test Event".to_string();
            let date = Timestamp(Utc::now());
            new_event(name, date)
        });
    }

    #[bench]
    fn bench_index(b: &mut Bencher) {
        b.iter(|| index());
    }

    #[bench]
    fn bench_random_id(b: &mut Bencher) {
        b.iter(|| random_id());
    }

    #[test]
    fn test_event_to_string() {
        let name = "Test Event".to_string();
        let date = Timestamp(Utc.ymd(2018, 1, 1).and_hms(0, 0, 0));
        let event = Event {
            id: "ABCD".to_string(),
            name,
            date,
        };
        assert_eq!(
            event.to_string(),
            "ABCD Test Event 2018-01-01T00:00:00+00:00"
        );
    }

    #[test]
    fn test_random_id_length() {
        assert_eq!(random_id().len(), 4);
    }

    #[test]
    fn test_random_id_uppercase_ascii() {
        let id: String = random_id();
        assert!(id.chars().all(|c| c.is_ascii_uppercase()));
    }
}
