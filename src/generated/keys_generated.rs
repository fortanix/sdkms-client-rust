/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Request to copy a security object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopySobjectRequest {
    /// Unique identifier of the security object to be copied.
    pub key: SobjectDescriptor,
    /// Properties for the new security object.
    #[serde(flatten)]
    pub dest: SobjectRequest
}

/// Export security object by components response.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ExportComponentsResponse {
    /// Key components
    pub components: Vec<SobjectComponent>,
    /// Initialization vector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// Tag, if required by the encryption mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Blob>,
    /// KCV for the exported key calculated by encryption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_kcv: Option<String>,
    /// KCV for the exported key calculated by CMAC
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_kcv_cmac: Option<String>,
    /// Description of the exported key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>
}

/// Request to Export a security object by components
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ExportSobjectComponentsRequest {
    /// Unique identifier of the security object
    pub key: SobjectDescriptor,
    /// Details of wrapping key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_key_params: Option<WrapKeyParams>,
    /// Key holder identifier
    pub custodians: Vec<Principal>,
    /// Splitting method
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<SplittingMethod>,
    /// Description of the exported security object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>
}

/// Parameters to show sobject details.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetSobjectParams {
    /// Response data encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<SobjectEncoding>,
    /// Show destroyed security object(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_destroyed: Option<bool>,
    /// Show deleted security object(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_deleted: Option<bool>,
    /// Show value of security object(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_value: Option<bool>,
    /// Show public key of security objects(s) if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_pub_key: Option<bool>,
    /// Whether to include the effective export policy in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_effective_export_policy: Option<bool>
}

