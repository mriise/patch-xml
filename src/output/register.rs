use super::{AccessType, ModifiedWriteValues, WriteConstraint, ReadAction, Field};
use serde::{Serialize};
use crate::input::svd as input_svd;
use itertools::Itertools;

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    pub name: String,
    pub size: u32,
    pub reset_value: u32,
    //Supported by yaml-config
    pub display_name: String,
    pub description: String,
    pub access: Option<AccessType>,

    pub alternate_group: Option<String>,
    pub address_offset: u32,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub read_fields: Vec<Field>,
    pub write_fields: Vec<Field>,
    pub read_write_fields: Vec<Field>,
}

impl Register {
    pub fn from(patch_svd_register: &input_svd::Register) -> Register {
        let access = patch_svd_register.access.clone();
        Register {
            name: patch_svd_register.name.clone(),
            size: patch_svd_register.size.clone(),
            reset_value: patch_svd_register.reset_value.clone(),
            display_name: patch_svd_register.display_name.clone(),
            description: patch_svd_register.description.clone(),
            access: access.clone(),
            alternate_group: patch_svd_register.alternate_group.clone(),
            address_offset: patch_svd_register.address_offset.clone(),
            modified_write_values: patch_svd_register.modified_write_values.clone(),
            write_constraint: patch_svd_register.write_constraint.clone(),
            read_action: patch_svd_register.read_action.clone(),
            read_fields: patch_svd_register
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::ReadOnly
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(|f| Field::from(&f))
                .collect(),
            write_fields: patch_svd_register
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::WriteOnly
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(|f| Field::from(&f))
                .collect(),
            read_write_fields: patch_svd_register
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::ReadWrite
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(|f| Field::from(&f))
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

