use std::time::Duration;

fn touch_arti_dependency() {
    // Force Arti to be linked so release-size measurements include the Tor client stack,
    // without changing runtime behavior yet.
    std::hint::black_box(arti_client::TorClientConfig::default());
}

/// Build a reqwest Client that uses webpki-roots for TLS cert verification,
/// bypassing rustls-platform-verifier (which requires Android JNI init)
pub fn new_client() -> Result<reqwest::Client, reqwest::Error> {
    touch_arti_dependency();

    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let tls_config =
        rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth();

    reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .tls_backend_preconfigured(tls_config)
        .build()
}
