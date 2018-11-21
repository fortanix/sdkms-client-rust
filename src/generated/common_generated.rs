/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Operations allowed to be performed on a given key.
pub use self::key_operations::KeyOperations;
pub mod key_operations {
    bitflags_set! {
        pub struct KeyOperations: u64 {
            const SIGN = 0x0000000000000001;
            const VERIFY = 0x0000000000000002;
            const ENCRYPT = 0x0000000000000004;
            const DECRYPT = 0x0000000000000008;
            const WRAPKEY = 0x0000000000000010;
            const UNWRAPKEY = 0x0000000000000020;
            const DERIVEKEY = 0x0000000000000040;
            const MACGENERATE = 0x0000000000000080;
            const MACVERIFY = 0x0000000000000100;
            const EXPORT = 0x0000000000000200;
            const APPMANAGEABLE = 0x0000000000000400;
            const HIGHVOLUME = 0x0000000000000800;
            const AGREEKEY = 0x0000000000001000;
        }
    }
}

/// Type of security object.
#[derive(Debug, Eq, PartialEq, Copy, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ObjectType {
    Aes,
    Des,
    Des3,
    Rsa,
    Ec,
    Opaque,
    Hmac,
    Secret,
    Certificate,
}

/// The origin of a security object - where it was created / generated.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum ObjectOrigin {
    FortanixHSM,
    Transient,
    External,
}

/// Identifies a standardized elliptic curve.
#[derive(Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum EllipticCurve {
    X25519,
    Ed25519,
    X448,
    SecP192K1,
    SecP224K1,
    SecP256K1,
    NistP192,
    NistP224,
    NistP256,
    NistP384,
    NistP521,
    Gost256A,
}

/// Linked security objects.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyLinks {
    #[serde(default)]
    pub replacement: Option<Uuid>,
    #[serde(default)]
    pub replaced: Option<Uuid>,
}

/// A security principal.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Principal {
    App(Uuid),
    User(Uuid),
    Plugin(Uuid),
    /// UserViaApp signifies a user authorizing some app to act on its behalf through OAuth.
    UserViaApp {
        user_id: Uuid,
        scopes: HashSet<OauthScope>,
    },
}

/// A hash algorithm.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum DigestAlgorithm {
    Blake2b256,
    Blake2b384,
    Blake2b512,
    Blake2s256,
    Ripemd160,
    Ssl3,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
    Streebog256,
    Streebog512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

/// OAuth scope.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OauthScope {
    App,
}

singleton_backcompat! {
    /// User's role in a group.
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum UserGroupRole {
        GroupAuditor,
        GroupAdministrator
    }
}

/// Signing keys used to validate signed JWT tokens.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum JwtSigningKeys {
    Stored {
        /// Mapping key ids to DER-encoded public key.
        keys: HashMap<String, Blob>,
    },
    Fetched {
        url: String,
        /// Number of seconds that the service is allowed to cache the fetched keys.
        cache_duration: u64,
    },
}

/// Constraints on RSA encryption parameters. In general, if a constraint is not specified, anything is allowed.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RsaEncryptionPolicy {
    pub padding: Option<RsaEncryptionPaddingPolicy>,
}

/// RSA encryption padding policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaEncryptionPaddingPolicy {
    Oaep { mgf: Option<MgfPolicy> },
    Pkcs1V15 {},
}

/// Constraints on RSA signature parameters. In general, if a constraint is not specified, anything is allowed.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RsaSignaturePolicy {
    pub padding: Option<RsaSignaturePaddingPolicy>,
}

/// RSA signature padding policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaSignaturePaddingPolicy {
    Pss { mgf: Option<MgfPolicy> },
    Pkcs1V15 {},
}

/// MGF policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MgfPolicy {
    Mgf1 { hash: Option<DigestAlgorithm> },
}

/// RSA-specific options.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct RsaOptions {
    /// Size in bits (not bytes) of the RSA key. Specify on Create only. Returned on Get.
    pub key_size: Option<u32>,
    /// Public exponent to use for generating the RSA key. Specify on Create only.
    #[serde(default)]
    pub public_exponent: Option<u32>,
    /// Encryption policy for an RSA key. When doing an encryption or key wrapping operation, the
    /// policies are evaluated against the specified parameters one by one. If one matches, the
    /// operation is allowed. If none match, including if the policy list is empty, the operation
    /// is disallowed. Missing optional parameters will have their defaults specified according to
    /// the matched policy. The default for new keys is `[{"padding":{"OAEP":{}}]`.
    /// If (part of) a constraint is not specified, anything is allowed for that constraint.
    /// To impose no constraints, specify `[{}]`.
    pub encryption_policy: Vec<RsaEncryptionPolicy>,
    /// Signature policy for an RSA key. When doing a signature operation, the policies are
    /// evaluated against the specified parameters one by one. If one matches, the operation is
    /// allowed. If none match, including if the policy list is empty, the operation is disallowed.
    /// Missing optional parameters will have their defaults specified according to the matched
    /// policy. The default for new keys is `[{}]` (no constraints).
    /// If (part of) a constraint is not specified, anything is allowed for that constraint.
    pub signature_policy: Vec<RsaSignaturePolicy>,
}

/// FPE-specific options.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct FpeOptions {
    /// The base for input data.
    pub radix: u32,
    /// The minimum allowed length for the input data.
    pub min_length: u32,
    /// The maximum allowed length for the input data.
    pub max_length: u32,
    /// The list of indices of characters to be preserved while performing encryption/decryption.
    pub preserve: Vec<isize>,
    /// The list of indices of characters to be masked while performing masked decryption.
    pub mask: Option<Vec<isize>>,
    /// Whether encrypted/decrypted data should satisfy LUHN checksum formula.
    pub luhn_check: Option<bool>,
    /// The user-friendly name for the data type that represents the input data.
    pub name: Option<String>,
}

