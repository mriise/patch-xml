mod input;
mod output;

pub use output::svd::*;

pub fn get_parent_directory(path: &String) -> String {
    std::path::Path::new(path)
        .parent()
        .expect("Expected file in a directory.")
        .to_str()
        .unwrap()
        .to_string()
}

pub fn read_svd_config(config_path: &String) -> output::svd::Svd {
    let base_conf_path = get_parent_directory(config_path);
    let yaml = input::config::Config::read(&config_path);
    println!(
        "Config read result: {}",
        serde_yaml::to_string(&yaml).unwrap()
    );
    let mut svd = input::svd::Svd::read(base_conf_path + &"/" + yaml.svd.as_str());
    yaml.merge_into(&mut svd);
    output::svd::Svd::from(&svd)
}
