use std::fs;

#[cfg(not(feature = "c"))]
fn main() {}

#[cfg(feature = "c")]
fn main() {
    let mut builder = cc::Build::new();

    let includes = [
        "SuiteSparse/AMD/Include",
        "SuiteSparse/AMD/Source",
        "SuiteSparse/CAMD/Include",
        "SuiteSparse/CAMD/Source",
        "SuiteSparse/CCOLAMD/Include",
        "SuiteSparse/CCOLAMD/Source",
        "SuiteSparse/CHOLMOD",
        "SuiteSparse/CHOLMOD/Cholesky",
        "SuiteSparse/CHOLMOD/Config",
        "SuiteSparse/CHOLMOD/Core",
        "SuiteSparse/CHOLMOD/Include",
        "SuiteSparse/CHOLMOD/SuiteSparse_metis/GKlib",
        "SuiteSparse/CHOLMOD/SuiteSparse_metis/include",
        "SuiteSparse/CHOLMOD/SuiteSparse_metis/libmetis",
        "SuiteSparse/COLAMD/Include",
        "SuiteSparse/COLAMD/Source",
        "SuiteSparse/UMFPACK/Include",
        "SuiteSparse/UMFPACK/Source",
    ];

    let build_cache: Vec<String> = fs::read_dir("build-cache")
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .filter(|f| f.ends_with(".a"))
        .collect();
    println!("cargo:rustc-link-search=build-cache");
    for filename in build_cache.iter(){
        println!("cargo:rustc-link-lib=static={}", name_from_filename(filename));
    }

    let path = format!("examples/example.c");
    println!("cargo:rerun-if-changed=examples/example.c");
    builder.file(path).includes(includes).compile("example");

}

fn name_from_filename(filename: &str) -> &str {
    let mut parts = filename.split(".");
    let libname = parts.next().unwrap();
    let name = &libname[3..libname.len()];
    return name;
}
