
/// Types of keys we expect to support
pub enum KeyKind {
    Ed25519,
    Ecdsa,
    RSA,
}

/// Wraps key data along with its type
pub struct KeyContainer<'a> {
    /// The type of key
    pub key_type: KeyKind,
    /// The public key identifier for this key
    pub key_id: Option<&'a str>,
    /// public key data, may be, for example, formatted as a PEM string
    /// or eg  '-----BEGIN RSA PUBLIC KEY----- ...'
    pub public_key: Option<&'a str>,
    /// private key data
    /// eg '-----BEGIN RSA PRIVATE KEY----- ...'
    pub private_key: Option<&'a str>,
}

/// The particular scheme used to verify the signature, such as
/// rsassa-pss-sha256 or ecdsa-sha2-nistp256
pub enum SignatureMethod {
    /// 'ed25519'
    Ed25519,
    /// 'nacl'
    NaCl,
    /// 'rsassa-pss'
    RsaSsaPss,
}

pub struct SignatureContainer<'a> {
    pub key_id: Option<&'a str>,
    pub method: SignatureMethod,
}

/// Supported hashing algorithms
pub enum HashMethod {
    Sha256,
    Sha512,
}

/// Metadata formats used when communicating with repositories
pub enum MetadataFormat {
    /// JSON or canonical JSON format?
    JSON,
    /// ASN.1/DER (may not be supported)
    DER,
}


// /// An indicator of the type of role
// pub enum RoleKind {
//     Root,
//     Targets,
//     Snapshot,
//     Timestamp
// }

/// numeric timestamp type, unix timestamp format
type ExpirationTimestamp = i64;
type MetadataVersionNumber = u32;
type ImageFileLength = usize;

pub struct CommonRoleMetadata {
    /// expiration date and time
    /// derived from eg   "expires": "2030-01-01T00:00:00Z",
    expiration_timestamp: ExpirationTimestamp,
    /// integer version number, which SHOULD be incremented each time the metadata file is updated
    version: MetadataVersionNumber,
}


/// Contains information about a target image
pub struct ImageMetadata<'a> {
    file_name: Option<&'a str>,
    file_length: ImageFileLength,
    file_hash: Option<&'a str>,
    hash_method: HashMethod
}

/// See section 5.2.2: Root metadata
/// Not implemented: TAP-5 support for mapping roles to role metadata URLs
pub struct RootMetadata<'a> {
    ///
    info: CommonRoleMetadata,
    /// Key for the Root role
    root_role_key: KeyContainer<'a>,
    /// threshold of signatures required for the Root role
    root_role_threshold: u8,
    /// Key for the Targets role
    targets_role_key: KeyContainer<'a>,
    /// threshold of signatures required for the Targets role
    targets_role_threshold: u8,
    /// Key for the Snapshot role
    snapshot_role_key: KeyContainer<'a>,
    /// threshold of signatures required for the Snapshot role
    snapshot_role_threshold: u8,
    /// Key for the Timestamp role
    timestamp_role_key: KeyContainer<'a>,
    /// threshold of signatures required for the Timestamp role
    timestamp_role_threshold: u8,
}


//A repository’s Root metadata distributes the public keys of the top-level
// Root, Targets, Snapshot, and Timestamp roles,
// as well as revocations of those keys.
// It SHALL contain two attributes:
//
// - A representation of the public keys for all four roles. Each key SHALL have a unique
// public key identifier.
// - An attribute mapping each role to (1) its public key(s), and (2) the threshold of
// signatures required for that role
