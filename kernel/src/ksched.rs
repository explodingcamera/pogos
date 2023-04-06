use alloc::{vec, vec::Vec};
use pasts::{prelude::*, Executor, Loop};

use crate::println;

#[derive(Debug)]
pub enum TaskResult {
    Exit,
}

pub struct KernelScheduler {
    tasks: Vec<BoxNotify<'static, TaskResult>>,
}

impl KernelScheduler {
    fn completed(&mut self, (id, val): (usize, TaskResult)) -> Poll<bool> {
        println!("Received message from completed task: {val:?}");

        self.tasks.swap_remove(id);

        if self.tasks.is_empty() {
            Ready(true)
        } else {
            Pending
        }
    }

    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn with_task(mut self, task: BoxNotify<'static, TaskResult>) -> Self {
        self.tasks.push(task);
        self
    }

    pub async fn run(&mut self) {
        Loop::new(self)
            .on(|s| &mut s.tasks[..], Self::completed)
            .await;
    }

    pub fn block_on_run(mut self) {
        Executor::default().block_on(async move {
            self.run().await;
        });
    }
}
