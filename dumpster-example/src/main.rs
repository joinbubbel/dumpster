use dumpster_axum::run_with_axum;
use libdumpster::{
    mock_fs::MockFileSystem, Class, Executor, FileSystem, ImageFormat, ImageOperation,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let fs = MockFileSystem::mount("").await.unwrap();

    let profile_picture = Class::builder("profile_picture")
        .op(Arc::new(ImageOperation::builder(ImageFormat::Jpeg).build()))
        .build();

    let executor = Executor::new(fs, &[profile_picture]).await;
    run_with_axum(executor, "0.0.0.0:3000".parse().unwrap()).await;
}
