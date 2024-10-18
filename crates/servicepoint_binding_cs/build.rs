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
        if file == "lib"{
            continue;
        }

        let class = file.to_case(Case::UpperCamel) + "Native";
        csbindgen::Builder::default()
            .input_extern_file(path)
            .csharp_class_name(&class)
            .csharp_dll_name("servicepoint_binding_c")
            .csharp_namespace("ServicePoint.BindGen")
            .csharp_use_nint_types(true)
            .csharp_class_accessibility("public")
            .csharp_generate_const_filter(|_| true)
            .always_included_types(["SPByteSlice", "SPCompressionCode"])
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
            .generate_csharp_file(format!("ServicePoint/BindGen/{}.g.cs", &class))
            .unwrap();
    }

}
