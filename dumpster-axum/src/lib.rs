use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use b64::FromBase64;
use libdumpster::{Executor, FileSystem};
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;

mod api;

pub use api::*;

struct OurState<FS: FileSystem + Send + Sync> {
    exec: Executor<FS>,
}

pub async fn run_with_axum<FS>(exec: Executor<FS>, mount_dir: &str, addr: SocketAddr)
where
    FS: FileSystem + Send + Sync + 'static,
{
    let state = Arc::new(OurState { exec });

    let app = Router::new()
        .route("/", get(get_hello_world))
        .route("/upload_base64", post(api_upload_base64))
        .nest_service("/get/", ServeDir::new(mount_dir))
        .with_state(Arc::clone(&state));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_hello_world() -> &'static str {
    "Hello, World"
}

async fn api_upload_base64<FS: FileSystem + Send + Sync>(
    State(state): State<Arc<OurState<FS>>>,
    Json(req): Json<InUpload>,
) -> Json<ResUpload> {
    let Ok(data) = req.base64_data.from_base64() else {
        return Json(ResUpload {
            object_name: None,
            error: Some(ResError::InvalidBase64),
        });
    };
    let Some(object_name) = state.exec.incoming(&req.class_name, "", data).await else {
        return Json(ResUpload {
            object_name: None,
            error: Some(ResError::DataRejected),
        });
    };

    Json(ResUpload {
        object_name: Some(object_name),
        error: None,
    })
}
