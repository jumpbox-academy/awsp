use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aws ops", about = "AWS Configure Profile")]
pub struct Opt {
    #[structopt(short = "r", long = "region")]  
    pub region: bool
}

impl Opt {
    pub fn parse() -> Opt {
        Opt::from_args()
    }
}