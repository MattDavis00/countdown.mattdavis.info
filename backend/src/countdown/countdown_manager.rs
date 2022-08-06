use crate::countdown::event::Event;
use rusqlite::{params, Connection};
use std::error::Error;

pub struct CountdownManager {
    path: String,
    conn: Connection,
}

impl CountdownManager {
    pub fn new(path: Option<&str>) -> CountdownManager {
        let path = path.unwrap_or("./database.sqlite3");
        let conn = Connection::open(&path).expect("Could not establish database connection");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS event (
                id      TEXT PRIMARY KEY,
                name    TEXT NOT NULL,
                date   TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        CountdownManager {
            path: path.to_string(),
            conn,
        }
    }

    pub fn save_event(&self, event: &Event) -> Result<(), String> {
        let conn = &self.conn;
        conn.execute(
            "INSERT INTO event (id, name, date) VALUES (?1, ?2, ?3)",
            params![event.id, event.name, event.date.to_rfc3339()],
        )
        .map_err(|_e| "Failed to insert new event.")?;
        Ok(())
    }

    pub fn get_event(&self, id: &str) -> Result<Event, Box<dyn Error>> {
        let conn = &self.conn;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::countdown::event::random_id;
    use crate::countdown::timestamp::Timestamp;
    use chrono::{TimeZone, Utc};
    use std::fs;
    use test::Bencher;

    fn countdown_manager() -> CountdownManager {
        let database_path = format!("test_db_{}.sqlite3", random_id());
        CountdownManager::new(Some(&database_path))
    }

    fn delete_database(cm: CountdownManager) {
        cm.conn.close().unwrap();
        fs::remove_file(&cm.path).unwrap();
    }

    #[bench]
    fn bench_save_then_get_event(b: &mut Bencher) {
        let cm = countdown_manager();
        b.iter(|| {
            let event = Event::with_random_id(
                "bench_save_then_get_event".to_string(),
                Timestamp(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)),
            );
            let _ = cm.save_event(&event);
            let _ = cm.get_event(&event.id);
        });
        delete_database(cm)
    }

    #[bench]
    fn bench_get_event(b: &mut Bencher) {
        let cm = countdown_manager();
        let event = Event::with_random_id(
            "bench_get_event".to_string(),
            Timestamp(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)),
        );
        let _ = cm.save_event(&event);
        b.iter(|| {
            let _ = cm.get_event(&event.id);
        });
        delete_database(cm)
    }

    #[bench]
    fn bench_save_event(b: &mut Bencher) {
        let cm = countdown_manager();
        b.iter(|| {
            let event = Event::with_random_id(
                "bench_save_event".to_string(),
                Timestamp(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)),
            );
            let _ = cm.save_event(&event);
        });
        delete_database(cm)
    }

    #[test]
    fn test_save_then_get_event() {
        let cm = countdown_manager();
        let event = Event::with_random_id(
            "test_save_then_get_event".to_string(),
            Timestamp(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)),
        );
        let _ = cm.save_event(&event);
        let event_retrieved = cm.get_event(&event.id).unwrap();
        assert_eq!(event.to_string(), event_retrieved.to_string());
        delete_database(cm)
    }
}
