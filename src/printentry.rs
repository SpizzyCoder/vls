use std::fs;
use std::fs::Metadata;
use std::path::Path;
use crate::flags::Flags;

#[derive(Clone)]
#[derive(Copy)]
pub enum Format {
  Si,
  Iec
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
pub struct PrintEntry {
  obj_type: char,
  name: String,
  creation_date: String,
  modification_date: String,
  access_date: String,
  size: u64,
  error: Option<String>
}

const DATE_TIME_NODATA: &'static str = "---------- --:--:--";
const DATE_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const SI_UNITS: &'static [&'static str] = &["B","KB","MB","GB","TB"];
const IEC_UNITS: &'static [&'static str] = &["B","KiB","MiB","GiB","TiB"];

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
      name: format!["{}",path.file_name().unwrap().to_str().unwrap()],
      error: None
    };
    
    let metadata: Metadata = match fs::metadata(path) {
      Ok(metadata) => metadata,
      Err(error) => {
        print_entry.error = Some(format!["{}",error]);
        return print_entry
      }
    };
    
    if let Ok(time) = metadata.created() {
      let datetime: chrono::DateTime<chrono::Utc> = time.into();
      print_entry.creation_date = format!["{}",datetime.format(DATE_TIME_FORMAT)]
    }
    
    if let Ok(time) = metadata.modified() {
      let datetime: chrono::DateTime<chrono::Utc> = time.into();
      print_entry.modification_date = format!["{}",datetime.format(DATE_TIME_FORMAT)]
    }
    
    if let Ok(time) = metadata.accessed() {
      let datetime: chrono::DateTime<chrono::Utc> = time.into();
      print_entry.access_date = format!["{}",datetime.format(DATE_TIME_FORMAT)]
    }
    
    print_entry.size = metadata.len();
    print_entry.error = None;
    
    return print_entry
  }
  
  pub fn print(&self,flags: &Flags) {
    if !flags.show_sys && self.name.starts_with(".") {
      return
    }
    
    print!["{} ",self.obj_type];
  
    if flags.show_creation_date {
      print!["[{}] ",self.creation_date];
    }
    
    if flags.show_modification_date {
      print!["[{}] ",self.modification_date];
    }
    
    if flags.show_access_date {
      print!["[{}] ",self.access_date];
    }
    
    if flags.show_size {
      print!["[{}] ",get_human_readable_size(flags.format,self.size)];
    }
    
    print!["{}",self.name];
    
    if self.error.is_some() {
      print![" -> {}",self.error.as_ref().unwrap()];
    }
    
    println![];
  }
}

pub fn print_header(flags: &Flags) {
  print!["  "];
  
  if flags.show_creation_date {
    print!["{:22}","Created"];
  }
  
  if flags.show_modification_date {
    print!["{:22}","Last modified"];
  }
  
  if flags.show_access_date {
    print!["{:22}","Last accessed"];
  }
  
  if flags.show_size {
    match flags.format {
      Format::Si => print!["{:12}","Size"],
      Format::Iec => print!["{:13}","Size"]
    };
  }
  
  println!["Name"];
}

fn get_human_readable_size(format: Format,bytes: u64) -> String {
  let mut res_string: String = String::new();
  
  let format_infos: (&[&str],f64,usize,usize) = match format {
    Format::Si => (SI_UNITS,1000.0,2,9),
    Format::Iec => (IEC_UNITS,1024.0,3,10)
  };
  
  let mut divided_bytes: f64 = bytes as f64;
  for i in 0..format_infos.0.len() {
    if divided_bytes < format_infos.1 {
      res_string = format!["{:.2} {2:>1$}",divided_bytes,format_infos.2,format_infos.0[i]];
      while res_string.len() < format_infos.3 {
        res_string.insert(0,' ');
      }
      break;
    } else {
      divided_bytes /= format_infos.1;
    }
  }
  
  return res_string
}
