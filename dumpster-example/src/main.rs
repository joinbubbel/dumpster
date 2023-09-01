use dumpster_axum::run_with_axum;
use libdumpster::{
    tokio_fs::TokioFileSystem, Class, Executor, FileSystem, ImageFormat, ImageOperation,
    ImageOperationStep,
};
use std::{env, fs, sync::Arc};

#[tokio::main]
async fn main() {
    let mount_dir = env::temp_dir().to_str().unwrap().to_owned() + "/dumpster/";
    let _ = fs::create_dir(&mount_dir);
    let fs = TokioFileSystem::mount(&mount_dir).await.unwrap();

    let profile_picture = Class::builder("profile_picture")
        .op(Arc::new(
            ImageOperation::builder(ImageFormat::Jpeg)
                .add_step(ImageOperationStep::Resize(100, 100))
                .build(),
        ))
        .store("image.jpeg")
        .build();

    let executor = Executor::new(fs, &[profile_picture], &[]).await;
    run_with_axum(executor, &mount_dir, "0.0.0.0:3000".parse().unwrap()).await;
}
