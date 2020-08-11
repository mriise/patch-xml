use crate::output::{
    AccessType, ModifiedWriteValues, ReadAction, Register as OutputRegister, SvdConstant,
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
    //Not yet supported by yaml-config
    pub name: String,
    #[serde(with = "SvdConstant")]
    pub size: u32,
    #[serde(with = "SvdConstant")]
    pub reset_value: u32,
    //Supported by yaml-config
    pub display_name: String,
    pub description: String,
    pub access: Option<AccessType>,
    pub derived_from: Option<String>,

    pub alternate_group: Option<String>,
    #[serde(with = "SvdConstant")]
    pub address_offset: u32,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub fields: super::Fields,
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
        let access = self.access.clone();
        OutputRegister {
            name: self.name.clone(),
            size: self.size.clone(),
            reset_value: self.reset_value.clone(),
            display_name: self.display_name.clone(),
            description: self.description.clone(),
            access: access.clone(),
            alternate_group: self.alternate_group.clone(),
            address_offset: self.address_offset.clone(),
            modified_write_values: self.modified_write_values.clone(),
            write_constraint: self.write_constraint.clone(),
            read_action: self.read_action.clone(),
            read_fields: self
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::ReadOnly
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(super::Field::to_output)
                .collect(),
            write_fields: self
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::WriteOnly
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(super::Field::to_output)
                .collect(),
            read_write_fields: self
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::ReadWrite
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(super::Field::to_output)
                .collect(),
        }
    }
    fn prioritize_access_type(
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
    }
}
