use cc::Build;
use cfg_if::cfg_if;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut file = fs::File::create("build.log").unwrap();
    file.write_all(b"Start Build\n").unwrap();

    let mut log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("build.log")
        .unwrap();

    let mut builder = Build::new();
    builder
        .flag("-fopenmp")
        .flag("-static")
        .includes(suitesparse_includes());

    println!("cargo:rustc-link-search=/lib");
    println!("cargo:rustc-link-search=/usr/lib");

    // why do we need this?
    println!("cargo:rustc-link-lib=dylib=gomp");

    let blas = get_blas_feature(&mut log_file);
    writeln!(log_file, "blas: {blas}").unwrap();

    build_blas(&mut builder, &blas);

    let cache_dir = get_build_cache_dir(&blas);

    #[cfg(feature = "s3-sync")]
    {
        sync_s3_cache(&cache_dir, &mut log_file, &blas);
    }

    let build_cache = get_build_cache(&cache_dir);
    build_suitesparse(&mut builder, &build_cache, &mut log_file, &blas);

    let examples = ["example1", "example2"];
    for example in examples.iter() {
        let path = format!("examples/{example}.c");
        println!("cargo:rerun-if-changed=examples/{example}.c");
        builder.file(path).compile(example);
    }
}

fn get_build_cache_dir(blas: &str) -> String {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cache_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("umfpack")
        .join("out")
        .join(blas);
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).unwrap();
    }
    let cache_dir = cache_dir.to_str().unwrap();
    return cache_dir.to_owned();
}

fn get_build_cache(cache_dir: &str) -> Vec<String> {
    let build_cache: Vec<String> = match fs::read_dir(&cache_dir) {
        Ok(v) => {
            println!("cargo:rustc-link-search={cache_dir}");
            v.map(|f| f.unwrap().file_name().into_string().unwrap())
                .filter(|f| f.ends_with(".a"))
                .collect()
        }
        Err(_) => Vec::new(),
    };
    for filename in build_cache.iter() {
        println!(
            "cargo:rustc-link-lib=static={}",
            name_from_filename(filename)
        );
    }
    return build_cache;
}

#[allow(dead_code)]
fn build_suitesparse(
    builder: &mut Build,
    build_cache: &Vec<String>,
    log_file: &mut File,
    blas: &str,
) {
    let path = format!("SuiteSparse/SuiteSparse_config/SuiteSparse_config.c");
    cached_compilation(
        builder,
        &path,
        "SuiteSparse_config.c",
        build_cache,
        log_file,
        blas,
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
        cached_compilation(builder, &path, &filename, build_cache, log_file, blas);
    }

    let camd = ["camd_2.c", "camd_postorder.c"];
    for filename in camd {
        let path = format!("SuiteSparse/CAMD/Source/{filename}");
        cached_compilation(builder, &path, &filename, build_cache, log_file, blas);
    }

    let colamd = ["colamd.c"];
    for filename in colamd {
        let path = format!("SuiteSparse/COLAMD/Source/{filename}");
        cached_compilation(builder, &path, &filename, build_cache, log_file, blas);
    }

    let ccolamd = ["ccolamd.c"];
    for filename in ccolamd {
        let path = format!("SuiteSparse/CCOLAMD/Source/{filename}");
        cached_compilation(builder, &path, &filename, build_cache, log_file, blas);
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
        cached_compilation(builder, &path, &filename, build_cache, log_file, blas);
    }

    let umfpack: &Vec<String> = &fs::read_dir("SuiteSparse/UMFPACK/Source")
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .filter(|f| f.ends_with(".c"))
        .collect();
    for filename in umfpack {
        let path = format!("SuiteSparse/UMFPACK/Source/{filename}");
        cached_compilation(builder, &path, &filename, build_cache, log_file, blas);
    }

    let umfpack2: &Vec<String> = &fs::read_dir("SuiteSparse/UMFPACK/Source2")
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .filter(|f| f.ends_with(".c"))
        .collect();
    for filename in umfpack2 {
        let path = format!("SuiteSparse/UMFPACK/Source2/{filename}");
        cached_compilation(builder, &path, &filename, build_cache, log_file, blas);
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
    filename: &str,
    build_cache: &Vec<String>,
    log_file: &mut File,
    blas: &str,
) {
    let binary = stem(&filename);
    let lib_name = format!("lib{binary}.a");

    if build_cache.iter().any(|n| n == &lib_name) {
        return; // already compiled, no need to do it again.
    }

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
        "{}/umfpack/out/{}/{}.o",
        cache_dir.to_str().unwrap(),
        blas,
        stem(&path)
    );
    let cached_binary_path = PathBuf::from(&cached_binary);
    let cached_folder = cached_binary_path.parent().unwrap();
    let cached_library = format!(
        "{}/umfpack/out/{}/lib{}.a",
        cache_dir.to_str().unwrap(),
        blas,
        binary
    );
    let cached_library_path = PathBuf::from(&cached_library);

    if !cached_folder.exists() {
        fs::create_dir_all(&cached_folder).unwrap();
    }

    if cached_binary_path.exists() & cached_library_path.exists() {
        return;
    }

    writeln!(log_file, "compiling {filename}").unwrap();
    builder.file(path).compile(binary);
    std::fs::copy(&out_binary, &cached_binary).unwrap();
    std::fs::copy(&out_library, &cached_library).unwrap();
}

