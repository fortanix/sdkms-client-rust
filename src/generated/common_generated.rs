/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Role of a user or app in an account.
#[derive(Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountRole {
    AdminUser,
    MemberUser,
    AuditorUser,
    AdminApp,
    CryptoApp,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AesOptions {
    pub key_sizes: Option<Vec<u32>>,
    pub fpe: Option<FpeOptions>,
    pub tag_length: Option<i32>,
    pub cipher_mode: Option<CipherMode>,
    pub random_iv: Option<bool>,
    pub iv_length: Option<i32>,
}

/// A cryptographic algorithm.
#[derive(Debug, Eq, PartialEq, Copy, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Algorithm {
    Aes,
    Des,
    Des3,
    Seed,
    Rsa,
    Dsa,
    Ec,
    Lms,
    Hmac,
    LedaBeta,
    Round5Beta,
    Pbe,
}

/// A helper enum with a single variant, All, which indicates that something should apply to an
/// entire part. (This is here mainly to allow other untagged enums to work properly.)
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum All {
    All,
}

/// Operations allowed to be performed by an app.
pub use self::app_permissions::AppPermissions;
pub mod app_permissions {
    bitflags_set! {
        pub struct AppPermissions: u64 {
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
            const MANAGE = 0x0000000000000400;
            const AGREEKEY = 0x0000000000000800;
            const MASKDECRYPT = 0x0000000000001000;
            const AUDIT = 0x0000000000002000;
        }
    }
}

/// Authentication requirements for approval request reviewers.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalAuthConfig {
    pub require_password: bool,
    pub require_2fa: bool,
}

/// LDAP authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfigLdap {
    pub name: String,
    pub icon_url: String,
    pub ldap_url: String,
    pub dn_resolution: LdapDnResolution,
    pub tls: TlsConfig,
    #[serde(default)]
    pub base_dn: Option<String>,
    #[serde(default)]
    pub user_object_class: Option<String>,
    #[serde(default)]
    pub service_account: Option<LdapServiceAccount>,
    #[serde(default)]
    pub authorization: Option<LdapAuthorizationConfig>,
}

/// CA settings.
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CaConfig {
    CaSet(CaSet),
    Pinned(Vec<Blob>),
}

