use std::convert::TryInto;
use std::{error::Error, path::Path};
use std::fs;

use crate::cmdline::Opt;

use awsp::parse_config_file;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn run(opt: &Opt) -> Result<(), Box<dyn Error>> {
    let regions = &[
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

    //let contents = fs::read_to_string(".aws/config")?;
    let config_file = parse_config_file(Path::new("config")).unwrap();
    
    let mut profile_list = vec![];
        
    for profile in config_file.keys() {
        dbg!(profile);
        profile_list.push(profile);
    }

    let profile_list = profile_list.as_slice().try_into().unwrap();
    
    dbg!("kai: {}",&profile_list);

    if !opt.region {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your flavor")
            .default(0)
            .items(profile_list)
            .interact()
            .unwrap();
            dbg!(profile_list[selection]);
    }

    Ok(())
}