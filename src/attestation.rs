use crate::certificate;
use crate::errors::*;
use data_encoding::BASE64;
use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub struct AttestationSummary {
    pub commit: String,
    pub repository: String,
    pub subject: String,
}

#[derive(Debug, Deserialize)]
pub struct Provenance {
    pub attestation_bundles: Vec<AttestationBundle>,
}

#[derive(Debug, Deserialize)]
pub struct AttestationBundle {
    pub attestations: Vec<Attestation>,
}

#[derive(Debug, Deserialize)]
pub struct Attestation {
    pub envelope: Envelope,
    pub verification_material: VerificationMaterial,
}

#[derive(Debug, Deserialize)]
pub struct Envelope {
    // pub signature: String,
    pub statement: String,
}

#[derive(Debug, Deserialize)]
pub struct InTotoStatement {
    pub _type: String,
    pub subject: Vec<InTotoSubject>,
}

impl InTotoStatement {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let statement = BASE64.decode(bytes)?;
        let statement = serde_json::from_slice::<Self>(&statement)?;
        debug!("Parsed in-toto statement from attestation: {statement:?}");
        Ok(statement)
    }
}

#[derive(Debug, Deserialize)]
pub struct InTotoSubject {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct VerificationMaterial {
    pub certificate: String,
}

pub fn parse(bytes: &[u8]) -> Result<AttestationSummary> {
    let provenance =
        serde_json::from_slice::<Provenance>(bytes).context("Failed to parse provenance json")?;
    debug!("Parsed in-toto provenance: {provenance:?}");
    let Some(bundle) = provenance.attestation_bundles.first() else {
        bail!("Failed to find any bundles in provenance data")
    };
    let Some(attestation) = bundle.attestations.first() else {
        bail!("Failed to find any attestation in provenance data")
    };

    let cert_der = BASE64.decode(attestation.verification_material.certificate.as_bytes())?;
    let cert = certificate::parse(&cert_der)?;

    let statement = InTotoStatement::parse(attestation.envelope.statement.as_bytes())?;
    let Some(subject) = statement.subject.into_iter().next() else {
        bail!("Failed to find subject in in-toto statement")
    };

    Ok(AttestationSummary {
        commit: cert.commit,
        repository: cert.repository,
        subject: subject.name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cryptography_43_0_3() {
        let data = include_bytes!("../test_data/cryptography-43.0.3.provenance");
        let provenance = parse(data).unwrap();
        assert_eq!(
            provenance,
            AttestationSummary {
                commit: "5050fe5a0cf7f5c023e5068724f443eafb7cbca9".to_string(),
                repository: "https://github.com/pyca/cryptography".to_string(),
                subject: "cryptography-43.0.3.tar.gz".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_sigstore_3_5_1() {
        let data = include_bytes!("../test_data/sigstore-3.5.1.provenance");
        let provenance = parse(data).unwrap();
        assert_eq!(
            provenance,
            AttestationSummary {
                commit: "0ac33eeaeb62ca466cef2708ca1dd5864382a008".to_string(),
                repository: "https://github.com/sigstore/sigstore-python".to_string(),
                subject: "sigstore-3.5.1.tar.gz".to_string(),
            }
        );
    }
}