impl UrlEncode for GetSobjectParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.view {
            m.insert("view".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_destroyed {
            m.insert("show_destroyed".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_deleted {
            m.insert("show_deleted".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_value {
            m.insert("show_value".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_pub_key {
            m.insert("show_pub_key".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_effective_export_policy {
            m.insert("show_effective_export_policy".to_string(), v.to_string());
        }
    }
}

/// Request to import a security object by components.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportSobjectComponentsRequest {
    /// Properties of the imported security object
    pub key: SobjectRequest,
    /// Details of unwrapping key, if components are wrapped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unwrap_key_params: Option<UnwrapKeyParams>,
    /// Key holder identifier
    pub custodians: Vec<Principal>,
    /// Key material by parts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SobjectComponent>>,
    /// Description of the imported security object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Splitting method used to join the key components
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<SplittingMethod>,
    /// Authentication requirements for approval requests
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_config: Option<ApprovalAuthConfig>,
    /// Explicitly specify method used for calculating given KCV
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcv_method: Option<KcvMethod>
}

/// Request to retrieve a key attestation certificate for a security object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyAttestationRequest {
    /// The target security object.
    pub key: SobjectDescriptorPersisted
}

/// Key attestation response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyAttestationResponse {
    /// The DER-encoded certificate chain for the authority issuing the key
    /// attestation statement.
    pub authority_chain: Vec<ZeroizedBlob>,
    /// The key attestation statement.
    pub attestation_statement: KeyAttestationStatement
}

/// A key attestation statement
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct KeyAttestationStatement {
    /// The format of the `statement` field.
    pub format: KeyAttestationStatementFormat,
    /// The key attestation statement formatted according to `format`
    pub statement: Blob
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KeyAttestationStatementFormat {
    /// The attestation statement is formatted as a DER-encoded X.509 certificate.
    X509Certificate
}

/// Request for getting the KCV of a security object
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct KeyCheckValueRequest {
    /// Uniquely identifies a security object
    #[serde(flatten)]
    pub key: SobjectDescriptor,
    /// The method used to calculate KCV
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcv_method: Option<KcvMethod>
}

/// KCV of a key
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct KeyCheckValueResponse {
    /// UUID, only for persistent keys
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Key Checksum Value. Could be one of following two variants:
    /// - Encrypt KCV: 24-bit checksum as a 6-character case-insensitive hex string (Default)
    /// - Cmac KCV: 40-bit checksum as a 10-character case-insensitive hex string
    pub kcv: String
}

/// Request parameters for filtering and listing security objects.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListSobjectsParams {
    /// Filter security object(s) by group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Filter security object(s) by a particular creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Uuid>,
    /// Filter security object(s) by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter security object(s) by PKCS11 label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkcs11_label: Option<String>,
    /// Filter security object(s) by PKCS11 unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkcs11_id: Option<Blob>,
    /// Filter security object(s) by object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<ObjectType>,
    /// Set max security objects in returned in response (default: 1000).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Skip first n (offset) matches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Sorting method for listed security objects.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub sort: Option<SobjectSort>,
    /// Only show security objects complying with group and account policies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compliant_with_policies: Option<bool>,
    /// Filter security object(s) by custom_metadata fields.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<CustomMetadata>,
    /// Display query metadata in response, containing information on total objects
    /// and number of objects skipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_metadata: Option<bool>,
    /// Show destroyed security object(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_destroyed: Option<bool>,
    /// Show deleted security object(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_deleted: Option<bool>,
    /// Show non-sensitive key material of security object(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_value: Option<bool>,
    /// Show public key of security objects(s) if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_pub_key: Option<bool>,
    /// Show key check value for security object(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_kcv: Option<bool>,
    /// Provide custom filtering query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>
}

impl UrlEncode for ListSobjectsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.group_id {
            m.insert("group_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.creator {
            m.insert("creator".to_string(), v.to_string());
        }
        if let Some(ref v) = self.name {
            m.insert("name".to_string(), v.to_string());
        }
        if let Some(ref v) = self.pkcs11_label {
            m.insert("pkcs11_label".to_string(), v.to_string());
        }
        if let Some(ref v) = self.pkcs11_id {
            m.insert("pkcs11_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.obj_type {
            m.insert("obj_type".to_string(), v.to_string());
        }
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        if let Some(ref v) = self.offset {
            m.insert("offset".to_string(), v.to_string());
        }
        self.sort.url_encode(m);
        if let Some(ref v) = self.compliant_with_policies {
            m.insert("compliant_with_policies".to_string(), v.to_string());
        }
        self.custom_metadata.url_encode(m);
        if let Some(ref v) = self.with_metadata {
            m.insert("with_metadata".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_destroyed {
            m.insert("show_destroyed".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_deleted {
            m.insert("show_deleted".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_value {
            m.insert("show_value".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_pub_key {
            m.insert("show_pub_key".to_string(), v.to_string());
        }
        if let Some(ref v) = self.show_kcv {
            m.insert("show_kcv".to_string(), v.to_string());
        }
        if let Some(ref v) = self.filter {
            m.insert("filter".to_string(), v.to_string());
        }
    }
}

/// Request to compute digest of a key.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDigestRequest {
    /// Uniquely identifies a security object.
    pub key: SobjectDescriptor,
    /// Digest algorithm
    pub alg: DigestAlgorithm
}

/// Digest of a key.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ObjectDigestResponse {
    /// UUID, only displayed for persistent keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Digest value
    pub digest: Blob
}

/// Request to persist a transient key.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct PersistTransientKeyRequest {
    /// Intended activation date of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<Time>,
    /// Intended deactivation date of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deactivation_date: Option<Time>,
    /// Name of the persisted security object. Security object names must be unique within an account.
    pub name: String,
    /// User-defined readable description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined metadata for the persisted key stored as key-value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    /// Whether the new security object should be enabled. Disabled security objects may not perform cryptographic operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Group ID of the security group that the persisted key should belong to. The user or
    /// application creating this security object must be a member of this group. If no group is
    /// specified, the default group for the requesting application will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Intended initial state of the key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SobjectState>,
    /// Transient key to persist
    pub transient_key: Blob
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevertRequest {
    pub ids: Vec<Uuid>
}

/// Component of security object, held by a custodian.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SobjectComponent {
    /// Key component
    pub component: ZeroizedBlob,
    /// Key component KCV
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_kcv: Option<String>,
    /// Component custodian
    pub custodian: Principal
}

/// Response data encoding.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SobjectEncoding {
    /// JSON format
    Json,
    /// Value format
    Value
}

/// Request to rekey a security object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SobjectRekeyRequest {
    /// If set to true, the old key is deactivated on rekey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_rotated_key: Option<bool>,
    /// Parameters for the new security object.
    #[serde(flatten)]
    pub dest: SobjectRequest
}

/// Request to rotate a security object to an existing security object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SobjectReplaceRequest {
    /// Security object that will be replaced as part of this operation.
    pub replaced: SobjectDescriptorPersisted,
    /// New name for the replaced security object.
    pub replaced_new_name: String,
    /// Security object that will become the replacement of the security object
    /// that has to be replaced.
    pub replacement: SobjectDescriptorPersisted
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SobjectRequest {
    /// Activation date of security object in seconds since EPOCH.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<Time>,
    /// AES specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aes: Option<AesOptions>,
    /// Whether the sign operation response contains hash or data as output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_sign_hash: Option<bool>,
    /// ARIA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aria: Option<AriaOptions>,
    /// BIP32 specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bip32: Option<Bip32Options>,
    /// BLS specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bls: Option<BlsOptions>,
    /// User managed field for adding custom metadata to the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    /// Deactivation date of security object in seconds since EPOCH.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deactivation_date: Option<Time>,
    /// DES specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub des: Option<DesOptions>,
    /// DES3 specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub des3: Option<Des3Options>,
    /// Description of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optionally get deterministic signatures, if algorithm is EC or RSA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deterministic_signatures: Option<bool>,
    /// DSA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dsa: Option<DsaOptions>,
    /// ECKCDSA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eckcdsa: Option<EcKcdsaOptions>,
    /// Identifies a standard elliptic curve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elliptic_curve: Option<EllipticCurve>,
    /// Whether this security object has cryptographic operations enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// This export policy determines how exportable keys (ones with the `EXPORT` permission) may be exported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<SobjectExportPolicy>,
    /// Information specific to an external KMS. Currently, it only has AWS related information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalKmsInfo>,
    /// FPE specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fpe: Option<FpeOptions>,
    /// Key Access Justifications for GCP EKM.
    /// For more details: https://cloud.google.com/cloud-provider-access-management/key-access-justifications/docs/overview
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_access_reason_policy: Option<Removable<GoogleAccessReasonPolicy>>,
    /// KCDSA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcdsa: Option<KcdsaOptions>,
    /// Key Checksum Value of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcv: Option<String>,
    /// Operations allowed to be performed by a given key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<KeyOperations>,
    /// Key size of the security object in bits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_size: Option<u32>,
    /// Linked security objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<KeyLinks>,
    /// LMS specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lms: Option<LmsOptions>,
    /// ML-DSA specific options (beta).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mldsa_beta: Option<MlDsaBetaOptions>,
    /// ML-KEM specific options (beta).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mlkem_beta: Option<MlKemBetaOptions>,
    /// Name of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<ObjectType>,
    /// Public exponent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pub_exponent: Option<u32>,
    /// If enabled, the public key will be available publicly (without authentication)
    /// through the GetPublicKey API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_public_key: Option<PublishPublicKeyConfig>,
    /// Rotation policy of security objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_policy: Option<RotationPolicy>,
    /// RSA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsa: Option<RsaOptions>,
    /// Seed options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<SeedOptions>,
    /// Security object operational state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SobjectState>,
    /// If set to true, the security object will cease to exist after session ends.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient: Option<bool>,
    /// Security object stored as byte array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ZeroizedBlob>,
    /// UUID of the group which the security object belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>
}

/// Sorting order on listed security objects.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum SobjectSort {
    /// Security object UUID
    ByKid {
        /// Order of listing
        order: Order,
        /// Initial security object UUID
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<Uuid>
    },
    /// Security object name
    ByName {
        /// Order of listing
        order: Order,
        /// Initial security object Name
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<String>
    }
}

impl UrlEncode for SobjectSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            SobjectSort::ByKid{ ref order, ref start } => {
                m.insert("sort".to_string(), format!("kid:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
            SobjectSort::ByName{ ref order, ref start } => {
                m.insert("sort".to_string(), format!("name:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
        }
    }
}

/// Method used to split the key into multiple components.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum SplittingMethod {
    /// Logical XOR operation
    XOR
}

/// Request to unwrap a security object
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct UnwrapKeyParams {
    /// Unique identifier of the security object.
    pub key: SobjectDescriptor,
    /// Cryptographic algorithm used for unwrapping.
    pub alg: Algorithm,
    /// Block cipher mode of operation, required for symmetric algorithms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CryptMode>,
    /// Initialization vector is required for symmetric algorithms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// Authenticated data is only applicable if mode is GCM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<Blob>,
    /// Tag is required if mode is GCM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Blob>
}

/// Verify KCV of a key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyKcvRequest {
    /// Key Checksum Value. Could be one of following two variants:
    /// - Encrypt KCV: 24-bit checksum as a 6-character case-insensitive hex string (Default)
    /// - Cmac KCV: 40-bit checksum as a 10-character case-insensitive hex string
    /// Note: Cmac KCV is only available for AES or 3-key DES3 keys
    pub kcv: String,
    /// Key material
    pub value: Blob,
    /// Type of Security object
    pub obj_type: ObjectType,
    /// The method used to calculate KCV
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcv_method: Option<KcvMethod>
}

/// Key Checksum Value verification status.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyKcvResponse {
    /// Verification status
    pub verified: bool
}

/// Wrapping key parameters
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WrapKeyParams {
    /// Wrapping key
    pub key: SobjectDescriptor,
    /// Cryptographic algorithm of security object
    pub alg: Algorithm,
    /// Block cipher mode of operation, required for symmetric algorithms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CryptMode>,
    /// Initialization vector is required for symmetric algorithms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// Authenticated data is only applicable if mode is GCM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<Blob>,
    /// Tag length is required when mode is GCM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_len: Option<usize>
}

pub struct OperationActivateSobject;
#[allow(unused)]
impl Operation for OperationActivateSobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{key_id}/activate", key_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn activate_sobject(&self, key_id: &Uuid) -> Result<()> {
        self.execute::<OperationActivateSobject>(&(), (key_id,), None)
    }
}

pub struct OperationCopySobject;
#[allow(unused)]
impl Operation for OperationCopySobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = CopySobjectRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/copy".to_string()
    }
}

impl SdkmsClient {
    pub fn copy_sobject(&self, req: &CopySobjectRequest) -> Result<Sobject> {
        self.execute::<OperationCopySobject>(req, (), None)
    }
    pub fn request_approval_to_copy_sobject(
        &self, req: &CopySobjectRequest,
        description: Option<String>) -> Result<PendingApproval<OperationCopySobject>> {
        self.request_approval::<OperationCopySobject>(req, (), None, description)
    }
}

pub struct OperationCreateSobject;
#[allow(unused)]
impl Operation for OperationCreateSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys".to_string()
    }
}

impl SdkmsClient {
    pub fn create_sobject(&self, req: &SobjectRequest) -> Result<Sobject> {
        self.execute::<OperationCreateSobject>(req, (), None)
    }
}

pub struct OperationDeleteSobject;
#[allow(unused)]
impl Operation for OperationDeleteSobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{key_id}", key_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_sobject(&self, key_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteSobject>(&(), (key_id,), None)
    }
    pub fn request_approval_to_delete_sobject(
        &self, key_id: &Uuid,
        description: Option<String>) -> Result<PendingApproval<OperationDeleteSobject>> {
        self.request_approval::<OperationDeleteSobject>(&(), (key_id,), None, description)
    }
}

