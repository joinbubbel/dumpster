use super::*;
use rand::{distributions::Alphanumeric, prelude::*, rngs::OsRng};
use std::collections::HashMap;

const OBJECT_TOKEN_LENGTH: usize = 32;

pub struct Executor<FS>
where
    FS: FileSystem,
{
    fs: Box<FS>,
    classes: HashMap<String, Class>,
}

impl<FS> Executor<FS>
where
    FS: FileSystem,
{
    pub async fn new(fs: FS, classes: &[Class]) -> Self {
        for class in classes {
            fs.register_class(&class.name)
                .await
                .expect("Failed to register class.");
        }

        Self {
            fs: Box::new(fs),
            classes: classes
                .iter()
                .cloned()
                .map(|c| (c.name.clone(), c))
                .collect(),
        }
    }

    pub async fn incoming(
        &self,
        class: &str,
        mut data: Vec<u8>,
    ) -> Option<String> {
        let class = self.classes.get(class).expect("That class does not exist.");

        let object_token = generate_token_alphanumeric(OBJECT_TOKEN_LENGTH);
        self.fs
            .new_object(&class.name, &object_token)
            .await
            .expect("Failed to create new object.");

        for op in class.operations.iter() {
            match op {
                Pipe::PipeOp(op) => {
                    data = op.incoming(data)?;
                }
                Pipe::StorePipeOp(output_name) => {
                    self.fs
                        .store(&class.name, &object_token, output_name, &data)
                        .await
                        .expect("Failed to store data.");
                }
            }
        }

        Some(object_token)
    }
}

/// Securely generate a random alphanumeric string of length `length`.
fn generate_token_alphanumeric(length: usize) -> String {
    OsRng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
