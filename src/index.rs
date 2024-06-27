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
    log_entries: Vec<BodyEntry>,
    settings_entries: Vec<BodyEntry>,
}

impl BodyEntries {
    pub fn new() -> Self {
        Self {
            bus_entries: make_bus_entries(),
            log_entries: make_log_entries(),
            settings_entries: make_settings_entries(),
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
            body_type: BodyEntryType::Demo("connect".into()),
        },
    ]
}

fn make_log_entries() -> Vec<BodyEntry> {
    vec![
        BodyEntry {
            header: "Connect logging".into(),
            description: "Connect services to a logger to keep all logs in one place. Log into stdout if needed. Log only into stdout during developments.".into(),
            body_type: BodyEntryType::Code(include_str!("examples/log.rs").into()),
        },
        BodyEntry {
            header: "Monitor logs".into(),
            description:
                "Use krossbar-log-viewer to view log file. Despite Krossbar logs are plain text files, the viewer highlights each process, log level, and timestamp for easy monitoring. Use 'follow' mode to watch log as they appear"
                    .into(),
            body_type: BodyEntryType::Demo("log".into()),
        },
    ]
}

fn make_settings_entries() -> Vec<BodyEntry> {
    vec![
        BodyEntry {
            header: "Manage service settings".into(),
            description: "Use Krossbar settings to store and retrieve service settings accross sessions. Use settings library to automatically save any serializeable structure.".into(),
            body_type: BodyEntryType::Code(include_str!("examples/settings.rs").into()),
        },
        BodyEntry {
            header: "Read service settings".into(),
            description:
                "Use krossbar-settings-viewer to view current service settings"
                    .into(),
            body_type: BodyEntryType::Demo("settings".into()),
        },
    ]
}
