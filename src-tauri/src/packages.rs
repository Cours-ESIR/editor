use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};

use log::info;
use typst::diag::{eco_format, PackageError, PackageResult};

use typst_syntax::package::PackageSpec;


/// Make a package available in the on-disk cache.
pub fn prepare_package(spec: &PackageSpec) -> PackageResult<PathBuf> {
    let subdir = format!(
        "typst/packages/{}/{}/{}",
        spec.namespace, spec.name, spec.version
    );

    if let Some(data_dir) = dirs::data_dir() {
        let dir = data_dir.join(&subdir);
        if dir.exists() {
            return Ok(dir);
        }
    }

    if let Some(cache_dir) = dirs::cache_dir() {
        let dir = cache_dir.join(&subdir);

        // Download from network if it doesn't exist yet.
        if spec.namespace == "preview" && !dir.exists() {
            download_package(spec, &dir)?;
        }

        if dir.exists() {
            return Ok(dir);
        }
    }

    Err(PackageError::NotFound(spec.clone()))
}

/// Download a package over the network.
fn download_package(spec: &PackageSpec, package_dir: &Path) -> PackageResult<()> {
    // The `@preview` namespace is the only namespace that supports on-demand
    // fetching.
    assert_eq!(spec.namespace, "preview");

    let url = format!(
        "https://packages.typst.org/preview/{}-{}.tar.gz",
        spec.name, spec.version
    );
    info!(
        "Download {}:{} from typst packages",
        spec.name, spec.version
    );

    let data = match download(&url) {
        Ok(data) => data,
        Err(ureq::Error::Status(404, _)) => return Err(PackageError::NotFound(spec.clone())),
        Err(err) => return Err(PackageError::NetworkFailed(Some(eco_format!("{err}")))),
    };

    let decompressed = flate2::read::GzDecoder::new(data.as_slice());
    tar::Archive::new(decompressed)
        .unpack(package_dir)
        .map_err(|err| {
            fs::remove_dir_all(package_dir).ok();
            PackageError::MalformedArchive(Some(eco_format!("{err}")))
        })
}

fn download(url: &String) -> Result<Vec<u8>, ureq::Error> {
    let mut builder = ureq::AgentBuilder::new();
    let tls = native_tls::TlsConnector::builder();

    // Set user agent.
    builder = builder.user_agent(concat!("typst/", env!("CARGO_PKG_VERSION")));

    // Get the network proxy config from the environment and apply it.
    if let Some(proxy) = env_proxy::for_url_str(url)
        .to_url()
        .and_then(|url| ureq::Proxy::new(url).ok())
    {
        builder = builder.proxy(proxy);
    }

    // Configure native TLS.
    let connector = tls
        .build()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    builder = builder.tls_connector(Arc::new(connector));

    let mut reader = builder.build().get(url).call()?.into_reader();

    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;

    Ok(buf)
}
