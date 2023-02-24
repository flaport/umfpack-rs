#[cfg(feature = "c")]
fn main() {
    let filenames = ["example.c"];
    for filename in filenames {
        let path = format!("examples/{filename}");
        println!("cargo:rerun-if-changed={path}");
        cc::Build::new()
            .file(path)
            .compile(stem(filename));
    }

    // let filenames: Vec<String> = fs::read_dir("vendor")
    //     .unwrap()
    //     .map(|f| f.unwrap().file_name().into_string().unwrap())
    //     .filter(|f| f.ends_with(".c"))
    //     .collect();
    // for filename in filenames {
    //     let path = format!("vendor/{filename}");
    //     println!("cargo:rerun-if-changed={path}");
    //     cc::Build::new()
    //         .file(path)
    //         .include("vendor")
    //         .compile(stem(&filename));
    // }
}

#[cfg(not(feature = "c"))]
fn main() {}

fn stem(filename: &str) -> &str {
    let mut parts = filename.split(".");
    return parts.next().unwrap();
}
