use std::env;
use std::fs;
use std::fs::{ReadDir,DirEntry};
use std::path::Path;

mod printentry;
use printentry::PrintEntry;

mod flags;
use flags::Flags;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() > 1 && (args[1].to_lowercase() == "help" || args[1].to_lowercase() == "--help") {
    println!["=== [Help] {} v{}",env!["CARGO_PKG_NAME"],env!["CARGO_PKG_VERSION"]];
    print!["{}",flags::get_help()];
    return
  }
  
  let flags: Flags = match Flags::new(&args[1..]) {
    Ok(flags) => flags,
    Err(error) => {
      eprintln!["{}",error];
      return
    }
  };
  
  let path: &Path;
  if args.len() != 1 && Path::new(&args[args.len() - 1]).exists() {
    path = Path::new(&args[args.len() - 1]);
  } else {
    path = Path::new(".");
  }
  
  match list_dir(path,&flags) {
    Ok(_) => {},
    Err(error) => eprintln!["{}",error]
  };
}

fn list_dir(path: &Path,flags: &Flags) -> Result<(),String> {
  let dir_iterator: ReadDir = match fs::read_dir(path) {
    Ok(iterator) => iterator,
    Err(error) => return Err(format!["Failed to open {} [Error: {}]",path.display(),error])
  };
  
  printentry::print_header(flags);

  let mut entries: Vec<PrintEntry> = Vec::new();
  for dir_entry in dir_iterator {
    let dir_entry: DirEntry = match dir_entry {
      Ok(dir_entry) => dir_entry,
      Err(_) => continue
    };
    
    entries.push(PrintEntry::new(&dir_entry.path()));
  }
  
  entries.sort();
  
  for entry in entries {
    entry.print(flags);
  }

  return Ok(())
}
