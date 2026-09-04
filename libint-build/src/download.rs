use anyhow::Result;
use std::path::{Path, PathBuf};
use ureq::{
    config::Config,
    tls::{TlsConfig, TlsProvider},
};

const LIBINT_VERSION: &str = "2.13.1";

fn libint_source_url() -> String {
    format!("https://github.com/evaleev/libint/archive/refs/tags/v{LIBINT_VERSION}.tar.gz")
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