pub struct OperationDestroySobject;
#[allow(unused)]
impl Operation for OperationDestroySobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{key_id}/destroy", key_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn destroy_sobject(&self, key_id: &Uuid) -> Result<()> {
        self.execute::<OperationDestroySobject>(&(), (key_id,), None)
    }
    pub fn request_approval_to_destroy_sobject(
        &self, key_id: &Uuid,
        description: Option<String>) -> Result<PendingApproval<OperationDestroySobject>> {
        self.request_approval::<OperationDestroySobject>(&(), (key_id,), None, description)
    }
}

pub struct OperationDigestSobject;
#[allow(unused)]
impl Operation for OperationDigestSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = ObjectDigestRequest;
    type Output = ObjectDigestResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/digest".to_string()
    }
}

impl SdkmsClient {
    pub fn digest_sobject(&self, req: &ObjectDigestRequest) -> Result<ObjectDigestResponse> {
        self.execute::<OperationDigestSobject>(req, (), None)
    }
}

pub struct OperationExportSobject;
#[allow(unused)]
impl Operation for OperationExportSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectDescriptor;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/export".to_string()
    }
}

impl SdkmsClient {
    pub fn export_sobject(&self, req: &SobjectDescriptor) -> Result<Sobject> {
        self.execute::<OperationExportSobject>(req, (), None)
    }
    pub fn request_approval_to_export_sobject(
        &self, req: &SobjectDescriptor,
        description: Option<String>) -> Result<PendingApproval<OperationExportSobject>> {
        self.request_approval::<OperationExportSobject>(req, (), None, description)
    }
}