fn suitesparse_includes<'a>() -> Vec<&'a str> {
    vec![
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

#[cfg(feature = "s3-sync")]
fn sync_s3_cache(cache_dir: &str, _log_file: &mut File, blas: &str) {
    use rayon::iter::ParallelBridge;
    use rayon::prelude::ParallelIterator;
    use rusoto_core::Region;
    use rusoto_s3::{GetObjectRequest, ListObjectsV2Request, S3Client, S3};
    use tokio::runtime::Runtime;

    let region = Region::UsWest2;
    let s3_client = S3Client::new(region);
    let bucket = "umfpack";
    let prefix = format!("linux/{blas}");
    let request = ListObjectsV2Request {
        bucket: bucket.to_owned(),
        prefix: Some(prefix.to_owned()),
        ..Default::default()
    };
    let rt = Runtime::new().unwrap();
    let response = rt.block_on(s3_client.list_objects_v2(request)).unwrap();
    let objects = response.contents.unwrap();
    let _: Vec<()> = objects
        .into_iter()
        .par_bridge()
        .map(|object| {
            let key = object.key.unwrap();
            let short_key = key.split("/").fold("", |_, part| part);
            let local_path = format!("{}/{}", cache_dir, short_key);
            let request = GetObjectRequest {
                bucket: bucket.to_owned(),
                key: key.to_owned(),
                ..Default::default()
            };
            let response = rt.block_on(s3_client.get_object(request)).unwrap();
            let body = response.body.unwrap();
            let mut buffer = Vec::new();
            body.into_blocking_read().read_to_end(&mut buffer).unwrap();
            let mut file = File::create(&local_path).unwrap();
            file.write_all(&buffer).unwrap();
            // writeln!(log_file, "downloading {short_key} to {local_path}").unwrap();
        })
        .collect();
}

fn build_blas(builder: &mut Build, blas: &str) {
    match blas {
        "no-blas" => {
            builder.flag("-DNBLAS");
        }
        "blas-static" => {
            println!("cargo:rustc-link-lib=static=blas");
        }
        "openblas-static" => {
            println!("cargo:rustc-link-lib=static=openblas");
        }
        "blas" => {
            println!("cargo:rustc-link-lib=dylib=blas");
        }
        "openblas" => {
            println!("cargo:rustc-link-lib=dylib=openblas");
        }
        _ => {
            let msg = "Please enable one of the following features: ";
            let msg = format!("{msg} 'no-blas', 'blas', 'blas-static', ");
            let msg = format!("{msg} 'openblas', 'openblas-static'.");
            panic!("{msg}");
        }
    };
}

fn get_blas_feature(_log_file: &mut File) -> String {
    cfg_if! {
        if #[cfg(feature = "blas-static")] {
            return "blas-static".to_owned();
        } else if  #[cfg(feature = "openblas-static")] {
            return "openblas-static".to_owned();
        } else if  #[cfg(feature = "blas")] {
            return "blas".to_owned();
        } else if  #[cfg(feature = "openblas")] {
            return "openblas".to_owned();
        } else {
            return "no-blas".to_owned();
        }
    };
}
