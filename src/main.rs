use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use handlebars::{DirectorySourceOptions, Handlebars};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

#[cfg(debug_assertions)]
use listenfd::ListenFd;
#[cfg(debug_assertions)]
use tokio::net::TcpListener;

async fn index(State(renderer): State<Handlebars<'static>>) -> Response {
    render("index", &renderer)
}

async fn faq(State(renderer): State<Handlebars<'static>>) -> Response {
    render("faq", &renderer)
}

async fn contacts(State(renderer): State<Handlebars<'static>>) -> Response {
    render("contacts", &renderer)
}

async fn handler_404(State(renderer): State<Handlebars<'static>>) -> impl IntoResponse {
    let mut response = render("404", &renderer);
    *response.status_mut() = StatusCode::NOT_FOUND;

    response
}

fn render(template: &str, renderer: &Handlebars<'_>) -> Response {
    match renderer.render(template, &()) {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error: {err}"),
        )
            .into_response(),
    }
}

fn make_state() -> Handlebars<'static> {
    let mut template_engine = Handlebars::new();

    template_engine
        .register_templates_directory("templates/", DirectorySourceOptions::default())
        .unwrap();

    template_engine
}

#[cfg(debug_assertions)]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // initialize tracing
    pretty_env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    // build our application with a route
    let app = Router::new()
        .nest_service("/images", ServeDir::new("images"))
        .nest_service("/demos", ServeDir::new("demos"))
        .nest_service("/css", ServeDir::new("css"))
        .nest_service("/scripts", ServeDir::new("scripts"))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .layer(TraceLayer::new_for_http())
        .route("/", get(index))
        .route("/faq", get(faq))
        .route("/contacts", get(contacts))
        .fallback(handler_404)
        .with_state(make_state());

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(debug_assertions))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new()
        .nest_service("/images", ServeDir::new("images"))
        .nest_service("/demos", ServeDir::new("demos"))
        .nest_service("/css", ServeDir::new("css"))
        .nest_service("/scripts", ServeDir::new("scripts"))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .layer(TraceLayer::new_for_http())
        .route("/", get(index))
        .route("/faq", get(faq))
        .route("/contacts", get(contacts))
        .fallback(handler_404)
        .with_state(make_state());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
