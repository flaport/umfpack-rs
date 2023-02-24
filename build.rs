use std::fs;

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
    builder
        .file(path)
        .includes(includes)
        .compile(stem("example"));

    let path = format!("SuiteSparse/SuiteSparse_config/SuiteSparse_config.c");
    println!("cargo:rerun-if-changed=SuiteSparse/SuiteSparse_config/SuiteSparse_config.c");
    builder
        .file(path)
        .includes(includes)
        .compile(stem("SuiteSparse_config"));

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
        builder
            .file(path)
            .includes(includes)
            .compile(stem(&filename));
    }

    let camd = [
        "camd_2.c",
        "camd_postorder.c",
    ];
    for filename in camd {
        let path = format!("SuiteSparse/CAMD/Source/{filename}");
        println!("cargo:rerun-if-changed={path}");
        builder
            .file(path)
            .includes(includes)
            .compile(stem(&filename));
    }

    let colamd = [
        "colamd.c",
    ];
    for filename in colamd {
        let path = format!("SuiteSparse/COLAMD/Source/{filename}");
        println!("cargo:rerun-if-changed={path}");
        builder
            .file(path)
            .includes(includes)
            .compile(stem(&filename));
    }

    let ccolamd = [
        "ccolamd.c",
    ];
    for filename in ccolamd {
        let path = format!("SuiteSparse/CCOLAMD/Source/{filename}");
        println!("cargo:rerun-if-changed={path}");
        builder
            .file(path)
            .includes(includes)
            .compile(stem(&filename));
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
        builder
            .file(path)
            .includes(includes)
            .compile(stem(&filename));
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
        builder
            .file(path)
            .includes(includes)
            .compile(stem(&filename));
    }

    // let umfpack: Vec<String> = fs::read_dir("SuiteSparse/UMFPACK/Source")
    //     .unwrap()
    //     .map(|f| f.unwrap().file_name().into_string().unwrap())
    //     .filter(|f| f.ends_with(".c"))
    //     .collect();
}

#[cfg(not(feature = "c"))]
fn main() {}

fn stem(filename: &str) -> &str {
    let mut parts = filename.split(".");
    return parts.next().unwrap();
}
