//This file follows the definition as described in https://www.keil.com/pack/doc/CMSIS/SVD/html/index.html
use crate::input::svd as input_svd;
use serde::{Serialize};

use super::{Device};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Svd {
    pub device: Device,
}

impl Svd {
    pub fn from(input_svd: &input_svd::Svd) -> Svd {
        Svd {
            device: Device::from(input_svd)
        }
    }
}
