/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
*/
use std::env;

fn main() {
    // check pkgconfig dependencies
    // system_deps::Config::new().probe().unwrap();

    // invalidate the built crate whenever the wrapper changes
    println!("cargo:rustc-link-search=/usr/local/lib64");
    println!("cargo:rustc-link-arg=-liso15118");
    // println!("cargo:rustc-link-arg=-lgnutls");

    if let Ok(value) = env::var("CARGO_TARGET_DIR") {
        if let Ok(profile) = env::var("PROFILE") {
            println!("cargo:rustc-link-search=crate={}{}", value, profile);
        }
    }
    let header = "
    // -----------------------------------------------------------------------
    //         <- private 'lib-iso15118' Rust/C unsafe binding ->
    // -----------------------------------------------------------------------
    //   Do not exit this file it will be regenerated automatically by cargo.
    //   Check:
    //     - build.rs for C/Rust glue options
    //     - src/capi/capi-exi.h for C prototype inputs
    // -----------------------------------------------------------------------
    ";
    println!("cargo:rerun-if-changed=capi/capi-exi.h");
    let libcapi = bindgen::Builder::default()
        .header("capi/capi-exi.h") // Chargebyte C prototype wrapper input
        .raw_line(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_debug(false)
        .layout_tests(false)
        .allowlist_item("exi_.*")
        .allowlist_var("EXI_.*")
        .allowlist_type(".*_exiDocument")
        .allowlist_function("decode_exi_.*")
        .allowlist_function("encode_.exi_*")
        .allowlist_function("init_.*")
        .generate()
        .expect("Unable to generate _exi-capi.rs");

    libcapi
        .write_to_file("capi/_exi-capi.rs")
        .expect("Couldn't write _exi-capi.rs!");


        let header = "
    // -----------------------------------------------------------------------
    //         <- private 'lib-iso15118' Rust/C unsafe binding ->
    // -----------------------------------------------------------------------
    //   Do not exit this file it will be regenerated automatically by cargo.
    //   Check:
    //     - build.rs for C/Rust glue options
    //     - src/capi/capi-v2g.h for C prototype inputs
    // -----------------------------------------------------------------------
    ";
    println!("cargo:rerun-if-changed=capi/capi-v2g.h");
    let libcapi = bindgen::Builder::default()
        .header("capi/capi-v2g.h") // Chargebyte C prototype wrapper input
        .raw_line(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_debug(false)
        .layout_tests(false)
        .allowlist_item("sdp_.*")
        .allowlist_item("v2g_.*")
        .allowlist_item("v2g0_.*")
        .allowlist_var("SDP_.*")
        .allowlist_item("V2GTP.*")
        .allowlist_function("init_.*")
        .allowlist_function("decode_appHand_.*")
        .allowlist_function("encode_appHand_.*")
        .blocklist_item("exi.*")

        .generate()
        .expect("Unable to generate _v2g-capi.rs");

    libcapi
        .write_to_file("capi/_v2g-capi.rs")
        .expect("Couldn't write _v2g-capi.rs!");


        let header = "
    // -----------------------------------------------------------------------
    //         <- private 'lib-iso15118' Rust/C unsafe binding ->
    // -----------------------------------------------------------------------
    //   Do not exit this file it will be regenerated automatically by cargo.
    //   Check:
    //     - build.rs for C/Rust glue options
    //     - src/capi/capi-iso2.h for C prototype inputs
    // -----------------------------------------------------------------------
    ";
    println!("cargo:rerun-if-changed=capi/capi-iso2.h");
    let libcapi = bindgen::Builder::default()
        .header("capi/capi-iso2.h") // Chargebyte C prototype wrapper input
        .raw_line(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_debug(false)
        .layout_tests(false)
        .allowlist_item("iso2_.*")
        .allowlist_function("decode_iso2_.*")
        .allowlist_function("encode_iso2_.*")
        .blocklist_item("exi.*")
        .generate()
        .expect("Unable to generate _iso2-capi.rs");

    libcapi
        .write_to_file("capi/_iso2-capi.rs")
        .expect("Couldn't write _iso2-capi.rs!");
}
