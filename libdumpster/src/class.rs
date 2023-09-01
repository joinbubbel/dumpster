use super::*;
use std::sync::Arc;

#[derive(Clone)]
pub enum Pipe {
    PipeOp(Arc<dyn Operation + Send + Sync>),
    StorePipeOp(String),
}

#[derive(Clone)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) operations: Vec<Pipe>,
}

impl Class {
    pub fn builder(name: &str) -> Self {
        assert_ne!(name, LOOSE_FILE_CLASS_NAME);

        Class {
            name: name.to_owned(),
            operations: vec![],
        }
    }

    pub fn op(mut self, operation: Arc<dyn Operation + Send + Sync>) -> Self {
        self.operations.push(Pipe::PipeOp(operation));
        self
    }

    pub fn store(mut self, store_name: &str) -> Self {
        self.operations
            .push(Pipe::StorePipeOp(store_name.to_owned()));
        self
    }

    /// This serves no purpose other than to signify that you are done building.
    pub fn build(self) -> Self {
        self
    }
}