pub struct OperationExportSobjectComponents;
#[allow(unused)]
impl Operation for OperationExportSobjectComponents {
    type PathParams = ();
    type QueryParams = ();
    type Body = ExportSobjectComponentsRequest;
    type Output = ExportComponentsResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/components/export".to_string()
    }
}

impl SdkmsClient {
    pub fn export_sobject_components(&self, req: &ExportSobjectComponentsRequest) -> Result<ExportComponentsResponse> {
        self.execute::<OperationExportSobjectComponents>(req, (), None)
    }
    pub fn request_approval_to_export_sobject_components(
        &self, req: &ExportSobjectComponentsRequest,
        description: Option<String>) -> Result<PendingApproval<OperationExportSobjectComponents>> {
        self.request_approval::<OperationExportSobjectComponents>(req, (), None, description)
    }
}

pub struct OperationGetKcv;
#[allow(unused)]
impl Operation for OperationGetKcv {
    type PathParams = ();
    type QueryParams = ();
    type Body = KeyCheckValueRequest;
    type Output = KeyCheckValueResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/kcv".to_string()
    }
}

impl SdkmsClient {
    pub fn get_kcv(&self, req: &KeyCheckValueRequest) -> Result<KeyCheckValueResponse> {
        self.execute::<OperationGetKcv>(req, (), None)
    }
}

