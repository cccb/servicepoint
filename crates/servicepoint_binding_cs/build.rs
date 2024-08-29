//! Build script generating the C# code needed to call methods from the `servicepoint` C library.

fn main() {
    println!("cargo::rerun-if-changed=../servicepoint_binding_c/src");
    println!("cargo::rerun-if-changed=build.rs");
    csbindgen::Builder::default()
        .input_extern_file("../servicepoint_binding_c/src/bit_vec.rs")
        .input_extern_file("../servicepoint_binding_c/src/brightness_grid.rs")
        .input_extern_file("../servicepoint_binding_c/src/cp437_grid.rs")
        .input_extern_file("../servicepoint_binding_c/src/command.rs")
        .input_extern_file("../servicepoint_binding_c/src/connection.rs")
        .input_extern_file("../servicepoint_binding_c/src/pixel_grid.rs")
        .input_extern_file("../servicepoint_binding_c/src/lib.rs")
        .input_extern_file("../servicepoint_binding_c/src/c_slice.rs")
        .input_extern_file("../servicepoint_binding_c/src/packet.rs")
        .input_extern_file("../servicepoint/src/command.rs")
        .input_extern_file("../servicepoint/src/connection.rs")
        .input_extern_file("../servicepoint/src/pixel_grid.rs")
        .input_extern_file("../servicepoint/src/lib.rs")
        .input_extern_file("../servicepoint/src/packet.rs")
        .input_extern_file("../servicepoint/src/compression_code.rs")
        .csharp_dll_name("servicepoint_binding_c")
        .csharp_namespace("ServicePoint.BindGen")
        .csharp_use_nint_types(true)
        .csharp_class_accessibility("public")
        .csharp_generate_const_filter(|_| true)
        .csharp_type_rename(move |name| {
            if name.len() > 1
                && name.starts_with("C")
                && name.chars().nth(1).unwrap().is_uppercase()
            {
                name[1..].to_string()
            } else {
                name
            }
        })
        .generate_csharp_file("ServicePoint/BindGen/ServicePoint.g.cs")
        .unwrap();
}
