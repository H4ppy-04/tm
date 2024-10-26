use catppuccin::Flavor;
use chrono::{Local, NaiveDate};
use serde::Serialize;
use std::fmt::Display;

use crate::color::to_ansi;

#[derive(Serialize)]
pub struct Task {
    pub name: String,
    pub due: Option<String>,
    pub dependencies: Vec<Self>,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.due {
            Some(value) => write!(f, "Task: {} (due {})", self.name, value),
            None => write!(f, "Task: {}", self.name),
        }
    }
}

impl Task {
    pub fn positive_date_delta(date: NaiveDate) -> bool {
        Local::now().date_naive() < date
    }

    pub fn new(name: String, due: Option<NaiveDate>) -> Self {
        if due.is_some_and(|x| !Self::positive_date_delta(x)) {
            panic!("Task date must be positive")
        }
        Self {
            name,
            due: due.map(|x| x.to_string()),
            dependencies: Vec::new(),
        }
    }

    // Add dependency to task
    pub fn _add_subtask(&mut self, task: Self) {
        self.dependencies.push(task);
    }
}

/// Sort tasks by dependencies
pub fn sort_tasks(tasks: &mut [Task]) -> std::slice::IterMut<'_, Task> {
    if !tasks.is_sorted_by(|a, b| a.dependencies.len() <= b.dependencies.len()) {
        tasks.sort_by(|a, b| a.dependencies.len().cmp(&b.dependencies.len()));
    }
    tasks.iter_mut()
}

pub fn list_tasks(tasks: &mut [Task], flavor: Flavor) {
    for task in sort_tasks(tasks) {
        match &task.due {
            Some(date) => println!(
                "{}: {} ({}) +{}",
                to_ansi(&flavor.colors.pink).bold().paint("Task"),
                to_ansi(&flavor.colors.text).normal().paint(&task.name),
                to_ansi(&flavor.colors.text)
                    .dimmed()
                    .paint(date.to_string()),
                to_ansi(&flavor.colors.peach)
                    .bold()
                    .paint(task.dependencies.len().to_string()),
            ),
            None => {
                println!(
                    "{}: {}",
                    to_ansi(&flavor.colors.pink).bold().paint("Task"),
                    to_ansi(&flavor.colors.text).normal().paint(&task.name),
                )
            }
        }
    }
}
