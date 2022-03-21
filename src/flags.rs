use crate::printentry::Format;

pub struct Flags {
  pub show_creation_date: bool,
  pub show_modification_date: bool,
  pub show_access_date: bool,
  pub show_size: bool,
  pub show_sys: bool,
  pub format: Format
}

const DEFAULT_FORMAT: Format = Format::Iec;

impl Flags {
  pub fn new(args: &[String]) -> Result<Self,String> {
    let flags: Self = Self {
      show_creation_date:     args[..].contains(&"-c".to_string()) || args[..].contains(&"--creation".to_string()),
      show_modification_date: args[..].contains(&"-m".to_string()) || args[..].contains(&"--modification".to_string()),
      show_access_date:       args[..].contains(&"-a".to_string()) || args[..].contains(&"--access".to_string()),
      show_size:              args[..].contains(&"-s".to_string()) || args[..].contains(&"--size".to_string()),
      show_sys:               args[..].contains(&"--show-sys".to_string()),
      format: {
        let si: bool = args[..].contains(&"--si".to_string());
        let iec: bool = args[..].contains(&"--iec".to_string());
          
        if si && iec {
          return Err(format!["Either SI format or IEC"])
        }
          
        if si {
          Format::Si
        } else if iec {
          Format::Iec
        } else {
          DEFAULT_FORMAT
        }
      }
    };
    
    return Ok(flags)
  }
}

pub fn get_help() -> &'static str {
  concat![
    "Flags:\n",
    "  -c | --creation: Show creation date\n",
    "  -m | --modification: Show modification date\n",
    "  -a | --access: show accessed date\n",
    "  -s | --size: Show size\n",
    "  --show-sys: Show dotfiles\n",
    "  --si: Show size in SI format\n",
    "  --iec: Show size in IEC format\n"
  ]
}
