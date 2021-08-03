use cmdline::Opt;
use std::{ process };

pub mod config;
pub mod cmdline;
mod selector;

fn main() {

    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args);

    let opt = Opt::parse();
    dbg!(&opt);
    // println!("{:?}", opt);

    if let Err(e) = selector::run(&opt) {
        eprintln!("App error:");
        process::exit(1);
    }
}