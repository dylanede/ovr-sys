fn main() {
    use std::path::PathBuf;
    use std::env;

    let triple = env::var("TARGET").unwrap();
    let triple = triple.split('-').collect::<Vec<_>>();
    let arch = &*triple[0];
    let sys = &*triple[2];
    if sys == "win32" || sys == "windows" {
        let mut path = PathBuf::new();
        path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
        path.push("lib");
        let lib_name;
        path.push("windows");
        if arch == "i686" {
            path.push("x86");
        } else if arch == "x86_64" {
            path.push("x86_64");
        }
        lib_name = "LibOVR";
        println!("cargo:rustc-link-search=native={}", path.display());
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }
}