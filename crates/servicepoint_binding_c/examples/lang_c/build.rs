const SP_INCLUDE: &str = "DEP_SERVICEPOINT_INCLUDE";

fn main() {
    println!("cargo::rerun-if-changed=src/main.c");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed={SP_INCLUDE}");

    let sp_include =
        std::env::var_os(SP_INCLUDE).unwrap().into_string().unwrap();

    // this builds a lib, this is only to check that the example compiles
    let mut cc = cc::Build::new();
    cc.file("src/main.c");
    cc.include(&sp_include);
    cc.opt_level(2);
    cc.compile("lang_c");
}