/// Predefined CA sets.
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CaSet {
    GlobalRoots,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct CertificateOptions {}

/// Cipher mode used for symmetric key algorithms.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum CipherMode {
    Ecb,
    Cbc,
    CbcNoPad,
    Cfb,
    Ofb,
    Ctr,
    Gcm,
    Ccm,
    Kw,
    Kwp,
    Ff1,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ClientConfigurations {
    /// NOTE: not all clients use `common` configurations.
    #[serde(default)]
    pub common: Option<CommonClientConfig>,
    #[serde(default)]
    pub pkcs11: Option<Pkcs11ClientConfig>,
    #[serde(default)]
    pub kmip: Option<KmipClientConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ClientConfigurationsRequest {
    pub common: Option<Option<CommonClientConfig>>,
    pub pkcs11: Option<Option<Pkcs11ClientConfig>>,
    pub kmip: Option<Option<KmipClientConfig>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum ClientFileLogging {
    Enabled(ClientFileLoggingConfig),
    Disabled,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ClientFileLoggingConfig {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub file_size_kb: Option<u64>,
    #[serde(default)]
    pub max_files: Option<u32>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ClientLogConfig {
    #[serde(default)]
    pub system: Option<bool>,
    #[serde(default)]
    pub file: Option<ClientFileLogging>,
    #[serde(default)]
    pub level: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CommonClientConfig {
    #[serde(default)]
    pub retry_timeout_millis: Option<u64>,
    #[serde(default)]
    pub log: Option<ClientLogConfig>,
    #[serde(default)]
    pub h2_num_connections: Option<usize>,
}

/// `CipherMode` or `RsaEncryptionPadding`, depending on the encryption algorithm.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CryptMode {
    Symmetric(CipherMode),
    Rsa(RsaEncryptionPadding),
    Pkcs8Mode(Pkcs8Mode),
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct CryptographicPolicy {
    pub aes: Option<AesOptions>,
    pub des: Option<DesOptions>,
    pub des3: Option<Des3Options>,
    pub seed: Option<SeedOptions>,
    pub rsa: Option<RsaOptions>,
    pub dsa: Option<DsaOptions>,
    pub ec: Option<EcOptions>,
    pub opaque: Option<OpaqueOptions>,
    pub hmac: Option<HmacOptions>,
    pub secret: Option<SecretOptions>,
    pub certificate: Option<CertificateOptions>,
    pub key_ops: Option<KeyOperations>,
    pub legacy_policy: Option<LegacyKeyPolicy>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Des3Options {
    pub key_sizes: Option<Vec<u32>>,
    pub cipher_mode: Option<CipherMode>,
    pub random_iv: Option<bool>,
    pub iv_length: Option<i32>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct DesOptions {
    pub cipher_mode: Option<CipherMode>,
    pub random_iv: Option<bool>,
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
    Sha224,
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

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct DsaOptions {
    pub subgroup_size: Option<u32>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct EcOptions {
    pub elliptic_curves: Option<Vec<EllipticCurve>>,
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

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ExternalKeyId {
    Pkcs11 { id: Blob, label: Blob },
    Fortanix { id: Uuid },
    AwsKms { key_arn: String, key_id: String },
    AzureKeyVault { version: Uuid, label: String },
    GcpKeyRing { version: u32, label: String },
    Wrapped {},
}

/// This describes an external object. Virtual keys in SDKMS store this information instead of the key material.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ExternalSobjectInfo {
    /// The ID of the external object in the external HSM.
    pub id: ExternalKeyId,
    /// The group which corresponds to the external HSM.
    pub hsm_group_id: Uuid,
}

/// The character set to use for an encrypted portion of a complex tokenization data type.
/// Characters should be specified as a list of pairs, where each pair [a, b] represents the
/// range of characters from a to b, with both bounds being inclusive. A single character can
/// be specified as [c, c].
///
/// Normally, each character is assigned a numeric value for FF1. The first character is
/// assigned a value of 0, and subsequent characters are assigned values of 1, 2, and so on,
/// up to the size of the character set. Note that the order of the ranges matters; characters
/// appearing in later ranges are assigned higher numerical values compared to earlier
/// characters. For instance, in the character set [['a', 'z'], ['0', '9']], the digits '0' to
/// '9' are assigned values from 26 to 35, since they are listed after the 'a' to 'z' range.
///
/// In any case, ranges should not overlap with each other, and should not contain surrogate
/// codepoints.
pub type FpeCharSet = Vec<[char; 2]>;

/// Structure of a compound portion of a complex tokenization data type, itself composed of
/// smaller parts.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeCompoundPart {
    /// Represents an OR of multiple structures.
    Or {
        /// The actual subparts that make up this compound part.
        or: Vec<FpeDataPart>,
        /// Additional constraints that the token type must satisfy.
        #[serde(default)]
        constraints: Option<FpeConstraints>,
        /// Whether the entire OR should be preserved as-is (i.e., not tokenized). If this is
        /// set, any descendant subparts cannot contain any preserve-related fields set.
        #[serde(default)]
        preserve: Option<bool>,
        /// Whether the entire OR should be masked when doing masked decryption. If this is set,
        /// any descendant subparts cannot contain any mask-related fields set.
        #[serde(default)]
        mask: Option<bool>,
        /// The minimum allowed length for this part (in chars).
        #[serde(default)]
        min_length: Option<u32>,
        /// The maximum allowed length for this part (in chars).
        #[serde(default)]
        max_length: Option<u32>,
    },
    /// Represents a concatenation of multiple structures (in a particular order).
    Concat {
        /// The actual subparts that make up this compound part, in order.
        concat: Vec<FpeDataPart>,
        /// Additional constraints that the token type must satisfy.
        #[serde(default)]
        constraints: Option<FpeConstraints>,
        /// Whether the entire concat should be preserved as-is (i.e., not tokenized). If this is
        /// set, any descendant subparts cannot contain any preserve-related fields set.
        #[serde(default)]
        preserve: Option<bool>,
        /// Whether the entire concat should be masked when doing masked decryption. If this is
        /// set, any descendant subparts cannot contain any mask-related fields set.
        #[serde(default)]
        mask: Option<bool>,
        /// The minimum allowed length for this part (in chars).
        #[serde(default)]
        min_length: Option<u32>,
        /// The maximum allowed length for this part (in chars).
        #[serde(default)]
        max_length: Option<u32>,
    },
    /// Indicates a part that is possibly repeated multiple times.
    Multiple {
        /// The subpart that may be repeated.
        multiple: Box<FpeDataPart>,
        /// The minimum number of times the subpart can be repeated.
        min_repetitions: Option<usize>,
        /// The maximum number of times the subpart can be repeated.
        max_repetitions: Option<usize>,
        /// Additional constraints that the token type must satisfy.
        #[serde(default)]
        constraints: Option<FpeConstraints>,
        /// Whether the entire Multiple should be preserved as-is (i.e., not tokenized). If this
        /// is set, the `multiple` subpart and its descendants cannot contain any preserve-related
        /// fields set.
        #[serde(default)]
        preserve: Option<bool>,
        /// Whether the entire Multiple should be masked when doing masked decryption. If this is
        /// set, the `multiple` subpart and its descendants cannot contain any mask-related fields
        /// set.
        #[serde(default)]
        mask: Option<bool>,
        /// The minimum allowed length for this part (in chars).
        #[serde(default)]
        min_length: Option<u32>,
        /// The maximum allowed length for this part (in chars).
        #[serde(default)]
        max_length: Option<u32>,
    },
}

/// Constraints on a portion of a complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct FpeConstraints {
    /// Whether the token part should satisfy the Luhn checksum. It is an error to apply this
    /// constraint to non-numeric parts, or for an encrypted part to be under more than one
    /// Luhn check constraint. Also, if an encrypted part has a Luhn check constraint applied
    /// to it and may contain at least one digit that is not preserved, it must not specify
    /// any other constraints.
    #[serde(default)]
    pub luhn_check: Option<bool>,
    /// Number that the token part should be greater than. This constraint can only be
    /// specified on (non-compound) numeric encrypted parts guaranteed to preserve either
    /// everything or nothing at all.
    #[serde(default)]
    pub num_gt: Option<usize>,
    /// Number that the token part should be smaller than. This constraint can only be
    /// specified on (non-compound) numeric encrypted parts guaranteed to preserve either
    /// everything or nothing at all.
    #[serde(default)]
    pub num_lt: Option<usize>,
    /// Numbers that the token part should not be equal to. It is an error to apply this
    /// constraint to non-numeric parts.
    #[serde(default)]
    pub num_ne: Option<Vec<usize>>,
    /// Specifies that this portion is supposed to represent a date, or part of one. If used,
    /// no other constraints can be specified on this part.
    #[serde(default)]
    pub date: Option<FpeDateConstraint>,
    /// The subparts to apply the constaints to. If not specified, the constraints will be
    /// applied to all subparts (recursively).
    pub applies_to: FpeConstraintsApplicability,
}

/// A structure indicating which subparts to which to apply a set of constraints.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeConstraintsApplicability {
    /// Indicates that the constraints apply to the entire part (i.e., all of its subparts),
    /// including any descendants. This is the default value for this enum and the only option
    /// available for FpeEncryptedPart, literal, and OR subparts.
    Simple(All),
    /// An object representing the individual subparts that the constraints should apply to. This
    /// is a BTreeMap where for each key-value pair, the key represents the "index" of the subpart
    /// (with the first subpart having index 0), and the value is an FpeConstraintsApplicability
    /// instance. Note that a Multiple part only allows for one possible key-value pair, since it
    /// only contains one subpart.
    ///
    /// This cannot be used with OR parts; instead, specify constraints individually on each
    /// relevant subpart.
    BySubparts(HashMap<FpeSubpartIndex, FpeConstraintsApplicability>),
}

/// Structure for specifying (part of) a complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeDataPart {
    Encrypted(FpeEncryptedPart),
    Literal {
        /// The list of possible strings that make up this literal portion of the token.
        literal: Vec<String>,
    },
    Compound(FpeCompoundPart),
}

/// A structure for specifying a token part representing a date that occurs after a specified date
/// and/or occurs before a specified date. Depending on the subparts that make up the date, one of
/// the three options is used.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum FpeDate {
    /// Represents a date that consists of a Month subpart, a Day subpart, and a Year subpart. The
    /// Year part is allowed to be preserved, and the Day and Month parts are allowed to be
    /// preserved together. (The Day part cannot be preserved if the Month part is not, and vice
    /// versa.)
    #[serde(rename = "dmy_date")]
    DayMonthYear {
        #[serde(default)]
        before: Option<FpeDayMonthYearDate>,
        #[serde(default)]
        after: Option<FpeDayMonthYearDate>,
    },
    /// Represents a date that consists of a Month subpart and a Day subpart. It is an error to
    /// preserve only the Month part or the Day part.
    #[serde(rename = "month_day_date")]
    MonthDay {
        #[serde(default)]
        before: Option<FpeDayMonthDate>,
        #[serde(default)]
        after: Option<FpeDayMonthDate>,
    },
    /// Represents a date that consists of a Month subpart and a Year subpart. The Year part is
    /// allowed to be preserved; however, the Month part cannot be preserved by itself.
    #[serde(rename = "month_year_date")]
    MonthYear {
        #[serde(default)]
        before: Option<FpeMonthYearDate>,
        #[serde(default)]
        after: Option<FpeMonthYearDate>,
    },
}

/// Possible date-related constraint types for a portion of a complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeDateConstraint {
    /// Used to indicate that a token part represents a date, which should occur before and/or
    /// after any specified bounds. The part should be a concatenation that contains either
    /// - a Day part and a Month part
    /// - a Month part and a Year part
    /// - a Day part, a Month part, and a Year part
    /// (with this constraint applying to those subparts). Each of the three choices above
    /// corresponds to a particular FpeDate variant; using the wrong variant is an error.
    /// Furthermore, the individual Month, Day, and/or Year parts that comprise the date cannot
    /// appear under Or or Multiple compound part descendants of the overall Date part (i.e.,
    /// when applying the Date constraint, the "paths" from the Date part to the Month, Day,
    /// and/or Year parts can only "go through" concatenations, and not "through" Or or Multiple
    /// parts). Those parts also have additional restrictions on how they may be preserved; the
    /// exact rules depend on the FpeDate variant.
    ///
    /// It is an error to "share" Day, Month, or Year parts across multiple dates.
    Date(FpeDate),
    /// Used to indicate that a token part represents a month, day, or year (either as part of a
    /// date, or independently). The part should be a numeric encrypted part that is guaranteed
    /// to either preserve all of its digits or preserve none of them, and cannot be involved in
    /// any Luhn-check constraints.
    DatePart(FpeDatePart),
}

/// Possible date-related constraint types that do not form a complete date (by themselves) for a
/// complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FpeDatePart {
    /// Used to indicate that a token part represents a month. The part should be a number from 1
    /// to 12, have its min_length field be at least 1, and have its max_length field be 2. Any
    /// leading zero should be removed (unless the part is always 2 digits long, in which case a
    /// leading zero may be needed).
    Month,
    /// Used to indicate that a token part represents a day. The part should be a number from 1 to
    /// 31, have its min_length field be at least 1, and have its max_length field be 2. Any
    /// leading zero should be removed (unless the part is always 2 digits long, in which case a
    /// leading zero may be needed). Further restrictions apply when the Day part occurs within a
    /// date; for instance, a date of 2/29/2000 is fine, but 4/31 is not.
    Day,
    /// Used to indicate that a token part represents a year, with any zero value being treated as
    /// a leap year. The part should be a two to five digit number.
    Year,
}

/// A structure for specifying a particular date consisting of a day and a month, for use in an
/// FpeDate structure.
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct FpeDayMonthDate {
    /// The month, which should be a number from 1 to 12.
    pub month: u8,
    /// The day, which should be a number from 1 to either 29, 30, or 31, depending on the month
    /// and year. Here, February is treated as having 29 days.
    pub day: u8,
}

/// A structure for specifying a particular date consisting of a day, month, and year, for use in
/// an FpeDate structure.
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct FpeDayMonthYearDate {
    /// The year, which should be a number less than 100000. Zero is treated as a leap year.
    pub year: u32,
    /// The month, which should be a number from 1 to 12.
    pub month: u8,
    /// The day, which should be a number from 1 to either 28, 29, 30, or 31, depending on the
    /// month and year.
    pub day: u8,
}

/// Structure of a tokenized portion of a complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct FpeEncryptedPart {
    /// The minimum allowed length for this part (in chars).
    pub min_length: u32,
    /// The maximum allowed length for this part (in chars).
    pub max_length: u32,
    /// The character set to use for this part.
    pub char_set: FpeCharSet,
    /// Additional constraints that the token type must satisfy.
    #[serde(default)]
    pub constraints: Option<FpeConstraints>,
    /// The characters to be preserved while encrypting or decrypting.
    #[serde(default)]
    pub preserve: Option<FpePreserveMask>,
    /// The characters to be masked while performing masked decryption.
    #[serde(default)]
    pub mask: Option<FpePreserveMask>,
}

/// A structure for specifying a particular date consisting of a month and a year, for use in an
/// FpeDate structure.
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct FpeMonthYearDate {
    /// The year, which should be a number less than 100000. Zero is treated as a leap year.
    pub year: u32,
    /// The month, which should be a number from 1 to 12.
    pub month: u8,
}

/// FPE-specific options.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeOptions {
    /// For specifying basic tokens
    Basic(FpeOptionsBasic),
    Advanced {
        /// The structure of the data type.
        format: FpeDataPart,
        /// The user-friendly name for the data type that represents the input data.
        description: Option<String>,
    },
}

/// Basic FPE-specific options.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct FpeOptionsBasic {
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

/// A structure indicating which indices in an encrypted part to mask or preserve.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpePreserveMask {
    /// Indicates that the entire encrypted part is to be preserved or masked.
    Entire(All),
    /// Indicates that only certain characters are to be preserved or masked. Indices are
    /// Python-like; i.e., negative indices index from the back of the token portion, with
    /// index -1 being the end of the array. (Indicating that nothing should be preserved
    /// or masked can be done via an empty list, which is the default value for this enum.)
    ByChars(Vec<isize>),
}

