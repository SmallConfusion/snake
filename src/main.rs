#[macro_use]
extern crate rocket;

use log::info;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::{Value, json};

use crate::api_types::GameState;

pub mod api_types;
pub mod engine;

#[get("/")]
fn handle_index() -> Json<Value> {
    Json(json!({
        "apiversion": "1",
        "author": "SmallConfusion",
        "color": "#ffa9e9",
        "head": "trans-rights-scarf",
        "tail": "round-bum",
    }))
}

#[post("/start", format = "json", data = "<_start_req>")]
fn handle_start(_start_req: Json<GameState>) -> Status {
    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameState>) -> Json<Value> {
    Json(json!(engine::get_move(&move_req)))
}

#[post("/end", format = "json", data = "<_end_req>")]
fn handle_end(_end_req: Json<GameState>) -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    info!("Starting Battlesnake Server...");

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "battlesnake/github/starter-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}
