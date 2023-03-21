use cc::Build;
use std::env;
use std::fs::read_dir;
use std::path::Path;

fn main() {
    println!("cargo:include=/usr/include");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-lib=dylib=gomp");
    println!("cargo:rustc-link-lib=dylib=blas");

    let ss_dir = clone_suitesparse();

    let mut builder = Build::new();
    builder
        .flag("-fopenmp")
        .flag("-static")
        .includes(suitesparse_includes(&ss_dir));

    let ssconfig = [format!("{ss_dir}/SuiteSparse_config/SuiteSparse_config.c")];
    builder.files(ssconfig).compile("SuiteSparse_config");

    let amd: Vec<String> = [
        "amd_2.c",
        "amd_defaults.c",
        "amd_aat.c",
        "amd_postorder.c",
        "amd_valid.c",
        "amd_post_tree.c",
    ]
    .iter()
    .map(|f| format!("{ss_dir}/AMD/Source/{f}"))
    .collect();
    builder.files(amd).compile("amd");

    let camd: Vec<String> = ["camd_2.c", "camd_postorder.c"]
    .iter()
    .map(|f| format!("{ss_dir}/CAMD/Source/{f}"))
    .collect();
    builder.files(camd).compile("camd");

    let colamd = [format!("{ss_dir}/COLAMD/Source/colamd.c")];
    builder.files(colamd).compile("colamd");

    let ccolamd = [format!("{ss_dir}/CCOLAMD/Source/ccolamd.c")];
    builder.files(ccolamd).compile("ccolamd");

    let cholmod: Vec<String> = [
        "Check/cholmod_check.c",
        "Core/cholmod_aat.c",
        "Cholesky/cholmod_amd.c",
        "Cholesky/cholmod_analyze.c",
        "Cholesky/cholmod_etree.c",
        "Cholesky/cholmod_postorder.c",
        "Cholesky/cholmod_rowcolcounts.c",
        "Cholesky/cholmod_colamd.c",
        "Core/cholmod_band.c",
        "Core/cholmod_change_factor.c",
        "Core/cholmod_common.c",
        "Core/cholmod_copy.c",
        "Core/cholmod_error.c",
        "Core/cholmod_factor.c",
        "Core/cholmod_memory.c",
        "Core/cholmod_sparse.c",
        "Core/cholmod_transpose.c",
        "Partition/cholmod_metis.c",
        "Partition/cholmod_camd.c",
        "Partition/cholmod_ccolamd.c",
        "Partition/cholmod_csymamd.c",
        "Partition/cholmod_metis_wrapper.c",
        "Partition/cholmod_nesdis.c",
        "Supernodal/cholmod_super_symbolic.c",
    ]
    .iter()
    .map(|f| format!("{ss_dir}/CHOLMOD/{f}"))
    .collect();
    builder.files(cholmod).compile("cholmod");

    let mut umfpack: Vec<String> = read_dir(format!("{ss_dir}/UMFPACK/Source"))
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .filter(|f| f.ends_with(".c"))
        .map(|f| format!("{ss_dir}/UMFPACK/Source/{f}"))
        .collect();
    let umfpack2: Vec<String> = read_dir(format!("{ss_dir}/UMFPACK/Source2"))
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .filter(|f| f.ends_with(".c"))
        .map(|f| format!("{ss_dir}/UMFPACK/Source2/{f}"))
        .collect();
    umfpack.extend(umfpack2);
    builder.files(umfpack).compile("umfpack");

    let examples = ["example1", "example2"];
    for example in examples.iter() {
        let path = format!("examples/{example}.c");
        println!("cargo:rerun-if-changed={path}");
        builder.files([path]).compile(example);
    }
}

fn clone_suitesparse() -> String {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Specify the path to the directory where the Git repository will be cloned
    let ss_dir = Path::new(&out_dir).join("SuiteSparse");

    // Clone the Git repository
    match git2::Repository::clone(
        "https://github.com/DrTimothyAldenDavis/SuiteSparse",
        &ss_dir,
    ) {
        Ok(_) => {}
        Err(_) => {}
    };

    return ss_dir.to_str().unwrap().to_owned();
}

fn suitesparse_includes(ss_dir: &str) -> Vec<String> {
    vec![
        format!("{}/SuiteSparse_config", ss_dir),
        format!("{}/AMD/Include", ss_dir),
        format!("{}/AMD/Include", ss_dir),
        format!("{}/AMD/Source", ss_dir),
        format!("{}/CAMD/Include", ss_dir),
        format!("{}/CAMD/Source", ss_dir),
        format!("{}/CCOLAMD/Include", ss_dir),
        format!("{}/CCOLAMD/Source", ss_dir),
        format!("{}/CHOLMOD", ss_dir),
        format!("{}/CHOLMOD/Cholesky", ss_dir),
        format!("{}/CHOLMOD/Config", ss_dir),
        format!("{}/CHOLMOD/Core", ss_dir),
        format!("{}/CHOLMOD/Include", ss_dir),
        format!("{}/CHOLMOD/SuiteSparse_metis/GKlib", ss_dir),
        format!("{}/CHOLMOD/SuiteSparse_metis/include", ss_dir),
        format!("{}/CHOLMOD/SuiteSparse_metis/libmetis", ss_dir),
        format!("{}/COLAMD/Include", ss_dir),
        format!("{}/COLAMD/Source", ss_dir),
        format!("{}/UMFPACK/Include", ss_dir),
        format!("{}/UMFPACK/Source", ss_dir),
    ]
}
