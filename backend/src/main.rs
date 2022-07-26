#![feature(test)]
extern crate test;
#[macro_use]
extern crate rocket;
use chrono::{DateTime, Utc};
use rand::{self, Rng};
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::json::Json;
use rocket::{
    request::FromParam,
    tokio::time::{sleep, Duration},
};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;
use std::{fmt::Display, ops::Deref};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct Timestamp(DateTime<Utc>);
impl Deref for Timestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl FromSql for Timestamp {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let timestamp =
            DateTime::<Utc>::from_str(value.as_str()?).map_err(|_e| FromSqlError::InvalidType)?;
        Ok(Timestamp(timestamp))
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

#[derive(Serialize, Deserialize)]
struct NewEvent {
    name: String,
    date: Timestamp,
}

fn db_connection() -> Connection {
    // let path = "./database.sqlite3";
    // let conn = Connection::open(path).unwrap();
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS event (
                id      TEXT PRIMARY KEY,
                name    TEXT NOT NULL,
                date   TEXT NOT NULL
            )",
        [],
    )
    .unwrap();
    conn
}

fn save_event(event: &Event) -> Result<(), String> {
    let conn = db_connection();
    conn.execute(
        "INSERT INTO event (id, name, date) VALUES (?1, ?2, ?3)",
        params![event.id, event.name, event.date.to_rfc3339()],
    )
    .map_err(|_e| "Failed to insert new event.")?;
    Ok(())
}

fn get_event(id: &str) -> Result<Event, Box<dyn Error>> {
    let conn = db_connection();
    let mut stmt = conn.prepare("SELECT id, name, date FROM event WHERE id = ?1")?;
    let mut event_iter = stmt.query_map(params![id], |row| {
        Ok(Event {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
        })
    })?;

    let event = match event_iter.next() {
        Some(event) => event,
        None => return Err(format!("No event found for ID {}", id).into()),
    };
    let event = match event {
        Ok(event) => event,
        Err(_) => return Err(format!("No event found for ID {}", id).into()),
    };

    Ok(event)
}

#[post("/new-event", data = "<data>")]
fn post_new_event(data: Json<NewEvent>) -> Result<String, BadRequest<String>> {
    let event = Event {
        id: random_id(),
        name: data.name.clone(),
        date: Timestamp(data.date.clone()),
    };

    match save_event(&event) {
        Ok(_) => Ok(event.id),
        Err(_) => Err(BadRequest(None)),
    }
}

#[get("/new-event/<id>")]
fn get_event_by_id(id: String) -> Result<Json<Event>, NotFound<String>> {
    let event = get_event(&id);

    match event {
        Ok(event) => Ok(Json(event)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

/**
 * Displays a health check message on the index page, including the current time.
 */
#[get("/")]
fn index() -> String {
    let now = Utc::now();
    format!(
        "api.countdown.mattdavis.info \n\nStatus: Healthy\nCurrent time: {}",
        now.to_rfc3339()
    )
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_event_by_id, delay, index, post_new_event])
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use test::Bencher;

    #[bench]
    fn bench_index(b: &mut Bencher) {
        b.iter(|| index());
    }

    #[bench]
    fn bench_random_id(b: &mut Bencher) {
        b.iter(|| random_id());
    }

    #[bench]
    fn bench_save_then_get_event(b: &mut Bencher) {
        b.iter(|| {
            let event = Event {
                id: random_id(),
                name: "Save then get bench test".to_string(),
                date: Timestamp(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)),
            };
            let _ = save_event(&event);
            let _ = get_event(&event.id);
        });
    }

    #[test]
    fn test_save_then_get_event() {
        let event = Event {
            id: random_id(),
            name: "Save then get bench test".to_string(),
            date: Timestamp(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)),
        };
        let _ = save_event(&event);
        let event_retrieved = get_event(&event.id).unwrap();
        assert_eq!(event.to_string(), event_retrieved.to_string())
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
