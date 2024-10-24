use std::fmt::Display;

use chrono::format::ParseError;
use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        #[arg(long, value_name = "task-name")]
        name: String,
        #[arg(long, value_name = "due-date")]
        due: Option<String>,
    },
}

fn date_from_str(date: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
}

struct Task {
    name: String,
    due: Option<NaiveDate>,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.due {
            Some(value) => write!(f, "Task: {} (due {})", self.name, value),
            None => write!(f, "Task: {}", self.name),
        }
    }
}

impl Task {
    fn positive_date_delta(date: NaiveDate) -> bool {
        Local::now().date_naive() < date
    }

    fn new(name: String, due: Option<NaiveDate>) -> Self {
        if due.is_some_and(|x| !Self::positive_date_delta(x)) {
            panic!("Task date must be positive")
        }
        Self { name, due }
    }
}

fn main() {
    let parsed_args = Args::parse();
    let mut tasks: Vec<Task> = Vec::new();
    match parsed_args.command {
        Commands::New { name, due } => {
            if let Some(value) = due {
                match date_from_str(&value) {
                    Ok(x) => tasks.push(Task::new(name, Some(x))),
                    Err(err) => {
                        panic!("Date format is YYYY-MM-DD ({})", err)
                    }
                }
            }
        }
    }

    if !tasks.is_empty() {
        for task in tasks.iter() {
            print!("Added task: {}", task)
        }
    }
}
