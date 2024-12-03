use std::fs;
use std::fs::ReadDir;
use std::path::Path;

mod printentry;
use printentry::PrintEntry;

use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Iec,
    Si,
}

/// Simple program to list directories
#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Show creation date
    #[arg(short, long)]
    creation_date: bool,

    /// Show modification date
    #[arg(short, long)]
    modification_date: bool,

    /// Show access date
    #[arg(short, long)]
    access_date: bool,

    /// Show size
    #[arg(short, long)]
    size: bool,

    /// Show system files (dotfiles)
    #[arg(long)]
    sys: bool,

    /// Show estimated size of folders (recursive)
    #[arg(short, long)]
    recursive: bool,

    /// Format of size
    #[arg(short, long, value_enum, default_value_t = Format::Iec)]
    format: Format,

    /// Path
    #[arg(default_value = ".")]
    path: String,
}

fn main() {
    let args: Args = Args::parse();

    let path: &Path = Path::new(&args.path);
    if !path.exists() {
        eprintln!["{} doesn't exist", args.path];
        return;
    }

    if let Err(error) = list_dir(path, &args) {
        eprintln!["{}", error];
    }
}

fn list_dir(path: &Path, args: &Args) -> Result<(), String> {
    let dir_iterator: ReadDir = match fs::read_dir(path) {
        Ok(iterator) => iterator,
        Err(error) => {
            return Err(format![
                "Failed to open {} [Error: {}]",
                path.display(),
                error
            ])
        }
    };

    let mut entries: Vec<PrintEntry> = dir_iterator
        .filter_map(|x| x.ok())
        .map(|x| PrintEntry::new(&x.path(), &args))
        .collect();

    if entries.len() == 0 {
        println!["Nothing"];
        return Ok(());
    }

    entries.sort_unstable();

    printentry::print_header(args);

    for entry in entries {
        entry.print(args);
    }

    return Ok(());
}
