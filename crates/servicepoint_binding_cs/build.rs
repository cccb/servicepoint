fn main() {
    println!("cargo:rerun-if-changed=DOESNOTEXIST"); // rebuild every time
    csbindgen::Builder::default()
        .input_extern_file("../servicepoint/src/bit_vec.rs")
        .input_extern_file("../servicepoint/src/byte_grid.rs")
        .input_extern_file("../servicepoint/src/command.rs")
        .input_extern_file("../servicepoint/src/compression_code.rs")
        .input_extern_file("../servicepoint/src/connection.rs")
        .input_extern_file("../servicepoint/src/pixel_grid.rs")
        .input_extern_file("../servicepoint/src/lib.rs")
        .input_extern_file("../servicepoint/src/c_slice.rs")
        .input_extern_file("../servicepoint/src/packet.rs")
        .csharp_dll_name("servicepoint")
        .csharp_namespace("ServicePoint.BindGen")
        .csharp_use_nint_types(true)
        .csharp_class_accessibility("public")
        .generate_csharp_file("src/BindGen/ServicePoint.g.cs")
        .unwrap();
}
