use super::timestamp::Timestamp;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub date: Timestamp,
}

impl Event {
    pub fn with_random_id(name: String, date: Timestamp) -> Event {
        Event {
            id: random_id(),
            name,
            date,
        }
    }
}

impl From<&NewEventBody> for Event {
    fn from(event: &NewEventBody) -> Self {
        Event::with_random_id(event.name.clone(), event.date.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewEventBody {
    name: String,
    date: Timestamp,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.id, self.name, &*self.date.to_rfc3339())
    }
}

// Generate random 4 character ID using uppercase letters.
pub fn random_id() -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_index(b: &mut Bencher) {
        b.iter(|| random_id());
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
