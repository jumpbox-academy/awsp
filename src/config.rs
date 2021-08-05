// use std::path::PathBuf;
// use std::vec;

// #[cfg(test)]
// mod unit_test{
//     #[test]
//     fn new_config(){
//         let args = vec![];
//         let result = Config {
//             command: String::from("")
//         };
//         assert_eq!(result, new(&args))
//     }
// }

// pub struct Config {
//     pub command: String,
//     pub config_path: PathBuf
// }

// impl Config {

//     // const AWS_CONFIG_PAHT: &str = "/.aws/config";
//     pub fn new (args: &[String]) -> Result<Config, &str> {
//         const AWS_CONFIG_PATH: &str = ".aws/config";
//         let config  = Config {
//             command: String::from(""),
//             config_path:
//                 dirs::home_dir().unwrap().join(AWS_CONFIG_PATH),
//         };

//         if args.len() < 1 {
//             return Ok(Config {
//                 command: (String::from("")),
//                 config_path: None
//              })
//         }

//         Ok(Config {
//                 command: (String::from("")),
//                 flag: (String::from(""))
//         })
//     }

// }
