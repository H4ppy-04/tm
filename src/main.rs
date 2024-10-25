mod color;
mod task;

use chrono::format::ParseError;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use color::ColorVariant;
use task::{list_tasks, Task};

const DEFAULT_FLAVOR: ColorVariant = ColorVariant::Mocha;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Specify a color palette
    #[arg(long, num_args=0..=1)]
    palette: Option<ColorVariant>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new task
    New {
        #[arg(long, value_name = "task-name")]
        /// The name of the task
        name: String,
        #[arg(long, value_name = "due-date")]
        /// The date the task is due
        due: Option<String>,
    },
}

fn date_from_str(date: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
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
            } else {
                tasks.push(Task::new(name, None))
            }
        }
    }

    if !tasks.is_empty() {
        list_tasks(tasks, parsed_args.palette.unwrap_or_default().into());
    }
}
