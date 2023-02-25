use std::env;
use std::fs;
use std::path::PathBuf;

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

    let path = format!("examples/example.c");
    println!("cargo:rerun-if-changed=examples/example.c");
    builder.file(path).includes(includes).compile("example.c"); // we don't want to cache our own code

    let path = format!("SuiteSparse/SuiteSparse_config/SuiteSparse_config.c");
    println!("cargo:rerun-if-changed=SuiteSparse/SuiteSparse_config/SuiteSparse_config.c");
    cached_compilation(&mut builder, &path, &includes, "SuiteSparse_config.c");

    let amd = [
        "amd_2.c",
        "amd_defaults.c",
        "amd_aat.c",
        "amd_postorder.c",
        "amd_valid.c",
        "amd_post_tree.c",
    ];
    for filename in amd {
        let path = format!("SuiteSparse/AMD/Source/{filename}");
        println!("cargo:rerun-if-changed={path}");
        cached_compilation(&mut builder, &path, &includes, &filename);
    }

    let camd = ["camd_2.c", "camd_postorder.c"];
    for filename in camd {
        let path = format!("SuiteSparse/CAMD/Source/{filename}");
        println!("cargo:rerun-if-changed={path}");
        cached_compilation(&mut builder, &path, &includes, &filename);
    }

    let colamd = ["colamd.c"];
    for filename in colamd {
        let path = format!("SuiteSparse/COLAMD/Source/{filename}");
        println!("cargo:rerun-if-changed={path}");
        cached_compilation(&mut builder, &path, &includes, &filename);
    }

    let ccolamd = ["ccolamd.c"];
    for filename in ccolamd {
        let path = format!("SuiteSparse/CCOLAMD/Source/{filename}");
        println!("cargo:rerun-if-changed={path}");
        cached_compilation(&mut builder, &path, &includes, &filename);
    }

    let cholmod = [
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
    ];
    for filename in cholmod {
        let path = format!("SuiteSparse/CHOLMOD/{filename}");
        let mut filename_parts = filename.split('/');
        filename_parts.next();
        let filename = filename_parts.next().unwrap();
        println!("cargo:rerun-if-changed={path}");
        cached_compilation(&mut builder, &path, &includes, &filename);
    }

    let umfpack = [
        "umf_analyze.c",
        "umf_apply_order.c",
        "umf_cholmod.c",
        "umf_colamd.c",
        "umf_free.c",
        "umf_fsize.c",
        "umf_is_permutation.c",
        "umf_malloc.c",
        "umf_set_stats.c",
        "umf_singletons.c",
        "umf_symbolic_usage.c",
        "umf_transpose.c",
        "umfpack_free_symbolic.c",
        "umfpack_numeric.c",
        "umfpack_qsymbolic.c",
        "umfpack_symbolic.c",
        "umfpack_tictoc.c",
        "umfpack_timer.c",
    ];
    for filename in umfpack {
        let path = format!("SuiteSparse/UMFPACK/Source/{filename}");
        println!("cargo:rerun-if-changed={path}");
        cached_compilation(&mut builder, &path, &includes, &filename);
    }
    // let umfpack: Vec<String> = fs::read_dir("SuiteSparse/UMFPACK/Source")
    //     .unwrap()
    //     .map(|f| f.unwrap().file_name().into_string().unwrap())
    //     .filter(|f| f.ends_with(".c"))
    //     .collect();
}

fn stem(filename: &str) -> &str {
    let mut parts = filename.split(".");
    return parts.next().unwrap();
}

fn cached_compilation(builder: &mut cc::Build, path: &str, includes: &[&str], filename: &str) {
    let binary = stem(&filename);

    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // let out_binary = format!("{}/{}.o", out_dir.to_str().unwrap(), stem(&path));
    // let out_binary_path = PathBuf::from(&out_binary);
    // let out_folder = out_binary_path.parent().unwrap();
    // let out_library = format!("{}/lib{}.a", out_dir.to_str().unwrap(), binary);

    // if !out_folder.exists() {
    //     fs::create_dir_all(&out_folder).unwrap();
    // }

    // let cache_dir = out_dir.parent().unwrap().parent().unwrap();
    // let cached_binary = format!(
    //     "{}/umfpack/out/{}.o",
    //     cache_dir.to_str().unwrap(),
    //     stem(&path)
    // );
    // let cached_binary_path = PathBuf::from(&cached_binary);
    // let cached_folder = cached_binary_path.parent().unwrap();
    // let cached_library = format!(
    //     "{}/umfpack/out/lib{}.a",
    //     cache_dir.to_str().unwrap(),
    //     binary
    // );
    // let cached_library_path = PathBuf::from(&cached_library);

    // if !cached_folder.exists() {
    //     fs::create_dir_all(&cached_folder).unwrap();
    // }

    // if cached_binary_path.exists() & cached_library_path.exists() {
    //     return;
    // }

    // let mut includes2: Vec<&str> = includes.iter().map(|x| *x).collect();
    // includes2.extend([out_dir.to_str().unwrap(), out_folder.to_str().unwrap()]);

    builder.file(path).includes(includes).compile(binary);
    // std::fs::copy(&out_binary, &cached_binary).unwrap();
    // std::fs::copy(&out_library, &cached_library).unwrap();
}
