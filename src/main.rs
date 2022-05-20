use std::fs;
use std::fs::ReadDir;
use std::path::Path;

mod printentry;
use printentry::PrintEntry;

use clap::Parser;
use clap::ArgEnum;

#[derive(ArgEnum,Clone,Copy,Debug)]
enum Format {
  Iec,
  Si,
}

/// Simple program to list directories
#[derive(Parser,Debug)]
#[clap(version,about)]
pub struct Args {
    /// Show creation date
    #[clap(short,long)]
    creation_date: bool,

    /// Show modification date
    #[clap(short,long)]
    modification_date: bool,

    /// Show access date
    #[clap(short,long)]
    access_date: bool,

    /// Show size
    #[clap(short,long)]
    size: bool,

    /// Show system files (dotfiles)
    #[clap(long)]
    sys: bool,

    /// Format of size
    #[clap(short,long,arg_enum,default_value_t = Format::Iec)]
    format: Format,

    /// Path
    #[clap(default_value = ".")]
    path: String,
}

fn main() {
  let args = Args::parse();

  let path: &Path = Path::new(&args.path);
  if !path.exists() {
    eprintln!["{} doesn't exist",args.path];
    return
  }
  
  match list_dir(path,&args) {
    Ok(_) => {},
    Err(error) => eprintln!["{}",error]
  };
}

fn list_dir(path: &Path,args: &Args) -> Result<(),String> {
  let dir_iterator: ReadDir = match fs::read_dir(path) {
    Ok(iterator) => iterator,
    Err(error) => return Err(format!["Failed to open {} [Error: {}]",path.display(),error])
  };
  
  printentry::print_header(args);
  
  let mut entries: Vec<PrintEntry> = dir_iterator.filter_map(|x| x.ok())
                                                 .map(|x| PrintEntry::new(&x.path()))
                                                 .collect();
  entries.sort();
  
  entries.iter().for_each(|x| x.print(args));

  return Ok(())
}
