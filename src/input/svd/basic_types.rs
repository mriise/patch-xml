use crate::output::{
    AccessType, AddressBlock as OutputAddressBlock, DimArrayIndex as OutputDimArrayIndex,
    DimElementGroup as OutputDimElementGroup, EnumValue, EnumeratedValue as OutputEnumeratedValue,
    Protection, RegisterPropertiesGroup as OutputRegisterPropertiesGroup,
    SauRegionType as OutputSauRegionType, SauRegionsConfigType as OutputSauRegionsConfigType,
    SvdConstant,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValue {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<u32>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionsConfigType {
    pub enabled: Option<bool>,
    pub protection_when_disabled: Option<Protection>,
    pub regions: Vec<SauRegionType>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionType {
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
    pub dim_array_index: Option<DimArrayIndex>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimArrayIndex {
    pub header_enum_name: Option<String>,
    pub enumerated_values: Vec<EnumeratedValue>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AddressBlock {
    #[serde(with = "SvdConstant")]
    pub offset: u32,
    #[serde(with = "SvdConstant")]
    pub size: u32,
    pub usage: String,
    pub protection: Option<Protection>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NestedSvdConstant {
    #[serde(rename = "$value", with = "SvdConstant")]
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BitRange {
    OffsetWidthStyle {
        bit_offset: u32,
        bit_width: Option<u32>,
    },
    LsbMsbStyle {
        lsb: u32,
        msb: u32,
    },
    BitRangePattern {
        bit_range: Option<String>,
    },
}

impl AddressBlock {
    pub fn to_output(&self) -> OutputAddressBlock {
        OutputAddressBlock {
            offset: self.offset.clone(),
            size: self.size.clone(),
            usage: self.usage.clone(),
            protection: self.protection.clone(),
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
                Some(dai) => Some(dai.to_output()),
            },
        }
    }
}

impl SauRegionsConfigType {
    pub fn to_output(&self) -> OutputSauRegionsConfigType {
        OutputSauRegionsConfigType {
            enabled: self.enabled.clone(),
            protection_when_disabled: self.protection_when_disabled.clone(),
            regions: self.regions.iter().map(SauRegionType::to_output).collect(),
        }
    }
}

impl BitRange {
    pub fn to_mask(&self) -> u32 {
        let (bit_offset, bit_width) = match &self {
            BitRange::OffsetWidthStyle {
                bit_offset,
                bit_width,
            } => (
                bit_offset.clone(),
                match bit_width {
                    None => 32 - bit_offset,
                    Some(bit_width) => bit_width.clone(),
                },
            ),
            BitRange::LsbMsbStyle { lsb, msb } => {
                if lsb < msb {
                    (lsb.clone(), msb - lsb)
                } else {
                    (msb.clone(), lsb - msb)
                }
            }
            BitRange::BitRangePattern { .. } => panic!("Not supported yet"),
        };
        ((((1 as usize) << bit_width as usize) - 1) << bit_offset) as u32
    }
}

impl EnumeratedValue {
    pub fn to_output(&self) -> OutputEnumeratedValue {
        OutputEnumeratedValue {
            name: self.name.clone(),
            description: self.description.as_ref().unwrap().clone(),
            value: match (&self.is_default, &self.value) {
                (None, Some(value)) => EnumValue::Value(value.clone()),
                (Some(false), Some(value)) => EnumValue::Value(value.clone()),
                (_, _) => EnumValue::Default,
            },
        }
    }
}

impl DimArrayIndex {
    fn to_output(&self) -> OutputDimArrayIndex {
        OutputDimArrayIndex {
            header_enum_name: self.header_enum_name.clone(),
            enumerated_values: self
                .enumerated_values
                .iter()
                .map(EnumeratedValue::to_output)
                .collect(),
        }
    }
}

impl SauRegionType {
    fn to_output(&self) -> OutputSauRegionType {
        OutputSauRegionType {
            enabled: self.enabled.clone(),
            name: self.name.clone(),
            base: self.base.clone(),
            limit: self.limit.clone(),
            access: self.access.clone(),
        }
    }
}
