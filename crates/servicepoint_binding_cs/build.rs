//! Build script generating the C# code needed to call methods from the `servicepoint` C library.

use std::{fs, path::Path};

use convert_case::{Case, Casing};

fn main() {
    println!("cargo::rerun-if-changed=../servicepoint_binding_c/src");
    println!("cargo::rerun-if-changed=build.rs");

    let mut paths = fs::read_dir("../servicepoint_binding_c/src")
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect::<Vec<_>>();
    paths.sort();

    for path in &paths {
        println!("cargo:rerun-if-changed={}", path.display());
        let file: &str = Path::new(path).file_stem().unwrap().to_str().unwrap();
        if file == "lib" {
            continue;
        }

        let class = file.to_case(Case::UpperCamel);
        csbindgen::Builder::default()
            .input_extern_file(path)
            .csharp_class_name(format!("{class}Native"))
            .csharp_dll_name("servicepoint_binding_c")
            .csharp_namespace("ServicePoint")
            .csharp_use_nint_types(true)
            .csharp_class_accessibility("public")
            .csharp_generate_const_filter(|_| true)
            .always_included_types(["SPByteSlice", "SPCompressionCode"])
            .csharp_group_methods("sp_bitmap_", "Bitmap", "SPBitmap")
            .csharp_group_methods("sp_bitvec_", "BitVec", "SPBitVec")
            .csharp_group_methods("sp_command_", "Command", "SPCommand")
            .csharp_group_methods(
                "sp_connection_",
                "Connection",
                "SPConnection",
            )
            .csharp_group_methods("sp_cp437_grid_", "Cp437Grid", "SPCp437Grid")
            .csharp_group_methods("sp_packet_", "Packet", "SPPacket")
            .csharp_group_methods(
                "sp_brightness_grid_",
                "BrightnessGrid",
                "SPBrightnessGrid",
            )
            .csharp_type_rename(move |name| {
                if name == "SPCompressionCode" {
                    "CompressionCode".to_string()
                } else {
                    name
                }
            })
            .generate_csharp_file(format!("ServicePoint/{class}.g.cs"))
            .unwrap();
    }
}
