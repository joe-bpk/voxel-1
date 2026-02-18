/// # category
/// **server side**
///
/// build script to compile the c mesh generation backend.
fn main() {
    // re-run if the c source changes
    println!("cargo:rerun-if-changed=src/display/mesh/mesh_gen.c");

    // compile the c logic for performance-critical mesh generation
    cc::Build::new()
        .file("src/display/mesh/mesh_gen.c")
        .include("/usr/local/include")
        .compile("meshgen");

    // link the system raylib library
    println!("cargo:rustc-link-lib=raylib");
}
