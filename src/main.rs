use std::env;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

mod printentry;
use printentry::PrintEntry;

mod flags;
use flags::Flags;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() > 1 && (args[1].to_lowercase() == "-h" || args[1].to_lowercase() == "--help") {
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
  
  let mut entries: Vec<PrintEntry> = dir_iterator.filter_map(|x| x.ok())
                                                 .map(|x| PrintEntry::new(&x.path()))
                                                 .collect();
  entries.sort();
  
  entries.iter().for_each(|x| x.print(flags));

  return Ok(())
}
