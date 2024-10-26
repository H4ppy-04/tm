mod color;
mod task;

use chrono::format::ParseError;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use color::ColorVariant;
use serde_json::{Map, Value};
use task::{list_tasks, Task};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Specify a color palette
    #[arg(long, num_args=0..=1)] // NOTE: is there a better way to do this?
    palette: Option<ColorVariant>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new task
    New {
        #[arg(long, value_name = "description")]
        /// The name of the task
        name: String,
        #[arg(long, value_name = "date")]
        /// The date the task is due
        due: Option<String>,
    },
}

fn date_from_str(date: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
}

// This is just a context state object that can be passed around for convenience purposes.
pub struct State {
    args: Args,
    tasks: Vec<Task>,
    map: Map<String, Value>,
}

impl State {
    fn new() -> Self {
        Self {
            args: Args::parse(),
            tasks: Vec::new(),
            map: Map::new(),
        }
    }

    fn list_tasks(&mut self) {
        if !self.tasks.is_empty() {
            list_tasks(
                &mut self.tasks,
                self.args.palette.clone().unwrap_or_default().into(),
            );
        }
    }

    fn serialize_tasks(&mut self) {
        for task in self.tasks.iter_mut() {
            let key = task.due.clone().unwrap_or_default();
            let value = serde_json::to_value(&task).expect("Failed to serialize task.");
            self.map.insert(key, value);
        }
    }

    fn parse(&mut self) {
        match self.args.command {
            Commands::New { ref name, ref due } => {
                if let Some(value) = due {
                    match date_from_str(value) {
                        Ok(x) => self.tasks.push(Task::new(name.to_string(), Some(x))),
                        Err(err) => {
                            panic!("Invalid date format: format is YYYY-MM-DD ({})", err)
                        }
                    }
                } else {
                    self.tasks.push(Task::new(name.to_string(), None))
                }
            }
        }
    }
}

fn main() {
    let mut state = State::new();

    state.parse();
    state.list_tasks();
    state.serialize_tasks();
}
