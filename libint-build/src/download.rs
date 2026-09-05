use anyhow::Result;
use std::path::{Path, PathBuf};
use ureq::{
    config::Config,
    tls::{TlsConfig, TlsProvider},
};

// Tracking `master` HEAD rather than a release tag: v2.13.1 (the latest tag as of
// writing) has an install bug where the compiled static libs never get copied into
// CMAKE_INSTALL_PREFIX on hosts where CMAKE_INSTALL_LIBDIR resolves to `lib64` (fixed
// post-release upstream). Switch back to a pinned tag once a release ships the fix.
const LIBINT_VERSION: &str = "master";

fn libint_source_url() -> String {
    format!("https://github.com/evaleev/libint/archive/refs/heads/{LIBINT_VERSION}.tar.gz")
}

fn get_agent() -> ureq::Agent {
    Config::builder()
        .tls_config(TlsConfig::builder().provider(TlsProvider::Rustls).build())
        .build()
        .new_agent()
}

pub fn download(out_dir: &Path) -> Result<PathBuf> {
    let dest = out_dir.join(format!("libint-{LIBINT_VERSION}"));
    if !dest.exists() {
        let buf = get_agent()
            .get(&libint_source_url())
            .call()?
            .into_body()
            .into_reader();
        let gz_stream = flate2::read::GzDecoder::new(buf);
        let mut ar = tar::Archive::new(gz_stream);
        ar.unpack(out_dir)?;
        assert!(dest.exists());
    }
    Ok(dest)
}