pub struct OperationGetKeyAttestation;
#[allow(unused)]
impl Operation for OperationGetKeyAttestation {
    type PathParams = ();
    type QueryParams = ();
    type Body = KeyAttestationRequest;
    type Output = KeyAttestationResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/key_attestation".to_string()
    }
}

impl SdkmsClient {
    pub fn get_key_attestation(&self, req: &KeyAttestationRequest) -> Result<KeyAttestationResponse> {
        self.execute::<OperationGetKeyAttestation>(req, (), None)
    }
}

pub struct OperationGetPubkey;
#[allow(unused)]
impl Operation for OperationGetPubkey {
    type PathParams = (Uuid, String,);
    type QueryParams = ();
    type Body = ();
    type Output = HashMap<String,ZeroizedBlob>;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/pubkey/{acct_id}/{name}", acct_id = p.0, name = p.1)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_pubkey(&self, acct_id: &Uuid, name: &String) -> Result<HashMap<String,ZeroizedBlob>> {
        self.execute::<OperationGetPubkey>(&(), (acct_id, name,), None)
    }
}

pub struct OperationGetSobject;
#[allow(unused)]
impl Operation for OperationGetSobject {
    type PathParams = ();
    type QueryParams = GetSobjectParams;
    type Body = SobjectDescriptor;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/info?{q}", q = q.encode())
    }
}

impl SdkmsClient {
    pub fn get_sobject(&self, query_params: Option<&GetSobjectParams>, req: &SobjectDescriptor) -> Result<Sobject> {
        self.execute::<OperationGetSobject>(req, (), query_params)
    }
}

pub struct OperationImportSobject;
#[allow(unused)]
impl Operation for OperationImportSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Put
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys".to_string()
    }
}

impl SdkmsClient {
    pub fn import_sobject(&self, req: &SobjectRequest) -> Result<Sobject> {
        self.execute::<OperationImportSobject>(req, (), None)
    }
}

pub struct OperationImportSobjectByComponents;
#[allow(unused)]
impl Operation for OperationImportSobjectByComponents {
    type PathParams = ();
    type QueryParams = ();
    type Body = ImportSobjectComponentsRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/components/import".to_string()
    }
}

impl SdkmsClient {
    pub fn import_sobject_by_components(&self, req: &ImportSobjectComponentsRequest) -> Result<Sobject> {
        self.execute::<OperationImportSobjectByComponents>(req, (), None)
    }
    pub fn request_approval_to_import_sobject_by_components(
        &self, req: &ImportSobjectComponentsRequest,
        description: Option<String>) -> Result<PendingApproval<OperationImportSobjectByComponents>> {
        self.request_approval::<OperationImportSobjectByComponents>(req, (), None, description)
    }
}

pub struct OperationListSobjects;
#[allow(unused)]
impl Operation for OperationListSobjects {
    type PathParams = ();
    type QueryParams = ListSobjectsParams;
    type Body = ();
    type Output = ListSobjectsResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_sobjects(&self, query_params: Option<&ListSobjectsParams>) -> Result<ListSobjectsResponse> {
        self.execute::<OperationListSobjects>(&(), (), query_params)
    }
}

pub struct OperationPersistTransientKey;
#[allow(unused)]
impl Operation for OperationPersistTransientKey {
    type PathParams = ();
    type QueryParams = ();
    type Body = PersistTransientKeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/persist".to_string()
    }
}

impl SdkmsClient {
    pub fn persist_transient_key(&self, req: &PersistTransientKeyRequest) -> Result<Sobject> {
        self.execute::<OperationPersistTransientKey>(req, (), None)
    }
}

