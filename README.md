[![Crates.io](https://img.shields.io/crates/v/patch-svd.svg)](https://crates.io/crates/patch-svd)
[![Docs](https://docs.rs/patch-svd/badge.svg)](https://docs.rs/crate/patch-svd/)
[![Actions Status](https://github.com/VilNeo/patch-svd/workflows/Test/badge.svg)](https://github.com/VilNeo/patch-svd/actions)
[![grcov](https://img.shields.io/codecov/c/github/VilNeo/patch-svd)](https://app.codecov.io/gh/VilNeo/patch-svd)

***patch-svd* is a library that reads and patches SVD files from microcontroller manufacturers.**

The motivation of this library is to get rid of errors in SVD files of individual microcontrollers that are shipped by the corresponding manufacturers.

Technically, this library performs three steps:
1. Read [SVD-files](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_Format_pg.html)
2. patch the loaded SVD informations with a generic patch in YAML-format
3. Transfer the patched SVD informations into a dedicated structure that can be used in other crates

The syntax of the patch file format is documented in the crate [patch-xml](https://crates.io/crates/patch-xml).

# How to use patch-svd

Currently, patch-svd will require the unstable Rust toolchain because the ```external_doc```-feature is used.
The current state of this feature depends on [this pull request](https://github.com/rust-lang/rust/pull/83366).

```rust
let svd = r#"
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
let patch = r#"
    device:
        $modify:
            description: "Some other description"
    "#;
// Load SVD content, patch it and return it as Device structure
let result : patch_svd::output::Device = 
    patch_svd::get_patched_svd(svd.to_string(), patch.to_string()).unwrap();
```