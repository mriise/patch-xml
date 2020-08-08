pub mod input;
pub mod output;

use input::config::Config;
use input::svd::Svd;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "usage: {} <configuration_file> <result.svd>",
            args.get(0).unwrap()
        );
        std::process::exit(1);
    }
    let yaml_path = args.get(1).unwrap();
    let _result_path = args.get(2).unwrap();
    let base_conf_path = std::path::Path::new(yaml_path)
        .parent()
        .expect("Expected configuration file in a directory.")
        .to_str()
        .unwrap()
        .to_string();
    let yaml = Config::read(&yaml_path);
    println!(
        "Config read result: {}",
        serde_yaml::to_string(&yaml).unwrap()
    );
    let mut svd = Svd::read(base_conf_path + &"/" + yaml.svd.as_str());
    yaml.merge_into(&mut svd);
    //Serialization support of serde-xml-rs does not support current structure
    //  Maybe changing to another xml-serde library...
    //svd.write(result_path);
}
