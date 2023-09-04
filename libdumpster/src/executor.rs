use super::*;
use rand::{distributions::Alphanumeric, prelude::*, rngs::OsRng};
use std::{collections::HashMap, sync::Arc};

const OBJECT_TOKEN_LENGTH: usize = 32;

pub struct Executor<FS>
where
    FS: FileSystem,
{
    fs: Box<FS>,
    classes: HashMap<String, Class>,
    loose_operations: Vec<Arc<dyn Operation + Send + Sync>>,
}

impl<FS> Executor<FS>
where
    FS: FileSystem,
{
    pub async fn new(
        fs: FS,
        classes: &[Class],
        loose_operations: &[Arc<dyn Operation + Send + Sync>],
    ) -> Self {
        for class in classes {
            let _ = fs.register_class(&class.name).await;
        }

        Self {
            fs: Box::new(fs),
            classes: classes
                .iter()
                .cloned()
                .map(|c| (c.name.clone(), c))
                .collect(),
            loose_operations: loose_operations.to_vec(),
        }
    }

    pub async fn incoming(
        &self,
        class: &str,
        mut data: Vec<u8>,
    ) -> Result<String, OperationReject> {
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

        Ok(object_token)
    }

    pub async fn loose_incoming(
        &self,
        data_name: &str,
        mut data: Vec<u8>,
    ) -> Result<String, OperationReject> {
        let object_token = generate_token_alphanumeric(OBJECT_TOKEN_LENGTH);
        self.fs
            .new_object(LOOSE_FILE_CLASS_NAME, &object_token)
            .await
            .expect("Failed to create new object (loose).");

        for op in self.loose_operations.iter() {
            //  TODO I don't like this clone.
            data = match op.incoming(data.clone()) {
                Ok(data) => data,
                Err(OperationReject::DataCorrupt { .. }) => data,
                Err(OperationReject::DataConstraint { reason }) => {
                    Err(OperationReject::DataConstraint { reason })?
                }
            };
        }

        self.fs
            .store(LOOSE_FILE_CLASS_NAME, &object_token, data_name, &data)
            .await
            .expect("Failed to store data (loose).");

        Ok(format!("{}/{}", object_token, data_name))
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
