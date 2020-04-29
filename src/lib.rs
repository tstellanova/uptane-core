#![no_std]
/**
Copyright (c) 2020, Todd Stellanova
All rights reserved.

License: See LICENSE file
*/

// use chrono::{DateTime, FixedOffset, TimeZone};


mod fetcher;
// use fetcher::FileFetcher;

pub mod metadata;
use metadata::SignatureMethod;
use crate::metadata::{KeyContainer, SignatureContainer, MetadataFormat};

/// Unless otherwise noted, this implementation is intended to comply
/// with the "Uptane Standard for Design and Implementation" version 1.0.1
/// originally obtained [here](https://uptane.github.io/uptane-standard/uptane-standard.html).
/// Comments will frequently refer to section numbers (eg "5.4.4") in that standard.
///
/// From Uptane standard: 5.4.4. Metadata verification procedures:
/// - A Primary ECU MUST perform full verification of metadata.
/// - A Secondary ECU SHOULD perform full verification of metadata.
/// If a Secondary cannot perform full verification, it SHALL perform partial verification instead.
/// If a step in the following workflows does not succeed
/// (e.g., the update is aborted because a new metadata file was not signed),
/// an ECU SHOULD still be able to update again in the future.
/// Errors raised during the update process SHOULD NOT leave ECUs in an unrecoverable state.


/// This library supports full verification of metadata.
/// Full verification of metadata means that we check that the Targets metadata
/// about images from the Director repository matches the Targets metadata about the
/// same images from the Image repository. This provides resilience to a single key compromise.



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



pub struct Verifier {}

impl Verifier {
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
    pub fn verify_signature_over_metadata(
        key: &KeyContainer,
        sig: &SignatureContainer,
        data: &[u8],
        metadata: &[u8],
        format: MetadataFormat,
    ) -> Result<(), crate::Error> {

        //TODO call verify_signature
        Ok(())
    }

    /// Verify signature for data
    fn verify_signature(public_key: &[u8],
                        method: SignatureMethod,
                        signature: &[u8],
                        data: &[u8]
    ) -> Result<(), crate::Error> {

        // DateTime::parse_from_rfc3339()
        match method {
            SignatureMethod::RsaSsaPss => {

            }
            SignatureMethod::Ed25519 => {

            }
            SignatureMethod::NaCl => {

            }
        }

        Ok(())
    }


    /// Verify that the Targets metadata about an image, from the Director repository,
    /// matches the Targets metadata about the same image from the Image repository.
    /// This fulfills Uptane "Full Verification" requirements
    pub fn full_image_verification(director_targets: u32, image_targets: u32) -> Result<(), crate::Error> {

        // Verify that Targets metadata from the Director and Image repositories match.
        // A Primary ECU MUST perform this check on metadata for all images listed in
        // the Targets metadata file from the Director repository downloaded in step 6.
        //
        // A Secondary ECU MAY elect to perform this check only on the metadata for the
        // image it will install. (That is, the target metadata from the Director that
        // contains the ECU identifier of the current ECU.)
        // To check that the metadata for an image matches, complete the following procedure:
        // - Locate and download a Targets metadata file from the Image repository that contains
        // an image with exactly the same file name listed in the Director metadata, following
        // the procedure in Section 5.4.4.7.
        // - Check that the Targets metadata from the Image repository matches the Targets metadata
        // from the Director repository:
        // - Check that the non-custom metadata (i.e., length and hashes) of the unencrypted or
        // encrypted image are the same in both sets of metadata. Note: the Primary is responsible
        // for validating encrypted images and associated metadata. The target
        // ECU (Primary or Secondary) is responsible for validating the unencrypted image and
        // associated metadata.
        // - Check that all “MUST match” custom metadata (e.g., hardware identifier and
        // release counter) are the same in both sets of metadata.
        // - Check that the release counter in the previous Targets metadata file is less than
        // or equal to the release counter in this Targets metadata file.

        Ok(())
    }

}