/// An index for listing subparts of a compound part to which certain constraints are to be applied.
/// For Concat parts, this is the zero-based index of the subpart in the `concat` field, and for
/// Multiple parts, this is always 0 (due to a Multiple having only one subpart).
pub type FpeSubpartIndex = usize;

/// An access reason provided by Google when making EKMS API calls.
#[derive(Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GoogleAccessReason {
    ReasonUnspecified = 0,
    CustomerInitiatedSupport = 1,
    GoogleInitiatedService = 2,
    ThirdPartyDataRequest = 3,
    GoogleInitiatedReview = 4,
    CustomerInitiatedAccess = 5,
    GoogleInitiatedSystemOperation = 6,
    ReasonNotExpected = 7,
    ModifiedCustomerInitiatedAccess = 8,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct GoogleAccessReasonPolicy {
    pub allow: HashSet<GoogleAccessReason>,
    pub allow_missing_reason: bool,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct HistoryItem {
    pub id: Uuid,
    pub state: HistoryItemState,
    pub created_at: Time,
    pub expiry: Time,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct HistoryItemState {
    pub activation_date: Option<Time>,
    #[serde(default)]
    pub activation_undo_window: Option<Secs>,
    pub revocation_reason: Option<RevocationReason>,
    pub compromise_date: Option<Time>,
    pub deactivation_date: Option<Time>,
    #[serde(default)]
    pub deactivation_undo_window: Option<Secs>,
    pub destruction_date: Option<Time>,
    pub deletion_date: Option<Time>,
    pub state: SobjectState,
    pub key_ops: KeyOperations,
    pub public_only: bool,
    pub has_key: bool,
    pub rotation_policy: Option<RotationPolicy>,
    #[serde(default)]
    pub group_id: Option<Uuid>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct HmacOptions {
    pub minimum_key_length: Option<u32>,
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

#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct KeyHistoryPolicy {
    pub undo_time_window: Secs,
}

/// Linked security objects.
#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyLinks {
    #[serde(default)]
    pub replacement: Option<Uuid>,
    #[serde(default)]
    pub replaced: Option<Uuid>,
    #[serde(default)]
    pub copied_from: Option<Uuid>,
    #[serde(default)]
    pub copied_to: Option<Vec<Uuid>>,
    #[serde(default)]
    pub subkeys: Vec<Uuid>,
    #[serde(default)]
    pub parent: Option<Uuid>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct KeyMetadataPolicy {
    /// Applies to all objects.
    pub base: MetadataPolicyItem,
    /// Each entry in this map fully overrides `base` for a particular object type.
    pub for_obj_type: HashMap<ObjectType, MetadataPolicyItem>,
    /// What to do with legacy objects that are not compliant with this policy.
    /// Note that objects are not allowed to be created/updated if the result is
    /// not compliant with the policy. Non-compliant legacy objects can only be
    /// updated to comply with the policy (e.g. by adding missing required metadata).
    pub legacy_objects: LegacyKeyPolicy,
}

/// Operations allowed to be performed on a given key.
pub use self::key_operations::KeyOperations;
pub mod key_operations {
    bitflags_set! {
        pub struct KeyOperations: u64 {
            ///  If this is set, the key can be used to for signing.
            const SIGN = 0x0000000000000001;
            ///  If this is set, the key can used for verifying a signature.
            const VERIFY = 0x0000000000000002;
            ///  If this is set, the key can be used for encryption.
            const ENCRYPT = 0x0000000000000004;
            ///  If this is set, the key can be used for decryption.
            const DECRYPT = 0x0000000000000008;
            ///  If this is set, the key can be used wrapping other keys.
            ///  The key being wrapped must have the EXPORT operation enabled.
            const WRAPKEY = 0x0000000000000010;
            ///  If this is set, the key can be used to unwrap a wrapped key.
            const UNWRAPKEY = 0x0000000000000020;
            ///  If this is set, the key can be used to derive another key.
            const DERIVEKEY = 0x0000000000000040;
            ///  If this is set, the key can be used to compute a cryptographic
            ///  Message Authentication Code (MAC) on a message.
            const MACGENERATE = 0x0000000000000080;
            ///  If they is set, the key can be used to verify a MAC.
            const MACVERIFY = 0x0000000000000100;
            ///  If this is set, the value of the key can be retrieved
            ///  with an authenticated request. This shouldn't be set unless
            ///  required. It is more secure to keep the key's value inside DSM only.
            const EXPORT = 0x0000000000000200;
            ///  Without this operation, management operations like delete, destroy,
            ///  rotate, activate, restore, revoke, revert, update, remove_private, etc.
            ///  cannot be performed by a crypto App.
            ///  A user with access or admin app can still perform these operations.
            ///  This option is only relevant for crypto apps.
            const APPMANAGEABLE = 0x0000000000000400;
            ///  If this is set, audit logs will not be recorded for the key.
            ///   High volume here tries to signify a key that is being used a lot
            ///   and will produce lots of logs. Setting this operation disables
            ///   audit logs for the key.
            const HIGHVOLUME = 0x0000000000000800;
            ///  If this is set, the key can be used for key agreement.
            ///  Both the private and public key should have this option enabled
            ///  to perform an agree operation.
            const AGREEKEY = 0x0000000000001000;
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct KmipClientConfig {
    #[serde(default)]
    pub ignore_unknown_key_ops_for_secrets: Option<bool>,
}

/// LDAP authorization settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LdapAuthorizationConfig {
    /// Number of seconds after which the authorization should be checked again.
    pub valid_for: u64,
    /// A map from account roles to distinguished names of LDAP groups.
    /// If a DN is specified for an account role, entities with that role
    /// must be a member of the specified LDAP group.
    pub require_role: HashMap<AccountRole, String>,
}

/// Distinguished Name (DN) resolution method. Given a user's email address, a DN resolution method
/// is used to find the user's DN in an LDAP directory.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "method")]
pub enum LdapDnResolution {
    /// Transform the user email through a pattern to derive the DN.
    Construct {
        /// For example: "example.com" => "uid={},ou=users,dc=example,dc=com".
        domain_format: HashMap<String, String>,
    },
    /// Search the directory using the LDAP `mail` attribute matching user's email.
    SearchByMail,
    /// Use email in place of DN. This method works with Active Directory if the userPrincipalName
    /// attribute is set for the user. https://docs.microsoft.com/en-us/windows/desktop/ad/naming-properties
    #[serde(rename = "upn")]
    UserPrincipalName,
}

/// Credentials used by the service to authenticate itself to an LDAP server.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LdapServiceAccount {
    pub dn: String,
    pub password: String,
}

#[derive(Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LegacyKeyPolicy {
    /// The key can be used for all purposes.
    Allowed,
    /// The key cannot be used for any crypto operations until it becomes compliant.
    Prohibited,
    /// The key can only be used for these crypto operations:
    /// - DECRYPT
    /// - VERIFY
    /// - MACVERIFY
    /// - UNWRAPKEY
    UnprotectOnly,
}

/// LMS specific options
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LmsOptions {
    /// The height of the top level tree
    pub l1_height: u32,
    /// The height of the secondary tree
    pub l2_height: u32,
    /// The hash function to use
    pub digest: Option<DigestAlgorithm>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MetadataDurationConstraint {
    Forbidden {},
    Required {
        /// If specified, the value (typically a date) is restricted to be in a
        /// range expressed in terms of duration with respect to some known point
        /// in time. For example, if we specify min = 30 days and max = 180 days
        /// for `deactivation_date`, then the user must specify a deactivation date
        /// that is within 30 and 180 days of security object's creation time.
        #[serde(default)]
        allowed_values: RestrictedDuration,
    },
}

#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct MetadataPolicyItem {
    pub custom_metadata: HashMap<String, MetadataStringConstraint>,
    pub description: Option<MetadataStringConstraint>,
    /// If a restricted duration is specified, it is enforced w.r.t object creation time.
    pub deactivation_date: Option<MetadataDurationConstraint>,
    /// If a restricted duration is specified, it is enforced w.r.t object creation time.
    /// NOTE: Specifying a minimum duration for this field may not be a good
    /// idea since it would not be possible to create a key and start using it
    /// immediately in the affected group(s).
    pub activation_date: Option<MetadataDurationConstraint>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MetadataStringConstraint {
    Forbidden {},
    Required {
        /// If set to `true`, the value must have a length > 0 after trimming
        /// leading and trailing whitespace characters.
        non_empty_after_trim: bool,
        /// If not specified or empty, it will not impose any restrictions on the value.
        allowed_values: HashSet<String>,
    },
}

/// A challenge used for multi-factor authentication.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MfaChallengeResponse {
    pub u2f_challenge: String,
    pub u2f_keys: Vec<U2fRegisteredKey>,
}

/// Specifies the Mask Generating Function (MGF) to use.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Mgf {
    Mgf1 { hash: DigestAlgorithm },
}

/// MGF policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MgfPolicy {
    Mgf1 { hash: Option<DigestAlgorithm> },
}

/// OAuth scope.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OauthScope {
    App,
    OpenID,
    Email,
    Profile,
}