/// Approval policy.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ApprovalPolicy {
    #[serde(default)]
    pub quorum: Option<ApprovalPolicyQuorum>,
    #[serde(default)]
    pub user: Option<Uuid>,
    #[serde(default)]
    pub app: Option<Uuid>,
}

/// Quorum approval policy.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ApprovalPolicyQuorum {
    pub n: usize,
    pub members: Vec<ApprovalPolicy>,
    #[serde(flatten)]
    pub config: ApprovalAuthConfig,
}

/// Authentication requirements for approval request reviewers.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalAuthConfig {
    pub require_password: bool,
    pub require_2fa: bool,
}

/// Reason for revoking a key.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevocationReason {
    pub code: RevocationReasonCode,
    /// Message is used exclusively for audit trail/logging purposes and MAY contain additional
    /// information about why the object was revoked.
    pub message: Option<String>,
    pub compromise_occurance_date: Option<Time>,
}

/// Reasons to revoke a security object.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum RevocationReasonCode {
    Unspecified,
    KeyCompromise,
    CACompromise,
    AffiliationChanged,
    Superseded,
    CessationOfOperation,
    PrivilegeWithdrawn,
}

/// If enabled, the public key will be available publicly (without authentication) through the GetPublicKey API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum PublishPublicKeyConfig {
    Enabled {
        /// Additionally list the previous version of the key if not compromised.
        list_previous_version: bool,
    },
    Disabled,
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Sobject {
    pub acct_id: Uuid,
    #[serde(default)]
    pub activation_date: Option<Time>,
    #[serde(default)]
    pub compromise_date: Option<Time>,
    pub created_at: Time,
    pub creator: Principal,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    #[serde(default)]
    pub deactivation_date: Option<Time>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub deterministic_signatures: Option<bool>,
    #[serde(default)]
    pub elliptic_curve: Option<EllipticCurve>,
    pub enabled: bool,
    #[serde(default)]
    pub fpe: Option<FpeOptions>,
    pub key_ops: KeyOperations,
    #[serde(default)]
    pub key_size: Option<u32>,
    #[serde(default)]
    pub kid: Option<Uuid>,
    pub lastused_at: Time,
    #[serde(default)]
    pub links: Option<KeyLinks>,
    #[serde(default)]
    pub name: Option<String>,
    pub never_exportable: Option<bool>,
    pub obj_type: ObjectType,
    pub origin: ObjectOrigin,
    #[serde(default)]
    pub pub_key: Option<Blob>,
    pub public_only: bool,
    #[serde(default)]
    pub publish_public_key: Option<PublishPublicKeyConfig>,
    #[serde(default)]
    pub revocation_reason: Option<RevocationReason>,
    #[serde(default)]
    pub rsa: Option<RsaOptions>,
    #[serde(default)]
    pub state: Option<SobjectState>,
    #[serde(default)]
    pub transient_key: Option<Blob>,
    #[serde(default)]
    pub value: Option<Blob>,
    #[serde(default)]
    pub group_id: Option<Uuid>,
}

/// A request to sign data (or hash value) using an asymmetric key.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub hash_alg: DigestAlgorithm,
    /// Hash value to be signed. Exactly one of `hash` and `data` is required.
    pub hash: Option<Blob>,
    /// Data to be signed. Exactly one of `hash` and `data` is required.
    /// To reduce request size and avoid reaching the request size limit, prefer `hash`.
    pub data: Option<Blob>,
    pub mode: Option<SignatureMode>,
    pub deterministic_signature: Option<bool>,
}

/// Result of sign operation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignResponse {
    /// Key id is returned for non-transient keys.
    #[serde(default)]
    pub kid: Option<Uuid>,
    pub signature: Blob,
}

/// Request to verify a signature using an asymmetric key.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub hash_alg: DigestAlgorithm,
    /// The hash of the data on which the signature is being verified.
    /// Exactly one of `hash` and `data` is required.
    pub hash: Option<Blob>,
    /// The data on which the signature is being verified.
    /// Exactly one of `hash` and `data` is required.
    /// To reduce request size and avoid reaching the request size limit, prefer `hash`.
    pub data: Option<Blob>,
    pub mode: Option<SignatureMode>,
    /// The signature to verify.
    pub signature: Blob,
}

/// Result of verifying a signature or MAC.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct VerifyResponse {
    /// Key id is returned for non-transient keys.
    #[serde(default)]
    pub kid: Option<Uuid>,
    /// True if the signature verified and false if it did not.
    pub result: bool,
}

/// Specifies the Mask Generating Function (MGF) to use.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Mgf {
    Mgf1 { hash: DigestAlgorithm },
}

/// Signature mode.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SignatureMode {
    Rsa(RsaSignaturePadding),
}

/// Type of padding to use for RSA signatures. The padding specified must adhere to the key's
/// signature policy. If not specified, the default based on the key's policy will be used.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaSignaturePadding {
    /// Probabilistic Signature Scheme (PKCS#1 v2.1).
    Pss { mgf: Mgf },
    /// PKCS#1 v1.5 padding.
    Pkcs1V15 {},
}

/// Uniquely identifies a persisted or transient sobject.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SobjectDescriptor {
    Kid(Uuid),
    Name(String),
    TransientKey(Blob),
}

/// Request for second factor authentication with a U2f device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct U2fAuthRequest {
    pub key_handle: Blob,
    pub signature_data: Blob,
    pub client_data: Blob,
}

#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
pub enum SobjectState {
    PreActive,
    Active,
    Deactivated,
    Compromised,
}
