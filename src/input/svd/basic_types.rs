use crate::output::{
    AccessType, AddressBlock as OutputAddressBlock, DimElementGroup as OutputDimElementGroup,
    Protection, RegisterPropertiesGroup as OutputRegisterPropertiesGroup,
    SauRegionsConfigType as OutputSauRegionsConfigType, SvdConstant,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionsConfigType {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    #[serde(with = "SvdConstant")]
    pub base: u32,
    #[serde(with = "SvdConstant")]
    pub limit: u32,
    pub access: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStructItem {
    pub name: String,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPropertiesGroup {
    pub size: Option<NestedSvdConstant>,
    pub access: Option<AccessType>,
    pub protection: Option<Protection>,
    pub reset_value: Option<NestedSvdConstant>,
    pub reset_mask: Option<NestedSvdConstant>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimElementGroup {
    pub dim: Option<NestedSvdConstant>,
    pub dim_increment: Option<NestedSvdConstant>,
    pub dim_index: Option<NestedSvdConstant>,
    pub dim_name: Option<String>,
    pub dim_array_index: Option<NestedSvdConstant>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AddressBlock {
    #[serde(with = "SvdConstant")]
    pub offset: u32,
    #[serde(with = "SvdConstant")]
    pub size: u32,
    pub usage: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NestedSvdConstant {
    #[serde(rename = "$value", with = "SvdConstant")]
    pub value: u32,
}

impl AddressBlock {
    pub fn to_output(&self) -> OutputAddressBlock {
        OutputAddressBlock {
            offset: self.offset.clone(),
            size: self.size.clone(),
            usage: self.usage.clone(),
        }
    }
}

impl RegisterPropertiesGroup {
    pub fn to_output(&self) -> OutputRegisterPropertiesGroup {
        OutputRegisterPropertiesGroup {
            size: match &self.size {
                None => None,
                Some(s) => Some(s.value),
            },
            access: self.access.clone(),
            protection: self.protection.clone(),
            reset_value: match &self.reset_value {
                None => None,
                Some(rv) => Some(rv.value),
            },
            reset_mask: match &self.reset_mask {
                None => None,
                Some(rm) => Some(rm.value),
            },
        }
    }
}

impl DimElementGroup {
    pub fn to_output(&self) -> OutputDimElementGroup {
        OutputDimElementGroup {
            dim: match &self.dim {
                None => None,
                Some(d) => Some(d.value),
            },
            dim_increment: match &self.dim_increment {
                None => None,
                Some(di) => Some(di.value),
            },
            dim_index: match &self.dim_index {
                None => None,
                Some(di) => Some(di.value),
            },
            dim_name: self.dim_name.clone(),
            dim_array_index: match &self.dim_array_index {
                None => None,
                Some(dai) => Some(dai.value),
            },
        }
    }
}

impl SauRegionsConfigType {
    pub fn to_output(&self) -> OutputSauRegionsConfigType {
        OutputSauRegionsConfigType {
            enabled: self.enabled.clone(),
            name: self.name.clone(),
            base: self.base.clone(),
            limit: self.limit.clone(),
            access: self.access.clone(),
        }
    }
}