/// The origin of a security object - where it was created / generated.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum ObjectOrigin {
    FortanixHSM,
    Transient,
    External,
}

/// Type of security object.
#[derive(Debug, Eq, PartialEq, Copy, Hash, EnumIter, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ObjectType {
    Aes,
    Des,
    Des3,
    Rsa,
    Dsa,
    Ec,
    Opaque,
    Hmac,
    LedaBeta,
    Round5Beta,
    Secret,
    Seed,
    Lms,
    Certificate,
    Pbe,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct OpaqueOptions {}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Pkcs11ClientConfig {
    #[serde(default)]
    pub fake_rsa_x9_31_keygen_support: Option<bool>,
    #[serde(default)]
    pub signing_aes_key_as_hmac: Option<bool>,
    #[serde(default)]
    pub exact_key_ops: Option<bool>,
    #[serde(default)]
    pub prevent_duplicate_opaque_objects: Option<bool>,
    #[serde(default)]
    pub opaque_objects_are_not_certificates: Option<bool>,
    #[serde(default)]
    pub max_concurrent_requests_per_slot: Option<usize>,
}

#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Pkcs8Mode {
    PbeWithSHAAnd128BitRC4,
    PbeWithSHAAnd3KeyTripleDesCbc,
    PbeWithSHAAnd2KeyTripleDesCbc,
    Pbes2WithPBKDF2AndKeyDes,
    Pbes2WithPBKDF2AndKeyTripleDes,
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

