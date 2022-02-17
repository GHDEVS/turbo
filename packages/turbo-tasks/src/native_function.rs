use crate::{self as turbo_tasks, task::NativeTaskFn, SlotRef, TaskArgumentOptions};
use anyhow::Result;
use std::{fmt::Debug, hash::Hash};

#[turbo_tasks::value]
pub struct NativeFunction {
    pub name: String,
    // TODO avoid a function to avoid allocting many vectors
    #[trace_ignore]
    pub task_argument_options: Box<dyn Fn() -> Vec<TaskArgumentOptions> + Send + Sync + 'static>,
    #[trace_ignore]
    pub bind_fn: Box<dyn (Fn(Vec<SlotRef>) -> Result<NativeTaskFn>) + Send + Sync + 'static>,
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeFunction")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

#[turbo_tasks::value_impl]
impl NativeFunction {
    #[turbo_tasks::constructor]
    pub fn new(
        name: String,
        task_argument_options: impl Fn() -> Vec<TaskArgumentOptions> + Send + Sync + 'static,
        bind_fn: impl (Fn(Vec<SlotRef>) -> Result<NativeTaskFn>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            name,
            task_argument_options: Box::new(task_argument_options),
            bind_fn: Box::new(bind_fn),
        }
    }

    pub fn bind(&self, inputs: Vec<SlotRef>) -> Result<NativeTaskFn> {
        (self.bind_fn)(inputs)
    }
}

impl PartialEq for &'static NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        (*self as *const NativeFunction) == (*other as *const NativeFunction)
    }
}

impl Hash for &'static NativeFunction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&(*self as *const NativeFunction), state);
    }
}