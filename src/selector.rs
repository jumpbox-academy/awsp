#[cfg(test)]
mod tests {

    use std::{env};
    use super::*;

    #[test]
    fn select_profile_with_selection() {
        select_profile("ped");
        let result = env::var("AWS_PROFILE").unwrap();
        let expect = String::from("ped");
        assert_eq!(expect, result);
    }

    #[test]
    fn select_region_with_selection() {
        select_region("ped");
        let result = env::var("AWS_DEFAULT_REGION").unwrap();
        let expect = String::from("ped");
        assert_eq!(expect, result);
    }


}

use awsp::{ default_config_location, parse_config_file };

use std::convert::TryInto;
use std::path::PathBuf;
use std::{error::Error, env};
use std::process::{self, Command};
use crate::cmdline::Opt;
use sysinfo::get_current_pid;
use dialoguer::{theme::ColorfulTheme, Select};
use sysinfo::{System, SystemExt, Signal, ProcessExt};

#[cfg(unix)]
use std::os::unix::prelude::CommandExt;

const REGIONS: &'static [&str] = &[
    "us-east-2      | Ohio",
    "us-east-1      | N. Virginia",
    "us-west-1      | N. California",
    "us-west-2      | Oregon",
    "ap-south-1     | Mumbai",
    "ap-northeast-3 | Osaka-Local",
    "ap-northeast-2 | Seoul",
    "ap-northeast-1 | Tokyo",
    "ap-southeast-1 | Singapore",
    "ap-southeast-2 | Sydney",
    "ca-central-1   | Central",
    "cn-north-1     | Beijing",
    "cn-nortwest-1  | Ningxia",
    "eu-central-1   | Frankfurt",
    "eu-west-1      | Ireland",
    "eu-west-2      | London",
    "eu-west-3      | Paris",
    "eu-north-1     | Stockholm",
    "sa-east-1      | São Paulo",
];

pub fn run(opt: &Opt) -> Result<(), Box<dyn Error>> {

    let location = default_config_location().unwrap();
    let config_file = parse_config_file(location.as_path()).unwrap();
    
    let mut profile_list = vec![];
        
    for profile in config_file.keys() {
        dbg!(profile);
        profile_list.push(profile);
    }

    let profile_list = profile_list.as_slice().try_into().unwrap();
    
    dbg!("kai: {}",&profile_list);

    if !opt.region {
        let default_region = env::var("AWS_PROFILE").unwrap();
        let display_prompt = format!("profile (current: {} )", default_region);
        let selection = display(display_prompt, profile_list, 0);    
        dbg!(profile_list[selection]);
        select_profile(profile_list[selection]);
    }
    let default_profile = env::var("AWS_DEFAULT_REGION").unwrap();
    let display_prompt = format!("region (current: {} )", default_profile);
    let selection = display(display_prompt, REGIONS, 0);
    dbg!(REGIONS[selection]);
    select_region(REGIONS[selection]);

    // let shell =  env::var("SHELL").unwrap();

    let current_pid = get_current_pid().ok().unwrap();
    let s = System::new_all();
    let current_process = s.process(current_pid).unwrap();
    let parent_pid = current_process.parent().unwrap();
    let parent_process = s.process(parent_pid).unwrap();
    let shell_path = parent_process.exe();
    if cfg!(window) {
        let mut child = Command::new(shell_path).spawn().unwrap();
        child.wait().expect("wait msg");
        // child.kill().expect("kill msg");
        dbg!(parent_pid);
        dbg!(current_pid);
        s.process(parent_pid).unwrap().kill(Signal::Kill);
    } else {
        dbg!("kai");
        dbg!(parent_pid);
        dbg!(current_pid);
        Command::new(shell_path).exec();
        s.process(parent_pid).unwrap().kill(Signal::Kill);
    }

    dbg!("kai");

    
    // let path = find_shell().expect("cannot find shell path");
    // dbg!(&path);

    // use function
    // let mut child = Command::new(path).spawn().unwrap();

    // let current_pid = get_current_pid().ok().unwrap();
    // let s = System::new_all();
    // let current_process = s.process(current_pid).unwrap();
    // current_process.kill(Signal::Kill);


    Ok(())
}
fn find_shell()-> Option<PathBuf> {
    let current_pid = get_current_pid().ok()?;
    let s = System::new_all();
    let current_process = s.process(current_pid)?;
    let parent_pid = current_process.parent()?;
    let parent_process = s.process(parent_pid)?;
    let shell_path = parent_process.exe();
    Some(shell_path.to_path_buf())
}
fn display<T: ToString>(display_prompt: String, list: &[T], default: usize) -> usize {
    Select::with_theme(&ColorfulTheme::default())
            .with_prompt(display_prompt)
            .default(default)
            .items(list)
            .paged(true)
            .interact()
            .unwrap()
}

fn select_profile(profile: &str){
    dbg!(profile);
    env::set_var("AWS_PROFILE", profile);
    dbg!(env::var("AWS_PROFILE").unwrap());
}

fn select_region(region: &str) {
    dbg!(region);
    env::set_var("AWS_DEFAULT_REGION", region);
}