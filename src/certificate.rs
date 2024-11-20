use crate::errors::*;
use asn1_rs::{oid, Oid};
use std::collections::HashMap;
use std::str;
use x509_parser::extensions::X509Extension;

#[derive(Debug)]
pub struct Certificate {
    pub repository: String,
    pub commit: String,
}

pub fn x509_extension_utf8<'a>(
    extensions: &'a HashMap<Oid, &X509Extension>,
    oid: &Oid,
    label: &str,
) -> Result<&'a str> {
    let Some(ext) = extensions.get(oid) else {
        bail!("Failed to find {label} extension in certificate")
    };

    // TODO: this is slightly naive
    let (_len, value) = ext.value.split_at(2);

    let value = str::from_utf8(value)
        .with_context(|| anyhow!("Failed to parse {label} extension as utf-8"))?;

    info!("Parsed {label} extension from certificate: {value:?}");
    Ok(value)
}

pub fn parse(bytes: &[u8]) -> Result<Certificate> {
    let (_, cert) =
        x509_parser::parse_x509_certificate(bytes).context("Failed to parse x509 certificate")?;

    let extensions = cert.extensions_map()?;

    let source_repository_uri = x509_extension_utf8(
        &extensions,
        &oid!(1.3.6 .1 .4 .1 .57264 .1 .12),
        "source repository uri",
    )?;

    let source_repository_digest = x509_extension_utf8(
        &extensions,
        &oid!(1.3.6 .1 .4 .1 .57264 .1 .13),
        "source repository digest",
    )?;

    Ok(Certificate {
        repository: source_repository_uri.to_string(),
        commit: source_repository_digest.to_string(),
    })
}
