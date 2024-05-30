use axum::{extract::State, response::Response};
use handlebars::Handlebars;

use serde::Serialize;

use crate::render;

#[derive(Serialize)]
enum BodyEntryType {
    #[serde(rename = "demo")]
    Demo(String),
    #[serde(rename = "code")]
    Code(String),
}

#[derive(Serialize)]
struct BodyEntry {
    header: String,
    description: String,
    body_type: BodyEntryType,
}

#[derive(Serialize)]
struct BodyEntries {
    bus_entries: Vec<BodyEntry>,
    logger_entries: Vec<BodyEntry>,
    settings_entries: Vec<BodyEntry>,
}

impl BodyEntries {
    pub fn new() -> Self {
        Self {
            bus_entries: make_bus_entries(),
            logger_entries: vec![],
            settings_entries: vec![],
        }
    }
}

pub async fn index(State(renderer): State<Handlebars<'static>>) -> Response {
    render("index", &renderer, &BodyEntries::new())
}

fn make_bus_entries() -> Vec<BodyEntry> {
    vec![
        BodyEntry {
            header: "Create services".into(),
            description:
                "Register methods, signals, and states to provide service to client service.".into(),
            body_type: BodyEntryType::Code(include_str!("examples/server.rs").into()),
        },
        BodyEntry {
            header: "Connect clients".into(),
            description:
                "Call methods and states, subscribes to singlas and state changes from the clients."
                    .into(),
            body_type: BodyEntryType::Code(include_str!("examples/client.rs").into()),
        },
        BodyEntry {
            header: "Bus monitor".into(),
            description: "Monitor message exchange using Krossbar
            monitor tool."
                .into(),
            body_type: BodyEntryType::Demo("monitor".into()),
        },
        BodyEntry {
            header: "Bus connect".into(),
            description: "Connect to services to inspect
            registered endpoint, or make interactive
            calls."
                .into(),
            body_type: BodyEntryType::Demo("intro2".into()),
        },
    ]
}
