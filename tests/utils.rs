#![allow(dead_code)]

use patch_svd::output::*;

pub const SPARSE_DEVICE: &str = r#"
    <device>
        <schemaVersion>1.0.0</schemaVersion>
        <name>ucName</name>
        <version>1.0.0</version>
        <description>Some description</description>
        <cpu>
            <name>CM0</name>
            <revision>r4</revision>
            <endian>little</endian>
            <mpuPresent>true</mpuPresent>
            <fpuPresent>true</fpuPresent>
            <nvicPrioBits>8</nvicPrioBits>
            <vendorSystickConfig>true</vendorSystickConfig>
        </cpu>
        <addressUnitBits>32</addressUnitBits>
        <width>32</width>
        <peripherals>
            <peripheral>
                <name>PeripheralName</name>
                <baseAddress>77</baseAddress>
            </peripheral>
        </peripherals>
    </device>
    "#;

pub const SPARSE_PERIPHERAL: &str = r#"
    <device>
        <schemaVersion>1.0.0</schemaVersion>
        <name>ucName</name>
        <version>1.0.0</version>
        <description>Some description</description>
        <cpu>
            <name>CM0</name>
            <revision>r4</revision>
            <endian>little</endian>
            <mpuPresent>true</mpuPresent>
            <fpuPresent>true</fpuPresent>
            <nvicPrioBits>8</nvicPrioBits>
            <vendorSystickConfig>true</vendorSystickConfig>
        </cpu>
        <addressUnitBits>32</addressUnitBits>
        <width>32</width>
        <peripherals>
            <peripheral>
                <name>PERIPHERAL_NAME_1</name>
                <baseAddress>1234</baseAddress>
            </peripheral>
        </peripherals>
    </device>
    "#;

pub fn get_sparse_device() -> Device {
    Device {
        schema_version: "1.0.0".to_string(),
        vendor: None,
        vendor_id: None,
        name: "ucName".to_string(),
        series: None,
        version: "1.0.0".to_string(),
        description: "Some description".to_string(),
        license_text: None,
        cpu: Cpu {
            name: CpuNameType::CM0,
            revision: "r4".to_string(),
            endian: EndianType::Little,
            mpu_present: true,
            fpu_present: true,
            fpu_d_p: None,
            dsp_present: None,
            icache_present: None,
            dcache_present: None,
            itcm_present: None,
            dtcm_present: None,
            vtor_present: None,
            nvic_prio_bits: SvdConstant { value: 8 },
            vendor_systick_config: true,
            device_num_interrupts: None,
            sau_num_regions: None,
            sau_regions_config: None,
        },
        header_system_filename: None,
        header_definitions_prefix: None,
        address_unit_bits: SvdConstant { value: 32 },
        width: SvdConstant { value: 32 },
        size: None,
        access: None,
        protection: None,
        reset_value: None,
        reset_mask: None,
        peripherals: Peripherals {
            peripheral: vec![Peripheral {
                derived_from: None,
                dim: None,
                dim_increment: None,
                dim_index: None,
                dim_name: None,
                dim_array_index: None,
                name: "PeripheralName".to_string(),
                version: None,
                description: None,
                alternate_peripheral: None,
                group_name: None,
                prepend_to_name: None,
                append_to_name: None,
                header_struct_name: None,
                disable_condition: None,
                base_address: SvdConstant { value: 77 },
                size: None,
                access: None,
                protection: None,
                reset_value: None,
                reset_mask: None,
                address_block: None,
                interrupt: None,
                registers: None,
            }],
        },
    }
}

pub enum SparsePeripheralContent {
    Interrupts(Vec<Interrupt>),
    Registers(Registers),
}

pub fn get_sparse_peripheral(sparse_peripheral_content: Option<SparsePeripheralContent>) -> Device {
    let (interrupt, registers) = match sparse_peripheral_content {
        None => (None, None),
        Some(SparsePeripheralContent::Interrupts(interrupts)) => (Some(interrupts), None),
        Some(SparsePeripheralContent::Registers(registers)) => (None, Some(registers)),
    };
    let mut device = get_sparse_device();
    device.peripherals = Peripherals {
        peripheral: vec![Peripheral {
            derived_from: None,
            dim: None,
            dim_increment: None,
            dim_index: None,
            dim_name: None,
            dim_array_index: None,
            name: "PERIPHERAL_NAME_1".to_string(),
            version: None,
            description: None,
            alternate_peripheral: None,
            group_name: None,
            prepend_to_name: None,
            append_to_name: None,
            header_struct_name: None,
            disable_condition: None,
            base_address: SvdConstant { value: 1234 },
            size: None,
            access: None,
            protection: None,
            reset_value: None,
            reset_mask: None,
            address_block: None,
            interrupt,
            registers,
        }],
    };
    device
}
