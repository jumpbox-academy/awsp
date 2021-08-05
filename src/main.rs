use cmdline::Opt;

pub mod cmdline;
pub mod config;
mod selector;

fn main() {
    let opt = Opt::parse();
    // dbg!(opt);
    selector::run(&opt);
    // TODO handle error
    // if let Err(e) = selector::run(&opt) {
    //     eprintln!("App error: {}", e);
    //     process::exit(1);
    // }

}
