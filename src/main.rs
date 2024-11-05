use clap::{Parser, Subcommand};
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};

/// Simple todo list
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args
{
    #[command(subcommand)]
    commands: Commands
}

#[derive(Subcommand)]
enum Commands
{
    /// Show list
    Show {},

    /// Add task to list
    Add {
        /// Task to add
        task: String,
    },

    /// Remove task from list
    Del {
        /// Task to remove
        task: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.commands
    {
        Commands::Add { task } => {
            println!("Adding task {}", task);
            let mut file = fs::File::options().write(true).read(false).create(true).open("todo.txt").expect("Cannot open file.");
            file.seek(SeekFrom::End(0)).expect("Cannot seek file.");
            file.write_all(format!("{}\n", task).as_bytes()).expect("Cannot write to file.");
        },
        Commands::Del { task } => {
            println!("Deleting task {}", task);
            let mut file = fs::File::options().write(true).read(true).create(false).open("todo.txt").expect("Cannot open file.");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Cannot read file.");
            let content = content.lines().filter(|line| line.trim() != task.trim()).collect::<Vec<&str>>();
            file.set_len(0).ok();
            file.seek(SeekFrom::Start(0)).ok();
            file.write_all(format!("{}\n", content.join("\n")).as_bytes()).expect("Cannot write to file.");
        },
        Commands::Show { .. } => {
            let mut file = fs::File::options().write(true).read(true).create(false).open("todo.txt").expect("Cannot open file.");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Cannot read file.");
            for (index, line) in content.lines().enumerate()
            {
                if line.is_empty() && content.lines().count() == 1
                {
                    println!("No tasks.");
                }
                else
                {
                    println!("{}: {}", index+1, line);
                }
            }
        }
    }
}
