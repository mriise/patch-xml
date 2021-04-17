# patch-svd

[![Crates.io](https://img.shields.io/crates/v/patch-svd.svg)](https://crates.io/crates/patch-svd)
[![Docs](https://docs.rs/patch-svd/badge.svg)](https://docs.rs/crate/patch-svd/)
[![Actions Status](https://github.com/VilNeo/patch-svd/workflows/Test/badge.svg)](https://github.com/VilNeo/patch-svd/actions)
[![grcov](https://img.shields.io/codecov/c/github/VilNeo/patch-svd)](https://github.com/VilNeo/patch-svd/actions)

***patch-svd* is a library that reads and patches SVD files from microcontroller manufacturers.**

The motivation of this library is to get rid of errors in SVD files of individual microcontrollers that are shipped by the corresponding manufacturers.

Technically, this library performs three steps:
1. Read [SVD-files](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_Format_pg.html)
2. patch the loaded SVD informations with a generic patch in YAML-format
3. Transfer the patched SVD informations into a dedicated structure that can be used in other crates

The syntax of the patch file format is documented in the crate [patch-xml](https://crates.io/crates/patch-xml).