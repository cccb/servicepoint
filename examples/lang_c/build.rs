fn main() {
    println!("cargo::rerun-if-changed=src/main.c");
    println!("cargo::rerun-if-changed=build.rs");

    let sp_include = std::env::var_os("DEP_SERVICEPOINT_INCLUDE")
        .unwrap()
        .into_string()
        .unwrap();

    // this builds a lib, this is only to check that the example compiles
    let mut cc = cc::Build::new();
    cc.file("src/main.c");
    cc.include(&sp_include);
    cc.compile("lang_c");
}
