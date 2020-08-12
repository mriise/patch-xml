use crate::input::svd::{DimElementGroup, Fields, RegisterPropertiesGroup};
use crate::output::{
    DataType, ModifiedWriteValues, ReadAction, Register as OutputRegister, SvdConstant,
    WriteConstraint,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Registers {
    pub register: Vec<Register>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    pub derived_from: Option<String>,
    pub dim_element: DimElementGroup,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub alternate_group: Option<String>,
    pub alternate_register: Option<String>,
    #[serde(with = "SvdConstant")]
    pub address_offset: u32,
    pub register_properties: RegisterPropertiesGroup,
    pub data_type: Option<DataType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub fields: Option<Fields>,
}

impl Registers {
    pub fn to_output(&self) -> Vec<OutputRegister> {
        self.register
            .iter()
            .map(|r| r.to_output())
            .sorted_by(|r1, r2| r1.address_offset.cmp(&r2.address_offset))
            .collect()
    }
}

impl Register {
    pub fn to_output(&self) -> OutputRegister {
        OutputRegister {
            derived_from: self.derived_from.clone(),
            dim_element: self.dim_element.to_output(),
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            description: self.description.clone(),
            alternate_group: self.alternate_group.clone(),
            alternate_register: self.alternate_register.clone(),
            address_offset: self.address_offset,
            register_properties: self.register_properties.to_output(),
            data_type: self.data_type.clone(),
            modified_write_values: self.modified_write_values.clone(),
            write_constraint: self.write_constraint.clone(),
            read_action: self.read_action.clone(),
            fields: match &self.fields {
                None => Vec::new(),
                Some(fields) => fields.to_output(),
            },
        }
    }
    /*fn prioritize_access_type(
        register_access_type: &Option<AccessType>,
        field_access_type: &Option<AccessType>,
    ) -> AccessType {
        if field_access_type.is_some() {
            field_access_type.as_ref().unwrap().clone()
        } else if register_access_type.is_some() {
            register_access_type.as_ref().unwrap().clone()
        } else {
            AccessType::ReadWrite
        }
    }*/
}
