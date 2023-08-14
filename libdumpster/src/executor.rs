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
        data_name: &str,
        mut data: Vec<u8>,
    ) -> Option<(String, Vec<String>)> {
        let class = self.classes.get(class).expect("That class does not exist.");

        let object_token = generate_token_alphanumeric(OBJECT_TOKEN_LENGTH);
        self.fs
            .new_object(&class.name, &object_token)
            .await
            .expect("Failed to create new object.");

        let mut output_datas = vec![];

        for op in class.operations.iter() {
            match op {
                Pipe::PipeOp(op) => {
                    data = (op.incoming)(data_name, data)?;
                }
                Pipe::StorePipeOp(output_name, op) => {
                    data = (op.incoming)(data_name, data)?;
                    self.fs
                        .store(&class.name, &object_token, output_name, &data)
                        .await
                        .expect("Failed to store data.");
                    output_datas.push(output_name.to_owned());
                }
            }
        }

        Some((object_token, output_datas))
    }

    pub async fn outgoing(&self, class: &str, object_name: &str, data_name: &str) -> Vec<u8> {
        let class = self.classes.get(class).expect("That class does not exist.");

        let mut data = self
            .fs
            .load(&class.name, object_name, data_name)
            .await
            .expect("Failed to load data.");

        for op in class.operations.iter().rev() {
            match op {
                Pipe::PipeOp(op) | Pipe::StorePipeOp(_, op) => {
                    data = (op.outgoing)(data).expect("Corrupt outgoing data.");
                }
            }
        }

        data
    }
}

/// Securely generate a random alphanumeric string of length `length`.
pub fn generate_token_alphanumeric(length: usize) -> String {
    OsRng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
