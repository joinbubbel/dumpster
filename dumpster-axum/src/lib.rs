use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use libdumpster::{Executor, FileSystem};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

struct OurState<FS: FileSystem + Send + Sync> {
    exec: Mutex<Executor<FS>>,
}

pub async fn run_with_axum<FS>(exec: Executor<FS>, addr: SocketAddr)
where
    FS: FileSystem + Send + Sync + 'static,
{
    let state = Arc::new(OurState {
        exec: Mutex::new(exec),
    });

    let app = Router::new()
        .route("/", get(get_hello_world))
        .with_state(Arc::clone(&state));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_hello_world() -> &'static str {
    "Hello, World"
}