/// Quorum approval policy.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Quorum {
    pub n: usize,
    pub members: Vec<QuorumPolicy>,
    #[serde(flatten)]
    pub config: ApprovalAuthConfig,
}

/// Approval policy.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct QuorumPolicy {
    #[serde(default)]
    pub quorum: Option<Quorum>,
    #[serde(default)]
    pub user: Option<Uuid>,
    #[serde(default)]
    pub app: Option<Uuid>,
}

#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct RestrictedDuration {
    pub min: Option<TimeSpan>,
    pub max: Option<TimeSpan>,
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

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RotationInterval {
    IntervalDays(u32),
    IntervalMonths(u32),
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct RotationPolicy {
    #[serde(flatten)]
    pub interval: Option<RotationInterval>,
    #[serde(default)]
    pub effective_at: Option<Time>,
    pub deactivate_rotated_key: bool,
}

/// Type of padding to use for RSA encryption. The use of PKCS#1 v1.5 padding is strongly
/// discouraged, because of its susceptibility to Bleichenbacher's attack. The padding specified
/// must adhere to the key's encryption policy. If not specified, the default based on the key's
/// policy will be used.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaEncryptionPadding {
    /// Optimal Asymmetric Encryption Padding (PKCS#1 v2.1).
    Oaep {
        mgf: Mgf,
    },
    /// PKCS#1 v1.5 padding.
    Pkcs1V15 {},
    RawDecrypt {},
}

