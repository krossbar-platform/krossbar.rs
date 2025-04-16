mod index;

use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::State,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    BoxError, Router,
};
use axum_extra::extract::Host;
use clap::Parser;
use handlebars::{DirectorySourceOptions, Handlebars};
use serde::Serialize;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

/// krossbar.rs web service
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// SSL certificates dir
    #[arg(short, long)]
    certificates_dir: Option<PathBuf>,
}

async fn faq(State(renderer): State<Handlebars<'static>>) -> Response {
    render("faq", &renderer, &())
}

async fn contacts(State(renderer): State<Handlebars<'static>>) -> Response {
    render("contacts", &renderer, &())
}

async fn handler_404(State(renderer): State<Handlebars<'static>>) -> impl IntoResponse {
    let mut response = render("404", &renderer, &());
    *response.status_mut() = StatusCode::NOT_FOUND;

    response
}

pub fn render<T: Serialize>(template: &str, renderer: &Handlebars<'_>, data: &T) -> Response {
    match renderer.render(template, data) {
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

fn make_router<S>() -> Router<S> {
    Router::new()
        .nest_service("/images", ServeDir::new("images"))
        .nest_service("/demos", ServeDir::new("demos"))
        .nest_service("/css", ServeDir::new("css"))
        .nest_service("/scripts", ServeDir::new("scripts"))
        .nest_service("/book", ServeDir::new("book"))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .layer(TraceLayer::new_for_http())
        .route("/", get(index::index))
        .route("/faq", get(faq))
        .route("/contacts", get(contacts))
        .fallback(handler_404)
        .with_state(make_state())
}

#[cfg(debug_assertions)]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use listenfd::ListenFd;
    use tokio::net::TcpListener;

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
    let app = make_router();

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(debug_assertions))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::net::SocketAddr;

    use axum_server::tls_rustls::RustlsConfig;

    tokio::spawn(redirect_http_to_https());

    let args = Args::parse();
    let app = make_router();

    if let Some(certs_dir) = args.certificates_dir {
        let addr = SocketAddr::from(([0, 0, 0, 0], 443));

        let config =
            RustlsConfig::from_pem_file(certs_dir.join("cert.pem"), certs_dir.join("privkey.pem"))
                .await
                .unwrap();

        println!("Listening with SSL on :443");

        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    } else {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());

        axum::serve(listener, app).await.unwrap();
    }
}

#[allow(unused)]
async fn redirect_http_to_https() {
    fn make_https(host: String, uri: Uri) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace("80", "443");
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                eprintln!("Failed to convert URI to HTTPS: {error:?}");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}
