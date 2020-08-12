//This structure follows the definition as described in https://www.keil.com/pack/doc/CMSIS/SVD/html/index.html
use serde::Serialize;

use super::Device;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Svd {
    pub device: Device,
}
