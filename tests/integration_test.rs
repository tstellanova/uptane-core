
use uptane_core::{Verifier, KeyContainer, KeyKind, MetadataFormat};


#[test]
fn fnorp() {
    let my_key = KeyContainer {
        key_type: KeyKind::RSA,
        key_id: None,
        public_key: None,
        private_key: None
    };

    let res = Verifier::verify_signature_over_metadata(
        &my_key,
        "SIGNATURE".as_ref(),
        "DATABARN".as_ref(),
        "METADATA".as_ref(),
        MetadataFormat::JSON,
    );

    assert!(res.is_ok(), "verification failed");

}

