#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    use std::path::Path;

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

    #[test]
    fn string_eq() {
        let parent = Path::new("target/debug/kaiped");
        let current = Path::new("/usr/local/bin/awsp");
        let parent = parent.to_str().unwrap();
        let current = current.to_str().unwrap();
        // dbg!(parent.ne(current));
        assert!(parent.eq(current));
    }
}

use awsp::{default_config_location, parse_config_file};

use crate::cmdline::Opt;
use dialoguer::{theme::ColorfulTheme, Select};
use std::convert::TryInto;
use std::path::{PathBuf};
use std::process::{Command};
use std::{env};
use sysinfo::get_current_pid;
use sysinfo::{ProcessExt, Signal, System, SystemExt};

// #[cfg(unix)]car
// use std::os::unix::prelude::CommandExt;

const REGIONS_DISPLAY: &'static [&str] = &[
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

const REGIONS: &'static [&str] = &[
    "us-east-2",
    "us-east-1",
    "us-west-1",
    "us-west-2",
    "ap-south-1",
    "ap-northeast-3",
    "ap-northeast-2",
    "ap-northeast-1",
    "ap-southeast-1",
    "ap-southeast-2",
    "ca-central-1",
    "cn-north-1",
    "cn-nortwest-1",
    "eu-central-1",
    "eu-west-1",
    "eu-west-2",
    "eu-west-3",
    "eu-north-1",
    "sa-east-1",
];

// TODO pub fn run(opt: &Opt) -> Result<(), Box<dyn Error>> {
pub fn run(opt: &Opt) {
    
    let location = default_config_location().unwrap();
    let config_file = parse_config_file(location.as_path()).unwrap();
    
    let mut profile_list = vec![];
        
    for profile in config_file.keys() {
        // dbg!(profile);
        profile_list.push(profile);
    }

    let profile_list = profile_list.as_slice().try_into().unwrap();

    // dbg!("kai: {}", &profile_list);

    if !opt.region {
        let default_region = match env::var("AWS_PROFILE") {
            Ok(aws_profile) => aws_profile,
            Err(_) => String::from("")
        };
        let display_prompt = format!("profile (current: {} )", default_region);
        let selection = display(display_prompt, profile_list, 0);
        // dbg!(profile_list[selection]);
        select_profile(profile_list[selection]);
    }
    
    let default_profile = match env::var("AWS_DEFAULT_REGION") {
        Ok(aws_region) => aws_region,
        Err(_) => String::from("")
    };

    let display_prompt = format!("region (current: {} )", default_profile);
    let selection = display(display_prompt, REGIONS_DISPLAY, 0);
    // dbg!(REGIONS[selection]);
    select_region(REGIONS[selection]);

    
    Command::new(find_shell().unwrap()).spawn().unwrap().wait().unwrap();
    //TODO Reuse Process in find shell
    let current_pid = get_current_pid().ok().unwrap();
    let s = System::new_all();
    let current_process = s.process(current_pid).unwrap();
    let parent_pid = current_process.parent().unwrap();
    s.process(parent_pid).unwrap().kill(Signal::Kill);

    // TODO handle error case result<(), Box<dyn Error>>
    // Ok(())
}

fn find_shell() -> Option<PathBuf> {
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

fn select_profile(profile: &str) {
    // dbg!(profile);
    env::set_var("AWS_PROFILE", profile);
    // dbg!(env::var("AWS_PROFILE").unwrap());
}

fn select_region(region: &str) {
    // dbg!(region);
    env::set_var("AWS_DEFAULT_REGION", region);
}

// TODO manage stack process when run awsp multiple
// fn is_terminate_previous_process() -> Option<bool> {
//     let s = System::new_all();
//     let current_pid = get_current_pid().ok()?;
//     let current_process = s.process(current_pid)?;
//     let current_path = current_process.exe();
//     let parent_pid = current_process.parent()?;
//     let parent_process = s.process(parent_pid)?;
//     let parent_of_parent_pid = parent_process.parent()?;
//     let parent_of_parent_process = s.process(parent_of_parent_pid)?;
//     let parent_of_parent_path = parent_of_parent_process.exe();
//     let current_path = current_path.to_str().unwrap();
//     let parent_of_parent_path = parent_of_parent_path.to_str().unwrap();
//     dbg!(current_path);
//     dbg!(parent_of_parent_path);
//     if current_path.eq(parent_of_parent_path) {
//         parent_of_parent_process.kill(Signal::Kill);
//         return Some(true);
//     }
//     Some(false)
// }
