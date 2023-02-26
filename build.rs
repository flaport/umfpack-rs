use cc::Build;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

#[cfg(not(feature = "c"))]
fn main() {}

#[cfg(feature = "c")]
fn main() {
    let mut builder = Build::new();

    build_suitesparse(&mut builder);

    let path = format!("examples/example.c");
    println!("cargo:rerun-if-changed=examples/example.c");
    builder
        .file(path)
        .includes(suitesparse_includes())
        .compile("example");
}

fn suitesparse_includes<'a>() -> [&'a str; 18] {
    [
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
    ]
}

fn build_suitesparse(builder: &mut Build) {
    let mut file = fs::File::create("build.log").unwrap();
    file.write_all(b"Start Build\n").unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("build.log")
        .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cache_dir = out_dir.parent().unwrap().parent().unwrap();
    let cache_dir = format!("{}/umfpack/out", cache_dir.to_str().unwrap());
    let includes = suitesparse_includes();

    let build_cache: &Vec<String> = &match fs::read_dir(&cache_dir) {
        Ok(v) => {
            println!("cargo:rustc-link-search={cache_dir}");
            v.map(|f| f.unwrap().file_name().into_string().unwrap())
                .filter(|f| f.ends_with(".a"))
                .collect()
        }
        Err(_) => Vec::new(),
    };
    for filename in build_cache.iter() {
        writeln!(file, "{filename}").unwrap();
        println!(
            "cargo:rustc-link-lib=static={}",
            name_from_filename(filename)
        );
    }

    let path = format!("SuiteSparse/SuiteSparse_config/SuiteSparse_config.c");
    cached_compilation(
        builder,
        &path,
        &includes,
        "SuiteSparse_config.c",
        build_cache,
    );

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
        cached_compilation(builder, &path, &includes, &filename, build_cache);
    }

    let camd = ["camd_2.c", "camd_postorder.c"];
    for filename in camd {
        let path = format!("SuiteSparse/CAMD/Source/{filename}");
        cached_compilation(builder, &path, &includes, &filename, build_cache);
    }

    let colamd = ["colamd.c"];
    for filename in colamd {
        let path = format!("SuiteSparse/COLAMD/Source/{filename}");
        cached_compilation(builder, &path, &includes, &filename, build_cache);
    }

    let ccolamd = ["ccolamd.c"];
    for filename in ccolamd {
        let path = format!("SuiteSparse/CCOLAMD/Source/{filename}");
        cached_compilation(builder, &path, &includes, &filename, build_cache);
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
        cached_compilation(builder, &path, &includes, &filename, build_cache);
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
        "umf_kernel.c",
        "umf_realloc.c",
        "umf_valid_symbolic.c",
        "umfpack_free_numeric.c",
        "umf_kernel_init.c",
        "umf_local_search.c",
        "umf_create_element.c",
        "umf_kernel_wrapup.c",
        "umf_build_tuples.c",
        "umf_tuple_lengths.c",
        "umf_mem_free_tail_block.c",
        "umf_mem_alloc_tail_block.c",
        "umf_mem_alloc_element.c",
        "umf_mem_alloc_head_block.c",
        "umf_mem_init_memoryspace.c",
        "umf_scale.c",
        "umf_get_memory.c",
        "umf_garbage_collection.c",
        "umf_row_search.c",
        "umf_store_lu.c",
        "umf_blas3_update.c",
        "umf_extend_front.c",
        "umf_init_front.c",
        "umf_assemble.c",
        "umf_scale_column.c",
        "umf_grow_front.c",
        "umf_start_front.c",
    ];
    let umfpack: &Vec<String> = &fs::read_dir(&cache_dir)
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .filter(|f| f.ends_with(".a"))
        .collect();
    for filename in umfpack {
        let path = format!("SuiteSparse/UMFPACK/Source/{filename}");
        cached_compilation(builder, &path, &includes, &filename, build_cache);
    }
}

fn stem(filename: &str) -> &str {
    let mut parts = filename.split(".");
    return parts.next().unwrap();
}

fn name_from_filename(filename: &str) -> &str {
    let mut parts = filename.split(".");
    let libname = parts.next().unwrap();
    let name = &libname[3..libname.len()];
    return name;
}

fn cached_compilation(
    builder: &mut Build,
    path: &str,
    includes: &[&str],
    filename: &str,
    build_cache: &Vec<String>,
) {
    //let mut file = OpenOptions::new()
    //    .write(true)
    //    .append(true)
    //    .open("build.log")
    //    .unwrap();

    let binary = stem(&filename);
    let lib_name = format!("lib{binary}.a");

    //writeln!(file, "{filename}").unwrap();

    if build_cache.iter().any(|n| n == &lib_name) {
        return; // already compiled, no need to do it again.
    }
    //writeln!(file, "compiling...").unwrap();

    println!("cargo:rerun-if-changed={path}");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_binary = format!("{}/{}.o", out_dir.to_str().unwrap(), stem(&path));
    let out_binary_path = PathBuf::from(&out_binary);
    let out_folder = out_binary_path.parent().unwrap();
    let out_library = format!("{}/lib{}.a", out_dir.to_str().unwrap(), binary);

    if !out_folder.exists() {
        fs::create_dir_all(&out_folder).unwrap();
    }

    let cache_dir = out_dir.parent().unwrap().parent().unwrap();
    let cached_binary = format!(
        "{}/umfpack/out/{}.o",
        cache_dir.to_str().unwrap(),
        stem(&path)
    );
    let cached_binary_path = PathBuf::from(&cached_binary);
    let cached_folder = cached_binary_path.parent().unwrap();
    let cached_library = format!(
        "{}/umfpack/out/lib{}.a",
        cache_dir.to_str().unwrap(),
        binary
    );
    let cached_library_path = PathBuf::from(&cached_library);

    if !cached_folder.exists() {
        fs::create_dir_all(&cached_folder).unwrap();
    }

    if cached_binary_path.exists() & cached_library_path.exists() {
        return;
    }

    let mut includes2: Vec<&str> = includes.iter().map(|x| *x).collect();
    includes2.extend([out_dir.to_str().unwrap(), out_folder.to_str().unwrap()]);

    builder.file(path).includes(includes).compile(binary);
    std::fs::copy(&out_binary, &cached_binary).unwrap();
    std::fs::copy(&out_library, &cached_library).unwrap();
}
