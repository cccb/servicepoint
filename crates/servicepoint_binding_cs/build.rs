//! Build script generating the C# code needed to call methods from the `servicepoint` C library.

use std::fs;

fn main() {
    println!("cargo::rerun-if-changed=../servicepoint_binding_c/src");
    println!("cargo::rerun-if-changed=build.rs");

    let mut builder = csbindgen::Builder::default();

    let mut paths = fs::read_dir("../servicepoint_binding_c/src").unwrap()
        .map(|x| x.unwrap().path())
        .collect::<Vec<_>>();
    paths.sort();

    for path in paths {
        println!("cargo:rerun-if-changed={}", path.display());
        builder = builder.input_extern_file(path);
    }

    builder
        .csharp_dll_name("servicepoint_binding_c")
        .csharp_namespace("ServicePoint.BindGen")
        .csharp_use_nint_types(true)
        .csharp_class_accessibility("public")
        .csharp_generate_const_filter(|_| true)
        .csharp_type_rename(move |name| {
            if name.len() > 2
                && name.starts_with("SP")
                && name.chars().nth(2).unwrap().is_uppercase()
            {
                name[2..].to_string()
            } else {
                name
            }
        })
        .generate_csharp_file("ServicePoint/BindGen/ServicePoint.g.cs")
        .unwrap();
}
