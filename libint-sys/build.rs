use std::path::PathBuf;

fn build() {
    // define list of bridges
    // it is assumed that all bridges are defined in bridge/{bridge}.rs
    let bridges = ["atom", "basis", "initialize", "shell"];

    // collect shims
    let shim_root = std::path::PathBuf::from("shim");
    let dir = std::fs::read_dir(shim_root.join("src")).unwrap();
    let sources = dir
        .into_iter()
        .filter_map(|p| match p {
            Ok(p) => {
                if p.metadata().unwrap().is_file() {
                    Some(p.path())
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    // re-compile if one of the shims or bridges has changed
    for cc in &sources {
        println!("cargo:rerun-if-changed={}", cc.to_str().unwrap());

        let header = shim_root
            .join("include")
            .join(format!("{}.h", cc.file_stem().unwrap().display()));
        println!("cargo:rerun-if-changed={}", header.to_str().unwrap());
    }

    for bridge in bridges {
        println!("cargo:rerun-if-changed=src/bridge/{bridge}.rs");
    }

    // parse include paths passed from `libint-src`
    let includes = match std::env::var("DEP_INT2_INCLUDE") {
        Ok(s) => std::env::split_paths(&s).collect::<Vec<PathBuf>>(),
        Err(e) => panic!("{e}"),
    };

    // link libint, so libint_static_init will be found
    // TODO: figure out dynamic linking
    println!("cargo:rustc-link-lib=static:+whole-archive=int2");

    // compile bridge
    cxx_build::bridges(bridges.map(|bridge| format!("src/bridge/{bridge}.rs")))
        .files(sources)
        .includes(includes)
        .std("c++17")
        .compile("libint-sys");
}

fn main() {
    build();
}