pub struct OperationRemovePrivate;
#[allow(unused)]
impl Operation for OperationRemovePrivate {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{key_id}/private", key_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn remove_private(&self, key_id: &Uuid) -> Result<()> {
        self.execute::<OperationRemovePrivate>(&(), (key_id,), None)
    }
    pub fn request_approval_to_remove_private(
        &self, key_id: &Uuid,
        description: Option<String>) -> Result<PendingApproval<OperationRemovePrivate>> {
        self.request_approval::<OperationRemovePrivate>(&(), (key_id,), None, description)
    }
}

pub struct OperationReplaceSobject;
#[allow(unused)]
impl Operation for OperationReplaceSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectReplaceRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/replace".to_string()
    }
}

impl SdkmsClient {
    pub fn replace_sobject(&self, req: &SobjectReplaceRequest) -> Result<()> {
        self.execute::<OperationReplaceSobject>(req, (), None)
    }
    pub fn request_approval_to_replace_sobject(
        &self, req: &SobjectReplaceRequest,
        description: Option<String>) -> Result<PendingApproval<OperationReplaceSobject>> {
        self.request_approval::<OperationReplaceSobject>(req, (), None, description)
    }
}

pub struct OperationRevertPrevKeyOp;
#[allow(unused)]
impl Operation for OperationRevertPrevKeyOp {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = RevertRequest;
    type Output = ();

    fn method() -> Method {
        Method::Put
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{key_id}/revert", key_id = p.0)
    }
}

impl SdkmsClient {
    pub fn revert_prev_key_op(&self, key_id: &Uuid, req: &RevertRequest) -> Result<()> {
        self.execute::<OperationRevertPrevKeyOp>(req, (key_id,), None)
    }
    pub fn request_approval_to_revert_prev_key_op(
        &self, key_id: &Uuid, req: &RevertRequest,
        description: Option<String>) -> Result<PendingApproval<OperationRevertPrevKeyOp>> {
        self.request_approval::<OperationRevertPrevKeyOp>(req, (key_id,), None, description)
    }
}

pub struct OperationRevokeSobject;
#[allow(unused)]
impl Operation for OperationRevokeSobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = RevocationReason;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{key_id}/revoke", key_id = p.0)
    }
}

impl SdkmsClient {
    pub fn revoke_sobject(&self, key_id: &Uuid, req: &RevocationReason) -> Result<()> {
        self.execute::<OperationRevokeSobject>(req, (key_id,), None)
    }
    pub fn request_approval_to_revoke_sobject(
        &self, key_id: &Uuid, req: &RevocationReason,
        description: Option<String>) -> Result<PendingApproval<OperationRevokeSobject>> {
        self.request_approval::<OperationRevokeSobject>(req, (key_id,), None, description)
    }
}

pub struct OperationRotateSobject;
#[allow(unused)]
impl Operation for OperationRotateSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectRekeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/rekey".to_string()
    }
}

impl SdkmsClient {
    pub fn rotate_sobject(&self, req: &SobjectRekeyRequest) -> Result<Sobject> {
        self.execute::<OperationRotateSobject>(req, (), None)
    }
    pub fn request_approval_to_rotate_sobject(
        &self, req: &SobjectRekeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationRotateSobject>> {
        self.request_approval::<OperationRotateSobject>(req, (), None, description)
    }
}

pub struct OperationUpdateSobject;
#[allow(unused)]
impl Operation for OperationUpdateSobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = SobjectRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{key_id}", key_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_sobject(&self, key_id: &Uuid, req: &SobjectRequest) -> Result<Sobject> {
        self.execute::<OperationUpdateSobject>(req, (key_id,), None)
    }
    pub fn request_approval_to_update_sobject(
        &self, key_id: &Uuid, req: &SobjectRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdateSobject>> {
        self.request_approval::<OperationUpdateSobject>(req, (key_id,), None, description)
    }
}

pub struct OperationVerifyKcv;
#[allow(unused)]
impl Operation for OperationVerifyKcv {
    type PathParams = ();
    type QueryParams = ();
    type Body = VerifyKcvRequest;
    type Output = VerifyKcvResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/kcv/verify".to_string()
    }
}

impl SdkmsClient {
    pub fn verify_kcv(&self, req: &VerifyKcvRequest) -> Result<VerifyKcvResponse> {
        self.execute::<OperationVerifyKcv>(req, (), None)
    }
}

