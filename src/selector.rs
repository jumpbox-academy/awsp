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

use std::convert::TryInto;
use std::{error::Error, path::Path, env};

use crate::cmdline::Opt;

use awsp::parse_config_file;
use dialoguer::{theme::ColorfulTheme, Select};

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

    let config_file = parse_config_file(Path::new("config")).unwrap();
    
    let mut profile_list = vec![];
        
    for profile in config_file.keys() {
        dbg!(profile);
        profile_list.push(profile);
    }

    let profile_list = profile_list.as_slice().try_into().unwrap();
    
    dbg!("kai: {}",&profile_list);

    if !opt.region {
        dbg!("test");
        select_profile("profile");
        let default_region = env::var("AWS_PROFILE").unwrap();
        let display_prompt = format!("region (current: {} )", default_region);
        let selection = display(display_prompt, profile_list, 0);    
        dbg!(profile_list[selection]);
        select_profile(profile_list[selection]);
    }
    select_region("ped");
    let default_profile = env::var("AWS_DEFAULT_REGION").unwrap();
    let display_prompt = format!("region (current: {} )", default_profile);
    let selection = display(display_prompt, REGIONS, 0);
    dbg!(REGIONS[selection]);
    select_region(REGIONS[selection]);
    
    Ok(())
}

fn display<T: ToString>(display_prompt: String, list: &[T], default: usize) -> usize {
    Select::with_theme(&ColorfulTheme::default())
            .with_prompt(display_prompt)
            .default(0)
            .items(list)
            .paged(true)
            .interact()
            .unwrap()
}

fn select_profile(profile: &str){
    env::set_var("AWS_PROFILE", profile);
}

fn select_region(region: &str) {
    env::set_var("AWS_DEFAULT_REGION", region);
}

