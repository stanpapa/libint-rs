fn feature_enabled(feature: &str) -> bool {
    std::env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

fn main() {
    // don't do anything when requesting docs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    if feature_enabled("system") {
        let lib = pkg_config::Config::new()
            .statik(feature_enabled("static"))
            .cargo_metadata(false)
            .probe("libint2")
            .unwrap();

        // forward include paths to -sys crate
        for path in &lib.include_paths {
            println!("cargo:include={}", path.display());
        }

        // emit lib search paths
        for path in &lib.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }

        // Let pkg_config handle all other dependencies normally (Boost, Eigen, etc.)
        pkg_config::Config::new()
            .probe("eigen3")
            .unwrap()
            .include_paths
            .iter()
            .for_each(|path| {
                println!("cargo:include={}", path.display());
            });
        // pkg_config::Config::new().probe("boost").unwrap();
    } else {
        todo!("libint-build")
    }
}
