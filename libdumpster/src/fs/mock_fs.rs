use super::*;
use std::{collections::HashMap, sync::Mutex};

#[derive(Default)]
pub struct MockFileSystem {
    classes: Mutex<HashMap<String, Class>>,
    loose_files: Mutex<HashMap<String, Vec<u8>>>,
}

#[derive(Default)]
struct Class {
    objects: HashMap<String, Object>,
}

#[derive(Default)]
struct Object {
    datas: HashMap<String, Vec<u8>>,
}

#[async_trait]
impl FileSystem for MockFileSystem {
    async fn mount(_mount_point: &str) -> Result<Self, FileSystemError>
    where
        Self: Sized,
    {
        Ok(MockFileSystem::default())
    }

    async fn register_class(&self, name: &str) -> Result<(), FileSystemError> {
        let mut classes = self.classes.lock().unwrap();
        classes.insert(name.to_owned(), Class::default());
        Ok(())
    }

    async fn new_object(&self, class_name: &str, object_name: &str) -> Result<(), FileSystemError> {
        let mut classes = self.classes.lock().unwrap();
        classes
            .get_mut(class_name)
            .unwrap()
            .objects
            .insert(object_name.to_owned(), Object::default());
        Ok(())
    }

    async fn store(
        &self,
        class_name: &str,
        object_name: &str,
        name: &str,
        bytes: &[u8],
    ) -> Result<(), FileSystemError> {
        let mut classes = self.classes.lock().unwrap();
        *classes
            .get_mut(class_name)
            .unwrap()
            .objects
            .get_mut(object_name)
            .unwrap()
            .datas
            .get_mut(name)
            .unwrap() = bytes.to_vec();

        Ok(())
    }

    async fn store_loose(
        &self,
        _: &str,
        file_name: &str,
        bytes: &[u8],
    ) -> Result<(), FileSystemError> {
        let mut loose_files = self.loose_files.lock().unwrap();
        loose_files.insert(file_name.to_owned(), bytes.to_vec());

        Ok(())
    }
}
