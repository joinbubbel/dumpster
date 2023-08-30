use super::*;

#[tokio::test]
async fn hello_world() {
    let fs = mock_fs::MockFileSystem::mount("").await.unwrap();

    let profile_pictures = Class::builder("profile_picture").build();
}
