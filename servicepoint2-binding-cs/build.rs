fn main() {
    println!("cargo:rerun-if-changed=DOESNOTEXIST"); // rebuild every time
    csbindgen::Builder::default()
        .input_extern_file("../servicepoint2/src/bit_vec.rs")
        .input_extern_file("../servicepoint2/src/byte_grid.rs")
        .input_extern_file("../servicepoint2/src/command.rs")
        .input_extern_file("../servicepoint2/src/compression_code.rs")
        .input_extern_file("../servicepoint2/src/connection.rs")
        .input_extern_file("../servicepoint2/src/pixel_grid.rs")
        .input_extern_file("../servicepoint2/src/lib.rs")
        .input_extern_file("../servicepoint2/src/c_slice.rs")
        .input_extern_file("../servicepoint2/src/packet.rs")
        .csharp_dll_name("servicepoint2")
        .csharp_namespace("ServicePoint2.BindGen")
        .csharp_use_nint_types(true)
        .csharp_class_accessibility("public")
        .generate_csharp_file("src/BindGen/ServicePoint2.g.cs")
        .unwrap();
}
