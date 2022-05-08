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
      show_creation_date:     args.iter().any(|x| x == "-c" || x == "--creation"),
      show_modification_date: args.iter().any(|x| x == "-m" || x == "--modification"),
      show_access_date:       args.iter().any(|x| x == "-a" || x == "--access"),
      show_size:              args.iter().any(|x| x == "-s" || x == "--size"),
      show_sys:               args.iter().any(|x| x == "--show-sys"),
      format: {
        let si: bool = args.iter().any(|x| x == "--si");
        let iec: bool = args.iter().any(|x| x == "--iec");
          
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
