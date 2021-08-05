use cmdline::Opt;

pub mod cmdline;
pub mod config;
mod selector;

fn main() {
    let opt = Opt::parse();
    selector::run(&opt)
    // dbg!(opt);
    // TODO Error Handler
    // if let Err(e) = selector::run(&opt) {
    //     eprintln!("App error: {}", e);
    //     process::exit(1);
    // }
}
