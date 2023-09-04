use super::*;
use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use b64::FromBase64;
use libdumpster::{Executor, FileSystem, OperationReject};
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;

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
        .route("/upload_loose_base64", post(api_upload_loose_base64))
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
    Json(req): Json<InUploadBase64>,
) -> Json<ResUploadBase64> {
    let (_, base64_data) = req
        .base64_data
        .split_once(',')
        .unwrap_or(("", &req.base64_data));
    let Ok(data) = base64_data.from_base64() else {
        return Json(ResUploadBase64 {
            object_name: None,
            error: Some(UploadBase64Error::InvalidBase64),
        });
    };
    match state.exec.incoming(&req.class_name, data).await {
        Ok(object_name) => Json(ResUploadBase64 {
            object_name: Some(object_name),
            error: None,
        }),
        Err(OperationReject::DataCorrupt { reason }) => Json(ResUploadBase64 {
            object_name: None,
            error: Some(UploadBase64Error::DataCorrupt { reason }),
        }),
        Err(OperationReject::DataConstraint { reason }) => Json(ResUploadBase64 {
            object_name: None,
            error: Some(UploadBase64Error::DataConstraint { reason }),
        }),
    }
}

async fn api_upload_loose_base64<FS: FileSystem + Send + Sync>(
    State(state): State<Arc<OurState<FS>>>,
    Json(req): Json<InUploadLooseBase64>,
) -> Json<ResUploadLooseBase64> {
    let (_, base64_data) = req
        .base64_data
        .split_once(',')
        .unwrap_or(("", &req.base64_data));
    let Ok(data) = base64_data.from_base64() else {
        return Json(ResUploadLooseBase64 {
            object_name: None,
            error: Some(UploadLooseBase64Error::InvalidBase64),
        });
    };
    match state.exec.loose_incoming(&req.file_name, data).await {
        Ok(object_name) => Json(ResUploadLooseBase64 {
            object_name: Some(object_name),
            error: None,
        }),
        Err(OperationReject::DataCorrupt { .. }) => unreachable!(),
        Err(OperationReject::DataConstraint { reason }) => Json(ResUploadLooseBase64 {
            object_name: None,
            error: Some(UploadLooseBase64Error::DataConstraint { reason }),
        }),
    }
}
