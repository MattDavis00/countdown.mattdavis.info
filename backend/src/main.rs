#![feature(test)]
#[macro_use]
extern crate rocket;
extern crate test;

use backend::countdown::countdown_manager::CountdownManager;
use backend::countdown::event::{Event, NewEventBody};
use chrono::Utc;
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use rocket_cors::AllowedOrigins;
use std::env;
use std::sync::{Arc, Mutex};

struct ApplicationState {
    cm: Arc<Mutex<CountdownManager>>,
}

#[launch]
fn rocket() -> Rocket<Build> {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let frontend_url = match mode.as_str() {
        "dev" | "development" => "http://127.0.0.1:5173/",
        "prod" | "production" => "https://countdown.mattdavis.info",
        _ => panic!("Was expecting CLI argument for mode ('dev' or 'prod')"),
    };

    let cm = CountdownManager::new(None);
    let state = ApplicationState {
        cm: Arc::new(Mutex::new(cm)),
    };

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&[frontend_url]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS fairing");

    println!("{:#?}", cors);
    rocket::build()
        .mount("/", routes![get_event_by_id, index, post_new_event])
        .attach(cors)
        .manage(state)
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

#[post("/event", data = "<data>")]
fn post_new_event(
    state: &State<ApplicationState>,
    data: Json<NewEventBody>,
) -> Result<String, BadRequest<String>> {
    let event = Event::from(&data.into_inner());
    let cm = state.cm.lock().unwrap();
    match cm.save_event(&event) {
        Ok(_) => Ok(event.id),
        Err(_) => Err(BadRequest(None)),
    }
}

#[get("/event/<id>")]
fn get_event_by_id(
    state: &State<ApplicationState>,
    id: String,
) -> Result<Json<Event>, NotFound<String>> {
    let cm = state.cm.lock().unwrap();
    let event = cm.get_event(&id);

    match event {
        Ok(event) => Ok(Json(event)),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_index(b: &mut Bencher) {
        b.iter(|| index());
    }
}
