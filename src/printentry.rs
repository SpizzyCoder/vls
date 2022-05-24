use std::cmp::Ordering;
use std::fs;
use std::fs::Metadata;
use std::path::Path;
use crate::{Args,Format};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod colors;

pub struct PrintEntry {
  obj_type: char,
  name: String,
  creation_date: String,
  modification_date: String,
  access_date: String,
  size: u64,
  error: Option<String>,
  color: Color,
}

impl PartialEq for PrintEntry {
  fn eq(&self, other: &Self) -> bool {
    if self.obj_type == other.obj_type && self.name == other.name {
      return true
    }

    return false
  }
}

impl Eq for PrintEntry {}

impl Ord for PrintEntry {
  fn cmp(&self, other: &Self) -> Ordering {
    let self_tuple: (char,&str) = (self.obj_type,&self.name);
    let other_tuple: (char,&str) = (other.obj_type,&other.name);

    return self_tuple.cmp(&other_tuple)
  }
}

impl PartialOrd for PrintEntry {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

const DATE_TIME_NODATA: &'static str = "---------- --:--:--";
const DATE_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const SI_UNITS: &'static [&'static str] = &[" B","KB","MB","GB","TB"];
const IEC_UNITS: &'static [&'static str] = &["  B","KiB","MiB","GiB","TiB"];
const SI_NUM_LEN: usize = 6; // 999.00
const IEC_NUM_LEN: usize = 7; // 1023.00
const SI_STRLEN: usize = 9; // 999.00 MB
const IEC_STRLEN: usize = 11; // 1023.00 MiB

impl PrintEntry {
  pub fn new(path: &Path) -> PrintEntry {
    let mut print_entry: PrintEntry = PrintEntry {
      obj_type: {
        if path.is_file() {
          'F'
        } else if path.is_dir() {
          'D'
        } else if path.is_symlink() {
          'S'
        } else {
          'U'
        }
      },
      creation_date: format!["{}",DATE_TIME_NODATA],
      modification_date: format!["{}",DATE_TIME_NODATA],
      access_date: format!["{}",DATE_TIME_NODATA],
      size: 0,
      name: {
        let mut res: String = String::from("---");

        if let Some(os_str) = path.file_name() {
          if let Some(string) = os_str.to_str() {
            res = string.to_string();
          }
        }

        res
      },
      error: None,
      color: colors::get_color(path),
    };
    
    let metadata: Metadata = match fs::metadata(path) {
      Ok(metadata) => metadata,
      Err(error) => {
        print_entry.error = Some(format!["{}",error]);
        return print_entry
      }
    };
    
    if let Ok(time) = metadata.created() {
      let datetime: chrono::DateTime<chrono::Local> = time.into();
      print_entry.creation_date = format!["{}",datetime.format(DATE_TIME_FORMAT)]
    }
    
    if let Ok(time) = metadata.modified() {
      let datetime: chrono::DateTime<chrono::Local> = time.into();
      print_entry.modification_date = format!["{}",datetime.format(DATE_TIME_FORMAT)]
    }
    
    if let Ok(time) = metadata.accessed() {
      let datetime: chrono::DateTime<chrono::Local> = time.into();
      print_entry.access_date = format!["{}",datetime.format(DATE_TIME_FORMAT)]
    }
    
    print_entry.size = metadata.len();
    print_entry.error = None;
    
    return print_entry
  }
  
  pub fn print(&self,args: &Args) {
    if !args.sys && self.name.starts_with(".") {
      return
    }
    
    print!["{} ",self.obj_type];
  
    if args.creation_date {
      print!["[{}] ",self.creation_date];
    }
    
    if args.modification_date {
      print!["[{}] ",self.modification_date];
    }
    
    if args.access_date {
      print!["[{}] ",self.access_date];
    }
    
    if args.size {
      print!["[{}] ",get_human_readable_size(args.format,self.size)];
    }
    
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(self.color)));
    print!["{}",self.name];
    
    if self.error.is_some() {
      let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
      print![" -> {}",self.error.as_ref().unwrap()];
    }
    
    let _ = stdout.reset();
    
    println![];
  }
}

pub fn print_header(args: &Args) {
  print!["  "];
  
  if args.creation_date {
    print!["{:^22}","Created"];
  }
  
  if args.modification_date {
    print!["{:^22}","Last modified"];
  }
  
  if args.access_date {
    print!["{:^22}","Last accessed"];
  }
  
  if args.size {
    match args.format {
      Format::Iec => print!["{:^1$}","Size",IEC_STRLEN + 3],
      Format::Si => print!["{:^1$}","Size",SI_STRLEN + 3],
    };
  }
  
  println!["Name"];
}

fn get_human_readable_size(format: Format,bytes: u64) -> String {
  let mut res_string: String = String::new();
  
  // &[&str] - Array of format strings (MiB,GiB, etc)
  // f64     - Maximum number for wrapping
  // usize   - Length of num
  let format_infos: (&[&str],f64,usize) = match format {
    Format::Iec => (
      IEC_UNITS,
      1024.0,
      IEC_NUM_LEN
    ),
    Format::Si => (
      SI_UNITS,
      1000.0,
      SI_NUM_LEN
    ),
  };
  
  let mut divided_bytes: f64 = bytes as f64;
  for i in 0..format_infos.0.len() {
    if divided_bytes < format_infos.1 {
      res_string = format!["{0:1$.2} {2:}",divided_bytes,format_infos.2,format_infos.0[i]];
      break;
    } else {
      divided_bytes /= format_infos.1;
    }
  }
  
  return res_string
}
