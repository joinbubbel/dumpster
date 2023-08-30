use super::*;
use std::rc::Rc;

#[derive(Clone)]
pub enum Pipe {
    PipeOp(Rc<dyn Operation>),
    StorePipeOp(String),
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

    pub fn op(mut self, operation: impl Operation + 'static) -> Self {
        self.operations.push(Pipe::PipeOp(Rc::new(operation)));
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
