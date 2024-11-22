use clap::{Parser, Subcommand};
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;


/// Simple todo list cli
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
        /// Task, index, or range (eg. 1-5) to remove
        task: String,
    },
}

fn get_todo_path() -> Result<PathBuf, &'static str>
{
    let todo_path_parent = std::env::current_exe().unwrap_or("".into());

    if todo_path_parent.to_str().unwrap().is_empty()
    {
        return Err("Cannot get directory of executable to find todo.txt.");
    }

    let todo_path = todo_path_parent.parent().unwrap().join(PathBuf::from("todo.txt"));

    Ok(todo_path)
}

fn main()
{
    let args = Args::parse();

    match args.commands
    {
        Commands::Add { task } => {
            match add(task) {
                Ok(_) => {},
                Err(err) => eprintln!("{}", err),
            }
        },
        Commands::Del { task } => {
            match del(task) {
                Ok(_) => {},
                Err(err) => eprintln!("{}", err),
            }
        },
        Commands::Show { .. } => {
            match show() {
                Ok(_) => {},
                Err(err) => eprintln!("{}", err),
            }
        }
    }
}

fn add(task: String) -> Result<(), String>
{
    let todo_path = match get_todo_path() {
        Ok(path) => path,
        Err(err) => return Err(err.into()),
    };

    let mut file = match fs::File::options().write(true).read(false).create(true).open(&todo_path) {
        Ok(f) => f,
        Err(_) => return Err(format!("Cannot open file: {:#?}.", todo_path)),
    };

    if file.seek(SeekFrom::End(0)).is_err()
    {
        return Err(format!("Cannot seek file: {:#?}.", todo_path));
    }

    if file.write_all(format!("{}\n", task).as_bytes()).is_err()
    {
        return Err(format!("Cannot write to file: {:#?}.", todo_path));
    }

    println!("Adding task {}", task);

    Ok(())
}

fn del(task: String) -> Result<(), String>
{
    let todo_path = match get_todo_path() {
        Ok(path) => path,
        Err(err) => return Err(err.into()),
    };

    let mut file = match fs::File::options().write(true).read(true).create(false).open(&todo_path) {
        Ok(f) => f,
        Err(_) => return Err(format!("Cannot open file: {:#?}.", todo_path)),
    };

    let mut content = String::new();
    if file.read_to_string(&mut content).is_err()
    {
        return Err(format!("Cannot read file: {:#?}.", todo_path));
    }

    let mut new_content;
    if let Ok(index) = task.parse::<usize>()
    {
        new_content = content.lines().collect::<Vec<&str>>();

        if index <= 0 || index > new_content.len()
        {
            return Err(format!("Index {} is out of range {}-{}", index, 1, new_content.len()));
        }

        new_content.remove(index - 1);

    }
    else if let Some(range) = task.split_once("-")
    {
        if let Ok(start) = range.0.parse::<usize>()
        {
            if let Ok(end) = range.1.parse::<usize>()
            {
                new_content = content.lines().collect::<Vec<&str>>();
                if start > end
                {
                    return Err(format!("Start cannot be greater than end, {}-{}.", start, end));
                }
                if start <= 0
                {
                    return Err("Start is less than 1.".into());
                }
                if end > new_content.len()
                {
                    return Err(format!("End cannot be greater than max, {}>{}.", end, new_content.len()));
                }

                for index in (start-1..end).rev()
                {
                    new_content.remove(index);
                }
            }
            else
            {
                new_content = content.lines().filter(|line| line.trim() != task.trim()).collect::<Vec<&str>>();
            }
        }
        else
        {
            new_content = content.lines().filter(|line| line.trim() != task.trim()).collect::<Vec<&str>>();
        }
    }
    else
    {
        new_content = content.lines().filter(|line| line.trim() != task.trim()).collect::<Vec<&str>>();
    }
    if file.set_len(0).is_err() { return Err(format!("Cannot change file contents: {:#?}.", todo_path)); }
    if new_content.is_empty()
    {
        println!("Deleting task {}", task);
        return Ok(());
    }
    if file.seek(SeekFrom::Start(0)).is_err() { return Err(format!("Cannot change file contents: {:#?}.", todo_path)); }

    if file.write_all(format!("{}\n", new_content.join("\n")).as_bytes()).is_err() { return Err(format!("Cannot write to file: {:#?}.", todo_path)); }

    println!("Deleting task {}", task);

    Ok(())
}

fn show() -> Result<(), String>
{
    let todo_path = match get_todo_path() {
        Ok(path) => path,
        Err(err) => return Err(err.into()),
    };

    let mut file = match fs::File::options().write(true).read(true).create(false).open(&todo_path) {
        Ok(f) => f,
        Err(_) => return Err(format!("Cannot open file: {:#?}.", todo_path)),
    };

    let mut content = String::new();

    if file.read_to_string(&mut content).is_err()
    {
        return Err(format!("Cannot read file: {:#?}.", todo_path));
    }

    if content.trim().is_empty()
    {
        println!("No tasks.");
    }
    else
    {
        for (index, line) in content.lines().enumerate()
        {
            println!("{}: {}", index+1, line);
        }
    }

    Ok(())
}