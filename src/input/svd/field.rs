use crate::input::svd::{BitRange, DimElementGroup, EnumeratedValue};
use crate::output::{
    AccessType, EnumAccessType, EnumeratedValues as OutputEnumeratedValues, EnumeratedValuesUsage,
    Field as OutputField, ModifiedWriteValues, ReadAction, WriteConstraint,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub field: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValues {
    pub derived_from: Option<String>,
    pub name: Option<String>,
    pub header_enum_name: Option<String>,
    pub usage: Option<EnumeratedValuesUsage>,
    pub enumerated_value: Vec<EnumeratedValue>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub derived_from: Option<String>,
    pub dim_element: DimElementGroup,
    pub name: String,
    pub description: Option<String>,
    pub bit_range: BitRange,
    pub access: Option<AccessType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub enumerated_values: Vec<EnumeratedValues>,
}

impl EnumeratedValues {
    pub fn new() -> EnumeratedValues {
        EnumeratedValues {
            derived_from: None,
            name: None,
            header_enum_name: None,
            usage: None,
            enumerated_value: Vec::new(),
        }
    }
}

impl Fields {
    pub fn to_output(&self) -> Vec<OutputField> {
        self.field.iter().map(Field::to_output).collect()
    }
}

impl Field {
    pub fn to_output(&self) -> OutputField {
        OutputField {
            derived_from: None,
            dim_element: self.dim_element.to_output(),
            name: self.name.clone(),
            description: self.description.clone(),
            mask: self.bit_range.to_mask(),
            access: self.access.clone(),
            modified_write_values: self.modified_write_values.clone(),
            write_constraint: self.write_constraint.clone(),
            read_action: self.read_action.clone(),
            enumerated_values: self.enumerated_values_to_field_type(),
        }
    }
    fn enumerated_values_to_field_type(&self) -> EnumAccessType {
        let mut read_write = None;
        let mut read = None;
        let mut write = None;
        if self.enumerated_values.len() >= 1 {
            let ev = self.enumerated_values.get(0).unwrap().to_output();
            match &ev.usage {
                None => read_write = Some(ev),
                Some(usage) => match &usage {
                    EnumeratedValuesUsage::Read => read = Some(ev),
                    EnumeratedValuesUsage::Write => write = Some(ev),
                    EnumeratedValuesUsage::ReadWrite => read_write = Some(ev),
                },
            }
        }
        if self.enumerated_values.len() == 2 {
            let ev = self.enumerated_values.get(1).unwrap().to_output();
            match &ev.usage {
                None => read_write = Some(ev),
                Some(usage) => match &usage {
                    EnumeratedValuesUsage::Read => {
                        if read.is_some() {
                            panic!("Defined two 'read' usages for enumerated values. Only one is allowed.")
                        } else {
                            read = Some(ev)
                        }
                    }
                    EnumeratedValuesUsage::Write => {
                        if write.is_some() {
                            panic!("Defined two 'write' usages for enumerated values. Only one is allowed.")
                        } else {
                            write = Some(ev)
                        }
                    }
                    EnumeratedValuesUsage::ReadWrite => {
                        if read_write.is_some() {
                            panic!("Defined two 'read_write' usages for enumerated values. Only one is allowed.")
                        } else {
                            read_write = Some(ev)
                        }
                    }
                },
            }
        }
        match (read_write, read, write) {
            (None, None, None) => EnumAccessType::None,
            (Some(rw), None, None) => EnumAccessType::ReadAndWrite(rw),
            (None, Some(read), None) => EnumAccessType::Read(read),
            (None, None, Some(write)) => EnumAccessType::Write(write),
            (None, Some(read), Some(write)) => EnumAccessType::ReadWrite { read, write },
            _ => panic!(
                "Unsupported usage configuration. Either read-write or read/write ist allowed."
            ),
        }
    }
}

impl EnumeratedValues {
    fn to_output(&self) -> OutputEnumeratedValues {
        let enumerated_value = self
            .enumerated_value
            .iter()
            .map(EnumeratedValue::to_output)
            .collect();
        OutputEnumeratedValues {
            derived_from: self.derived_from.clone(),
            name: self.name.clone(),
            header_enum_name: self.header_enum_name.clone(),
            usage: self.usage.clone(),
            enumerated_value,
        }
    }
}
