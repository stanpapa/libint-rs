fn build_cpp_bindings() {
    // define list of bridges
    // it is assumed that all bridges are defined in bridge/{bridge}.rs
    let bridges = ["atom", "initialize", "shell"];

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

    for cc in &sources {
        println!("cargo:rerun-if-changed={}", cc.to_str().unwrap());

        let header = shim_root
            .join("include")
            .join(format!("{}.h", cc.file_stem().unwrap().display()));
        println!("cargo:rerun-if-changed={}", header.to_str().unwrap());
    }

    cxx_build::bridges(bridges.map(|bridge| format!("src/bridge/{bridge}.rs")))
        .files(sources)
        .compile("libint-sys");

    for bridge in bridges {
        println!("cargo:rerun-if-changed=src/bridge/{bridge}.rs");
    }
}

fn main() {
    build_cpp_bindings();
}
