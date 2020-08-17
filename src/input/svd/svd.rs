use crate::output as output_svd;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Svd {
    pub device: super::Device,
}

impl Svd {
    pub fn to_output(&self) -> output_svd::Svd {
        output_svd::Svd {
            device: self.device.to_output(),
        }
    }
    pub fn read(svd_path: String) -> Svd {
        Svd {
            device: serde_xml_rs::from_str(
                fs::read_to_string(svd_path)
                    .expect("Error while reading svd file")
                    .as_str(),
            )
            .expect("Error while parsing SVD file"),
        }
    }
    #[allow(dead_code)] // At the moment, serde_xml_rs::to_string is not fully supported
    pub fn write(&self, path: &String) {
        let write_result = match serde_xml_rs::to_string(self) {
            Ok(svd_string) => fs::write(&path, svd_string.as_bytes()),
            Err(e) => panic!("Could not serialize SVD struct: {}", e),
        };
        if write_result.is_err() {
            panic!(
                "Error while writing SVD to disk: {}",
                write_result.unwrap_err()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(2, 2);
    }
}
