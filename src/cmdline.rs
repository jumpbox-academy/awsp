
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "aws ops", 
    about = "AWS Configure Profile"
)]
pub struct Opt {

    #[structopt(
        short = "r",
        long = "region",
        help = "Region Selector"
    )]
    pub region: bool,
    // TODO add explicit profile
    // #[structopt(short = "p", long = "profile")]
    // pub profile: String,
  
    #[structopt(
        short = "v", 
        long = "version",
        help = "Print version info and exit"
    )]
    pub version: bool,
    
    #[structopt(
        short = "c", 
        long = "config", 
        parse(from_os_str),
        help = "Override an aws configuration file (default = ~/.aws/config)"
    )]
    pub config: Option<PathBuf>,
}

impl Opt {
    pub fn parse() -> Opt {
        Opt::from_args()
    }
}
