fn feature_enabled(feature: &str) -> bool {
    std::env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

fn main() {
    // don't do anything when requesting docs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    if feature_enabled("system") {
        // use pkg-config to find libint2
        match pkg_config::Config::new()
            .statik(feature_enabled("static"))
            .probe("libint2")
        {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        }
    } else {
        todo!("libint-build")
    }

    // if feature_enabled("static") {
    //     println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    //     println!("cargo:rustc-link-arg=-Wl,-llibint2");
    //     println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    // }
}