/// RSA encryption padding policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaEncryptionPaddingPolicy {
    Oaep { mgf: Option<MgfPolicy> },
    Pkcs1V15 {},
    RawDecrypt {},
}

/// Constraints on RSA encryption parameters. In general, if a constraint is not specified, anything is allowed.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RsaEncryptionPolicy {
    pub padding: Option<RsaEncryptionPaddingPolicy>,
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
    #[serde(default)]
    pub minimum_key_length: Option<u32>,
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

/// RSA signature padding policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaSignaturePaddingPolicy {
    Pss { mgf: Option<MgfPolicy> },
    Pkcs1V15 {},
}

/// Constraints on RSA signature parameters. In general, if a constraint is not specified, anything is allowed.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RsaSignaturePolicy {
    pub padding: Option<RsaSignaturePaddingPolicy>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SecretOptions {}

pub type Secs = u64;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SeedOptions {
    pub cipher_mode: Option<CipherMode>,
    pub random_iv: Option<bool>,
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

/// Signature mode.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SignatureMode {
    Rsa(RsaSignaturePadding),
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Sobject {
    pub acct_id: Uuid,
    #[serde(default)]
    pub activation_date: Option<Time>,
    #[serde(default)]
    pub aes: Option<AesOptions>,
    #[serde(default)]
    pub compliant_with_policies: Option<bool>,
    #[serde(default)]
    pub compromise_date: Option<Time>,
    pub created_at: Time,
    pub creator: Principal,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    #[serde(default)]
    pub deactivation_date: Option<Time>,
    #[serde(default)]
    pub deletion_date: Option<Time>,
    #[serde(default)]
    pub des: Option<DesOptions>,
    #[serde(default)]
    pub des3: Option<Des3Options>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub destruction_date: Option<Time>,
    #[serde(default)]
    pub deterministic_signatures: Option<bool>,
    #[serde(default)]
    pub dsa: Option<DsaOptions>,
    #[serde(default)]
    pub elliptic_curve: Option<EllipticCurve>,
    pub enabled: bool,
    #[serde(default)]
    pub external: Option<ExternalSobjectInfo>,
    #[serde(default)]
    pub fpe: Option<FpeOptions>,
    /// Key Access Justifications for GCP EKM.
    /// For more details: https://cloud.google.com/cloud-provider-access-management/key-access-justifications/docs/overview
    #[serde(default)]
    pub google_access_reason_policy: Option<GoogleAccessReasonPolicy>,
    #[serde(default)]
    pub history: Option<Vec<HistoryItem>>,
    #[serde(default)]
    pub kcv: Option<String>,
    pub key_ops: KeyOperations,
    #[serde(default)]
    pub key_size: Option<u32>,
    #[serde(default)]
    pub kid: Option<Uuid>,
    pub lastused_at: Time,
    #[serde(default)]
    pub links: Option<KeyLinks>,
    #[serde(default)]
    pub lms: Option<LmsOptions>,
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
    pub rotation_policy: Option<RotationPolicy>,
    #[serde(default)]
    pub rsa: Option<RsaOptions>,
    #[serde(default)]
    pub scheduled_rotation: Option<Time>,
    #[serde(default)]
    pub seed: Option<SeedOptions>,
    #[serde(default)]
    pub state: Option<SobjectState>,
    #[serde(default)]
    pub transient_key: Option<Blob>,
    #[serde(default)]
    pub value: Option<Blob>,
    #[serde(default)]
    pub group_id: Option<Uuid>,
}

/// Uniquely identifies a persisted or transient sobject.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SobjectDescriptor {
    Kid(Uuid),
    Name(String),
    TransientKey(Blob),
    Inline { value: Blob, obj_type: ObjectType },
}

#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
pub enum SobjectState {
    PreActive,
    Active,
    Deactivated,
    Compromised,
    Destroyed,
    Deleted,
}

#[derive(Debug, Copy, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TimeSpan {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
    Days(u32),
}

/// TLS settings.
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum TlsConfig {
    Disabled,
    Opportunistic,
    Required {
        validate_hostname: bool,
        ca: CaConfig,
        client_key: Option<Blob>,
        client_cert: Option<Blob>,
    },
}

/// Request for second factor authentication with a U2f device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct U2fAuthRequest {
    pub key_handle: Blob,
    pub signature_data: Blob,
    pub client_data: Blob,
}

/// Description of a registered U2F device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct U2fRegisteredKey {
    pub key_handle: String,
    pub version: String,
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
