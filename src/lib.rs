/**
Copyright (c) 2020, Todd Stellanova
All rights reserved.

License: See LICENSE file
 */

#![no_std]


/// From Uptane standard: 5.4.4. Metadata verification procedures
/// A Primary ECU MUST perform full verification of metadata.
/// A Secondary ECU SHOULD perform full verification of metadata.
/// If a Secondary cannot perform full verification,
/// it SHALL perform partial verification instead.
/// If a step in the following workflows does not succeed
/// (e.g., the update is aborted because a new metadata file was not signed),
/// an ECU SHOULD still be able to update again in the future.
/// Errors raised during the update process SHOULD NOT leave ECUs in an unrecoverable state.


/// This library supports full verification of metadata,
/// listed in  Uptane standard: 5.4.4 as a SHOULD for Secondaries


/// Errors in this crate
#[derive(Debug)]
pub enum Error {
    /// Invalid key format
    InvalidKeyFormat,
    /// Unsupported signing method
    UnsupportedSigningMethod,
    /// Unsupported metadata format (eg json)
    UnsupportedMetadataFormat,
    /// Could not verify metadata
    MetadataVerification,
    /// Invalid signature
    SignatureInvalid,
    /// Invalid Hash
    HashInvalid,
}

pub enum KeyType {
    RSA,
    Ed25519,
    Ecdsa,
}

pub struct KeyContainer<'a> {
    pub keytype: KeyType,
    pub keyid: Option<&'a str>,
    /// eg  '-----BEGIN RSA PUBLIC KEY----- ...'
    pub public_keyval: Option<&'a str>,
    /// eg '-----BEGIN RSA PRIVATE KEY----- ...'
    pub private_keyval: Option<&'a str>,
}

pub enum MetadataFormat {
    /// JSON or canonical JSON format?
    JSON,
    /// ASN.1/DER (may not be supported)
    DER,
}

pub struct UptaneCommonVerifier {}

impl UptaneCommonVerifier {
    //def verify_signature_over_metadata(
    //     key_dict, signature, data, datatype,
    //     metadata_format=tuf.conf.METADATA_FORMAT):


    /// Checks whether the signer (whose keys are provided) signed the given object
    /// to produce the given signature.
    ///
    /// - `key` The signer's key
    /// - `sig` The signature to be verified
    /// - `data` Data object used by create_signature() to generate the signature.
    /// - `metadata` the metadata to be verified
    /// - `format` the format of the metadata
    ///
    /// Returns Ok if verified, errors if not
    fn verify_signature_over_metadata(
        key: &KeyContainer,
        sig: &[u8],
        data: &[u8],
        metadata: &[u8],
        format: MetadataFormat,
    ) -> Result<(), crate::Error> {

        Ok(())
    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
