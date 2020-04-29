
use uptane_core::{Verifier};
// use uptane_core::metadata::{KeyContainer, KeyKind, MetadataFormat};

#[test]
fn full_verification() {

    let res = Verifier::full_image_verification(0, 0);
    assert!(res.is_ok(), "full verification failed")
}

//
// #[test]
// fn fnorp() {
//     let my_key = KeyContainer {
//         key_type: KeyKind::RSA,
//         key_id: None,
//         public_key: None,
//         private_key: None
//     };
//
//     let res = Verifier::verify_signature_over_metadata(
//         &my_key,
//         "SIGNATURE".as_ref(),
//         "DATABARN".as_ref(),
//         "METADATA".as_ref(),
//         MetadataFormat::JSON,
//     );
//
//     assert!(res.is_ok(), "verification failed");
//
// }

