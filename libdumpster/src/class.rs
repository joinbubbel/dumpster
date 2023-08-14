use super::*;

#[derive(Clone)]
pub enum Pipe {
    PipeOp(Operation),
    StorePipeOp(String, Operation),
}

#[derive(Clone)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) operations: Vec<Pipe>,
}

impl Class {
    pub fn builder(name: &str) -> Self {
        Class {
            name: name.to_owned(),
            operations: vec![],
        }
    }

    pub fn op(mut self, operation: Operation) -> Self {
        self.operations.push(Pipe::PipeOp(operation));
        self
    }

    pub fn store(mut self, store_name: &str, operation: Operation) -> Self {
        self.operations
            .push(Pipe::StorePipeOp(store_name.to_owned(), operation));
        self
    }

    /// This serves no purpose other than to signify that you are done building.
    pub fn build(self) -> Class {
        self
    }
}
