/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Options to use for key agreement mechanism.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AgreeKeyMechanism {
    /// Diffie-Hellman key exchange mechanism
    DiffieHellman
}

/// Request body to perform key agreement.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgreeKeyRequest {
    /// Activation date of the agreed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<Time>,
    /// Deactivation date of the agreed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deactivation_date: Option<Time>,
    /// Identifier of the private key used for agreement
    pub private_key: SobjectDescriptor,
    /// Identifier of the public key used for agreement
    pub public_key: SobjectDescriptor,
    /// Mechanism to use for key derivation.
    pub mechanism: AgreeKeyMechanism,
    /// Name of the agreed-upon key. Key names must be unique within an account.
    /// The name is ignored for transient keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Group ID of the security group that this security object should belong to. The user or
    /// application creating this security object must be a member of this group. If no group is
    /// specified, the default group for the requesting application will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Type of key to be derived. NB. for security reasons, you shouldn't specify anything but HMAC or Secret.
    pub key_type: ObjectType,
    /// Key size in bits. If less than the output size of the algorithm, the secret's most-significant bits will be truncated.
    pub key_size: u32,
    /// Whether the agreed key should have cryptographic operations enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Description of the agreed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined metadata for this key stored as key-value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    /// Optional array of key operations to be enabled for this security object. If not
    /// provided the service will provide a default set of key operations. Note that if you
    /// provide an empty array, all key operations will be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<KeyOperations>,
    /// State of the agreed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SobjectState>,
    /// If set to true, the resulting key will be transient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient: Option<bool>,
    /// The export policy to be applied to the agreed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<SobjectExportPolicy>
}

/// Request body of each item in batch decryption
/// 
/// **Note** : Provide the key ID in the *`kid`* field. The *`key`* field within the *`request`* field should be omitted.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchDecryptRequestItem {
    /// UUID of the sobject
    pub kid: Uuid,
    /// Request body for Decryption
    pub request: DecryptRequest
}

/// Request body of each item in batch encryption
/// 
/// **Note** : Provide the key ID in the *`kid`* field. The *`key`* field within the *`request`* field should be omitted.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchEncryptRequestItem {
    /// UUID of the sobject
    pub kid: Uuid,
    /// Request body for encryption
    pub request: EncryptRequest
}

/// Request body to perform key decapsulation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecapsulateKeyRequest {
    /// Reference to the decapsulation key
    pub key: SobjectDescriptor,
    /// The encapsulated key
    pub ciphertext: Blob,
    /// Name of the resulting symmetric key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Group ID of the security group that the resulting Sobject should belong
    /// to. The application creating the Sobject must be a member of
    /// this group. If no group is specified, the default group for the
    /// requesting application will be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Type of key to be decapsulated
    pub key_type: ObjectType,
    /// Key size in bits of the resulting symmetric key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<u32>,
    /// Key operations of the decapsulated key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<KeyOperations>,
    /// If set to true, the decapsulated key will be transient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient: Option<bool>
}

/// Request body to finalize a multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptFinalRequest {
    /// Identifier of the sobject used for finalizing multi-part decryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Current state of the encrypted cipher
    pub state: Blob,
    /// Tag value of the encrypted cipher. Only applicable when using GCM mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Blob>
}

/// Final response body of a multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptFinalResponse {
    /// Decrypted bytes
    pub plain: ZeroizedBlob
}

/// Request body to initialize multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptInitRequest {
    /// Identifier of the sobject used for initializing multi-part decryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Algorithm used for multi-part decryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<Algorithm>,
    /// Mode of multi-part decryption. Required for symmetric algorithms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CipherMode>,
    /// Initialization vector. Required for symmetric algorithms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// Authenticated data. Only applicable when using GCM mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<Blob>
}

/// Response body for initializing multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptInitResponse {
    /// The key id is returned for non-transient keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Opaque data, not to be interpreted or modified by the client and must be provided with next request.
    pub state: Blob
}

/// Request to decrypt data.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptRequest {
    /// Reference to the sobject to use for decryption. This can be a key
    /// ID, key name, or a transient key blob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Decryption algorithm to use. If specified, this must be compatible
    /// with the key type; for example, an RSA key cannot be used with AES.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<Algorithm>,
    /// Ciphertext bytes to be decrypted.
    ///
    /// Note that when performing format-preserving decryption (i.e.,
    /// detokenization), the ciphertext should be encoded as UTF-8 bytes.
    pub cipher: Blob,
    /// Decryption mode to use. This is required for symmetric decryption.
    /// For RSA decryption, the mode can be used to optionally specify the
    /// padding to use. For all other algorithms, this field should not be
    /// specified.
    ///
    /// If not specified for RSA decryption, the backend will pick a default
    /// padding mode based on the key's padding policy (which may or may not
    /// satisfy any group or account-level cryptographic policies).
    /// Specifically,
    /// - The backend will try to use the first allowed decryption padding
    ///   policy present in the sobject's encryption padding policy.
    /// - If the "first allowed padding" is a wildcard policy (i.e., `{}`),
    ///   the backend will use OAEP with SHA-256 as the MGF1 hash function.
    /// - In FIPS mode, if the "first allowed padding" is PKCS #1 v1.5 padding
    ///   or OAEP with SHA-1 as the MGF1 hash function, then the backend will
    ///   attempt to use a different padding mode, if allowed by the sobject.
    /// - Once chosen, the padding mode will be validated against any group or
    ///   account-level cryptographic policies. If this fails, the operation
    ///   will error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CryptMode>,
    /// The initialization vector to use, required for modes that take IVs
    /// (and irrelevant otherwise).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// The authenticated data to use. This is only applicable when using
    /// authenticated decryption modes (like GCM or CCM).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<Blob>,
    /// The authentication tag, relevant for authenticated encryption modes
    /// (i.e., GCM or CCM), and otherwise irrelevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Blob>,
    /// Whether to returned a masked result when detokenizing (i.e., when
    /// decrypting using the FF1/FPE mode). Defaults to false.
    ///
    /// This field is only useful if the app has the `DECRYPT` permission.
    /// In such situations, when this field is `true`, decryption returns
    /// masked output. However, with the `MASKDECRYPT` permission, this field
    /// is ignored and detokenization will always return the masked output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub masked: Option<bool>,
    /// The optional label to use. Currently this field only serves as the
    /// rsa_oaep_label when the decryption algorithm is RSA and the mode is
    /// Oaep. For other modes, providing this field causes a bad request error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Blob>
}

/// Response of a decryption request.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptResponse {
    /// The ID of the key used for decryption. Returned for non-transient keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Decrypted plaintext bytes.
    ///
    /// Note that when performing format-preserving decryption (i.e.,
    /// detokenization), the plaintext is encoded as UTF-8 bytes.
    pub plain: ZeroizedBlob
}

/// Request body for multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptUpdateRequest {
    /// Identifier of the sobject used for multi-part decryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Encrypted bytes
    pub cipher: Blob,
    /// Current state of the encrypted cipher
    pub state: Blob
}

/// Response body of multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptUpdateResponse {
    /// Decrypted bytes
    pub plain: ZeroizedBlob,
    /// Current state of the multi part decrypted object.
    /// Opaque data, not to be interpreted or modified by the client and must be provided with next request.
    pub state: Blob
}

/// Mechanism to be used when deriving a new key from an existing key.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DeriveKeyMechanism {
    EncryptData (
        EncryptRequest
    ),
    Bip32MasterKey {
        network: Bip32Network
    },
    Bip32HardenedChild {
        index: u32
    },
    Hkdf {
        hash_alg: DigestAlgorithm,
        #[serde(skip_serializing_if = "Option::is_none")]
        info: Option<Blob>,
        #[serde(skip_serializing_if = "Option::is_none")]
        salt: Option<Blob>
    }
}

/// Request body to derive a key.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeriveKeyRequest {
    /// Activation date of the derived key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<Time>,
    /// Deactivation date of the derived key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deactivation_date: Option<Time>,
    /// Identifier of the sobject from which new key will be derived
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Name of the derived key. Key names must be unique within an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Group ID of the security group that this security object should belong to. The user or
    /// application creating this security object must be a member of this group. If no group is
    /// specified, the default group for the requesting application will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Type of key to be derived.
    pub key_type: ObjectType,
    /// Key size of the derived key in bits.
    pub key_size: u32,
    /// Mechanism to use for key derivation.
    pub mechanism: DeriveKeyMechanism,
    /// Whether the derived key should have cryptographic operations enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Description for derived key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined metadata for this key stored as key-value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    /// Optional array of key operations to be enabled for this security object. If not
    /// provided the service will provide a default set of key operations. Note that if you
    /// provide an empty array, all key operations will be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<KeyOperations>,
    /// State of the derived key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SobjectState>,
    /// If set to true, the derived key will be transient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient: Option<bool>,
    /// Export policy of the derived key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<SobjectExportPolicy>
}

/// Request to compute the hash of arbitrary data.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DigestRequest {
    /// Hash Algorithm to compute digest
    pub alg: DigestAlgorithm,
    /// Raw binary data
    pub data: Blob
}

/// Response body of a hash operation.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DigestResponse {
    /// Hashed binary output
    pub digest: Blob
}

/// Request body to perform key encapsulation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncapsulateKeyRequest {
    /// Reference to the encapsulation key
    pub key: SobjectDescriptor,
    /// Name of the resulting symmetric key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Group ID of the security group that the resulting Sobject should belong
    /// to. The application creating this Sobject must be a member of
    /// this group. If no group is specified, the default group for the
    /// requesting application will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Type of the resulting key
    pub key_type: ObjectType,
    /// Key size in bits of the resulting key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<u32>,
    /// Key operations of the resulting key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<KeyOperations>,
    /// If set to true, the resulting key will be transient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient: Option<bool>
}

/// Result of an encapsulation request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncapsulateKeyResponse {
    /// Resulting (newly created) symmetric key
    pub key: Sobject,
    /// The encapsulated key
    pub ciphertext: Blob
}

/// Request body to finalize a multi-part encryption.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptFinalRequest {
    /// Reference to the sobject used for finalizing multi-part encryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Last state of the encrypted cipher
    pub state: Blob,
    /// Size of authentication tag.
    /// Tag length is only applicable when using GCM mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_len: Option<usize>
}

/// Final response body of a multi-part encryption.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptFinalResponse {
    /// Final encrypted bytes
    pub cipher: Blob,
    /// Tag is only returned for symmetric encryption with GCM mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Blob>
}

/// Request body to initialize multi-part encryption.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptInitRequest {
    /// Reference to the sobject used for initializing multi-part encryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Algorithm to be used for multipart encryption
    pub alg: Algorithm,
    /// Cipher mode of operation for symmetric multi-part encryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CipherMode>,
    /// Initialization vector
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// Authenticated data, required for AEAD algorithms
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<Blob>
}

/// Response body of initializing multi-part encryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct EncryptInitResponse {
    /// Key id is returned for non-transient keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Initialization vector. Only returned for symmetric encryption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// Current state of the encrypted cipher.
    /// Opaque data, not to be interpreted or modified by the client and must be provided with next request.
    pub state: Blob
}

/// Request to encrypt data.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct EncryptRequest {
    /// Reference to the sobject to use for encryption. This can be a key
    /// ID, key name, or a transient key blob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Encryption algorithm to use. The algorithm must be compatible with
    /// the key type; for example, an RSA key cannot be used with AES.
    pub alg: Algorithm,
    /// Plaintext bytes to be encrypted.
    ///
    /// Note that when performing format-preserving encryption (i.e.,
    /// tokenization), the plaintext should be encoded as UTF-8 bytes.
    pub plain: ZeroizedBlob,
    /// Encryption mode to use. This is required for symmetric encryption.
    /// For RSA encryption, the mode can be used to optionally specify the
    /// padding to use. For all other algorithms, this field should not be
    /// specified.
    ///
    /// If not specified for RSA encryption, the backend will pick a default
    /// padding mode based on the key's padding policy (which may or may not
    /// satisfy any group or account-level cryptographic policies).
    /// Specifically,
    /// - The backend will try to use the first allowed encryption padding
    ///   policy present in the sobject's encryption padding policy.
    /// - If the "first allowed padding" is a wildcard policy (i.e., `{}`),
    ///   the backend will use OAEP with SHA-256 as the MGF1 hash function.
    /// - In FIPS mode, if the "first allowed padding" is PKCS #1 v1.5 padding
    ///   or OAEP with SHA-1 as the MGF1 hash function, then the backend will
    ///   attempt to use a different padding mode, if allowed by the sobject.
    /// - Once chosen, the padding mode will be validated against any group or
    ///   account-level cryptographic policies. If this fails, the operation
    ///   will error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CryptMode>,
    /// The initialization vector to use. This is only applicable to modes
    /// that take IVs, and will be randomly generated if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// The authenticated data to use. This is only applicable when using
    /// authenticated encryption modes (like GCM or CCM).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<Blob>,
    /// The length of the authentication tag, in bits, for authenticated
    /// encryption modes (i.e., GCM or CCM). For other modes, this field
    /// is irrelevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_len: Option<usize>,
    /// The optional label to use. Currently this field only serves as the
    /// rsa_oaep_label when the encryption algorithm is RSA and the mode is
    /// Oaep. For other modes, providing this field causes a bad request error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Blob>
}

/// Response of an encryption request.
#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct EncryptResponse {
    /// The ID of the key used for encryption. Returned for non-transient keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Encrypted ciphertext bytes.
    ///
    /// Note that when performing format-preserving encryption (i.e.,
    /// tokenization), the ciphertext is encoded as UTF-8 bytes.
    pub cipher: Blob,
    /// The initialization vector used during encryption. This is only
    /// applicable for certain symmetric encryption modes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// When using the GCM or CCM modes, the tag is returned from
    /// authenticated encryption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Blob>
}

/// Request body for continuing multi part encryption
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptUpdateRequest {
    /// Reference to the sobject used for continuing multi part encryption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Data bytes to be encrypted
    pub plain: ZeroizedBlob,
    /// Last state of the encrypted cipher
    pub state: Blob
}

/// Response body of multi-part encryption.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptUpdateResponse {
    /// Encrypted bytes object from multi-part flow
    pub cipher: Blob,
    /// Current state of the encrypted cipher
    /// Opaque data, not to be interpreted or modified by the client and must be provided with next request.
    pub state: Blob
}

/// Key Format
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum KeyFormat {
    Default,
    Pkcs8
}

/// Request to compute a MAC.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MacRequest {
    /// Reference to the sobject with which to compute a MAC.
    /// This can be a key ID, key name, or a transient key blob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// The hash algorithm to use when computing an HMAC. Irrelevant
    /// if computing a CMAC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<DigestAlgorithm>,
    /// The data for which to generate a MAC
    pub data: Blob
}

/// Response of a MAC computation request.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MacResponse {
    /// The ID of the key used to compute the MAC. Returned for
    /// non-transient keys
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// MAC generated for the input data
    pub mac: Blob
}

/// Type of padding to use for RSA signatures. The padding specified must adhere to the key's
/// signature policy. If not specified, the default based on the key's policy will be used.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaSignaturePadding {
    /// Probabilistic Signature Scheme (PKCS#1 v2.1).
    Pss {
        mgf: Mgf
    },
    /// PKCS#1 v1.5 padding.
    Pkcs1V15 {

    }
}

/// Request to sign data (or hashed data) using an asymmetric key.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignRequest {
    /// Reference to the sobject to use for signing. This can be a key ID,
    /// key name, or a transient key blob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Hashing algorithm to use for signing. Specifically, this refers to
    /// the algorithm used to hash the data.
    ///
    /// Note that certain algorithms (SHA-1, SSL3, Blake2, Streebog) are
    /// disallowed in FIPS mode.
    pub hash_alg: DigestAlgorithm,
    /// Hashed data to be signed. Either `hash` or `data` should be specified;
    /// it is an error to specify both or none.
    /// Hash should be base64 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<Blob>,
    /// Data to be signed. Either `hash` or `data` should be specified; it is
    /// an error to specify both or none.
    /// Data should be base64 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Blob>,
    /// Signature mechanism to use. This is only relevant for RSA signatures;
    /// it should not be specified for all other signature algorithms.
    ///
    /// Additionally, in FIPS mode, if using PSS, the MGF1 hash function must
    /// be the same as the data hashing function (under the `alg`) field.
    ///
    /// If not specified for RSA signing, the backend will pick a default
    /// padding mode based on the key's padding policy (which may or may not
    /// satisfy any group or account-level cryptographic policies).
    /// Specifically,
    /// - The backend will try to use the first allowed signing padding policy
    ///   present in the sobject's signature padding policy.
    /// - If the "first allowed padding" is a wildcard policy (i.e., `{}`),
    ///   the backend will use PKCS #1 v1.5 padding. If the policy is instead
    ///   a wildcard PSS policy (where no MGF1 hash function is specified),
    ///   the backend will use PSS with SHA-256 as the MGF1 hash function.
    /// - In FIPS mode, the backend will try to avoid PSS padding modes where
    ///   the MGF1 hash function does not match the data hashing function.
    /// Once chosen, the padding mode will be validated against any group or
    /// account-level cryptographic policies. If this fails, the operation
    /// will error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<SignatureMode>,
    /// Whether signatures should be deterministic. Defaults to false. If
    /// specified, the value must be compatible with the key's settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deterministic_signature: Option<bool>
}

/// Response of a signing request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignResponse {
    /// The ID of the key used for signing. Returned for non-transient keys
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Signed data
    pub signature: ZeroizedBlob
}

/// Signature mechanism
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SignatureMode {
    /// RSA Signature mechanism with padding
    Rsa (
        RsaSignaturePadding
    )
}

/// Options for mechanism to be used when transforming a key
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TransformKeyMechanism {
    Bip32WeakChild {
        /// The index of a weak child is an integer between 0 and 2**31 - 1.
        index: u32
    }
}

/// Request body to transform a key.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformKeyRequest {
    /// Activation date of the transformed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<Time>,
    /// Deactivation date of the transformed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deactivation_date: Option<Time>,
    /// Identifier of the sobject which will be transformed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Name of the transformed key. Key names must be unique within an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Group ID of the group that this security object should belong to. The user or
    /// application creating this security object must be a member of this group. If no group is
    /// specified, the default group for the requesting application will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Type of the transformed key.
    pub key_type: ObjectType,
    /// Mechanism to use for key transformation.
    pub mechanism: TransformKeyMechanism,
    /// Whether the transformed key should have cryptographic operations enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Description of the transformed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined metadata for this key stored as key-value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    /// Optional array of key operations to be enabled for this security object. If not
    /// provided the service will provide a default set of key operations. Note that if you
    /// provide an empty array, all key operations will be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<KeyOperations>,
    /// State of the transformed key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SobjectState>,
    /// If set to true, the transformed key will be transient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient: Option<bool>,
    /// The export policy to be applied to the newly created sobject
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<SobjectExportPolicy>
}

/// Request to unwrap an sobject with another sobject.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct UnwrapKeyRequest {
    /// Reference to the unwrapping key. This can be a key ID, key name,
    /// or a transient key blob. It may also be a password (if unwrapping
    /// PKCS #8 blobs).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Algorithm to use for key unwrapping. The algorithm must be
    /// compatible with the key type; for example, an RSA key cannot
    /// be used with AES.
    pub alg: Algorithm,
    /// Object type of the key being unwrapped
    pub obj_type: ObjectType,
    /// RSA-specific options for the key being unwrapped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsa: Option<RsaOptions>,
    /// A security object previously wrapped with another key
    pub wrapped_key: Blob,
    /// Decryption mode to use. This is required for unwrapping via
    /// symmetric decryption. For RSA-based wrapping, the mode can be used
    /// to optionally specify the padding to use. For all other algorithms,
    /// this field should not be specified.
    ///
    /// If not specified for RSA decryption, the backend will pick a default
    /// padding mode based on the key's padding policy (which may or may not
    /// satisfy any group or account-level cryptographic policies).
    /// Specifically,
    /// - The backend will try to use the first allowed decryption padding
    ///   policy present in the sobject's encryption padding policy.
    /// - If the "first allowed padding" is a wildcard policy (i.e., `{}`),
    ///   the backend will use OAEP with SHA-256 as the MGF1 hash function.
    /// - In FIPS mode, if the "first allowed padding" is PKCS #1 v1.5 padding
    ///   or OAEP with SHA-1 as the MGF1 hash function, then the backend will
    ///   attempt to use a different padding mode, if allowed by the sobject.
    /// - Once chosen, the padding mode will be validated against any group or
    ///   account-level cryptographic policies. If this fails, the operation
    ///   will error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CryptMode>,
    /// The initialization vector to use, required for modes that take IVs
    /// (and irrelevant otherwise).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// The authenticated data to use. This is only applicable when using
    /// authenticated decryption modes (i.e., GCM or CCM).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<Blob>,
    /// The authentication tag, relevant for authenticated encryption modes
    /// (i.e., GCM or CCM), and otherwise irrelevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Blob>,
    /// Name to be given to the resulting security object, if persisted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ID of the group that the unwrapped security object should belong to
    /// (if persisted). The user or application creating this security object
    /// must be a member of this group. If no group is specified, and the
    /// requester is an app, the app's default group will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Whether the unwrapped key should have cryptographic operations enabled.
    /// Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// User-defined description of the unwrapped key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined metadata for the resulting key, stored as key-value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    /// Optional array of key operations to be enabled for the resulting security
    /// object. If not provided, DSM will provide a default set of key operations.
    /// Note that an empty array will result in all key operations being disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<KeyOperations>,
    /// Whether the unwrapped key should be a transient key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient: Option<bool>,
    /// Checksum value of the wrapped key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcv: Option<String>,
    /// Explicitly specify method used for calculating given KCV
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcv_method: Option<KcvMethod>,
    /// The export policy to be applied to the unwrapped key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<SobjectExportPolicy>
}

/// Request to verify a MAC.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyMacRequest {
    /// Reference to the sobject with which to verify a MAC.
    /// This can be a key ID, key name, or a transient key blob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// The hash algorithm used when computing the HMAC. Irrelevant
    /// if verifying a CMAC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<DigestAlgorithm>,
    /// The data over which the MAC needs to be verified
    pub data: Blob,
    /// The MAC to verify. Note that the previously available
    /// field `digest` is deprecated; this field should be used
    /// instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<Blob>
}

/// Request to verify a signature using an asymmetric key.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyRequest {
    /// Reference to the sobject to use for verification. This can be a key
    /// ID, key name, or a transient key blob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Hashing algorithm to use for signature verification. Specifically,
    /// this refers to the algorithm used to hash the data.
    ///
    /// Note that certain algorithms (SSL3, Blake2, Streebog) are disallowed
    /// in FIPS mode.
    pub hash_alg: DigestAlgorithm,
    /// The hash of the data on which the signature is being verified. Either
    /// `hash` or `data` should be specified; it is an error to specify both
    /// or none.
    /// Hash should be base64 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<Blob>,
    /// The data on which the signature is being verified. Either `hash` or
    /// `data` should be specified; it is an error to specify both or none.
    /// Data should be base64 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Blob>,
    /// Signature mechanism to use for verification. This is only relevant
    /// for RSA signatures; it should not be specified for all other signature
    /// algorithms.
    ///
    /// Additionally, in FIPS mode, if using PSS, the MGF1 hash function must
    /// be the same as the data hashing function (under the `alg`) field.
    ///
    /// If not specified for RSA signature verification, the backend will pick
    /// a default padding mode based on the key's padding policy (which may or
    /// may not satisfy any group or account-level cryptographic policies).
    /// Specifically,
    /// - The backend will try to use the first allowed signing padding policy
    ///   present in the sobject's signature padding policy.
    /// - If the "first allowed padding" is a wildcard policy (i.e., `{}`),
    ///   the backend will use PKCS #1 v1.5 padding. If the policy is instead
    ///   a wildcard PSS policy (where no MGF1 hash function is specified),
    ///   the backend will use PSS with SHA-256 as the MGF1 hash function.
    /// - In FIPS mode, the backend will try to avoid PSS padding modes where
    ///   the MGF1 hash function does not match the data hashing function.
    /// Once chosen, the padding mode will be validated against any group or
    /// account-level cryptographic policies. If this fails, the operation
    /// will error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<SignatureMode>,
    /// The signature to verify
    pub signature: ZeroizedBlob
}

/// Result of verifying a signature or MAC.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct VerifyResponse {
    /// The ID of the key used for verification. Returned for non-transient keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// True if the signature verified and false if it did not.
    pub result: bool
}

/// Request to wrap an sobject with another sobject.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WrapKeyRequest {
    /// Reference to the wrapping key. This can be a key ID, key name,
    /// or a transient key blob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<SobjectDescriptor>,
    /// Reference to the sobject being wrapped. This can be an sobject
    /// ID, sobject name, or a transient sobject blob.
    ///
    /// If specified, the `kid` field should not be present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<SobjectDescriptor>,
    /// ID of the sobject to be wrapped. (This is a legacy field,
    /// mutually exclusive with `subject`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Algorithm to use for key wrapping. The algorithm must be
    /// compatible with the key type; for example, an RSA key cannot
    /// be used with AES.
    pub alg: Algorithm,
    /// Encryption mode to use. This is required for wrapping via symmetric
    /// encryption. For RSA-based wrapping, the mode can be used to
    /// optionally specify the padding to use. For all other algorithms,
    /// this field should not be specified.
    ///
    /// If not specified for RSA encryption, the backend will pick a default
    /// padding mode based on the key's padding policy (which may or may not
    /// satisfy any group or account-level cryptographic policies).
    /// Specifically,
    /// - The backend will try to use the first allowed encryption padding
    ///   policy present in the sobject's encryption padding policy.
    /// - If the "first allowed padding" is a wildcard policy (i.e., `{}`),
    ///   the backend will use OAEP with SHA-256 as the MGF1 hash function.
    /// - In FIPS mode, if the "first allowed padding" is PKCS #1 v1.5 padding
    ///   or OAEP with SHA-1 as the MGF1 hash function, then the backend will
    ///   attempt to use a different padding mode, if allowed by the sobject.
    /// - Once chosen, the padding mode will be validated against any group or
    ///   account-level cryptographic policies. If this fails, the operation
    ///   will error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CryptMode>,
    /// The initialization vector to use. This is only applicable to modes
    /// that take IVs, and will be randomly generated if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// The authenticated data to use. This is only applicable when using
    /// authenticated encryption modes (i.e., GCM or CCM).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad: Option<Blob>,
    /// The length of the authentication tag, in bits, for authenticated
    /// encryption modes (i.e., GCM or CCM). For other modes, this field
    /// is irrelevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_len: Option<usize>,
    /// Format of the wrapped key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_format: Option<KeyFormat>
}

/// Result of a key wrapping request.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct WrapKeyResponse {
    /// The wrapped key blob
    pub wrapped_key: Blob,
    /// The initialization vector used during encryption. This is only
    /// applicable for certain symmetric encryption modes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iv: Option<Blob>,
    /// The authenticated tag returned from authenticated encryption
    /// (i.e., using GCM or CCM mode). For other modes, this field is
    /// not applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Blob>
}

pub struct OperationAgree;
#[allow(unused)]
impl Operation for OperationAgree {
    type PathParams = ();
    type QueryParams = ();
    type Body = AgreeKeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/agree".to_string()
    }
}

impl SdkmsClient {
    pub fn agree(&self, req: &AgreeKeyRequest) -> Result<Sobject> {
        self.execute::<OperationAgree>(req, (), None)
    }
    pub fn request_approval_to_agree(
        &self, req: &AgreeKeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationAgree>> {
        self.request_approval::<OperationAgree>(req, (), None, description)
    }
}

pub struct OperationBatchDecrypt;
#[allow(unused)]
impl Operation for OperationBatchDecrypt {
    type PathParams = ();
    type QueryParams = ();
    type Body = Vec<BatchDecryptRequestItem>;
    type Output = Vec<BatchResponseItem<DecryptResponse>>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/batch/decrypt".to_string()
    }
}

impl SdkmsClient {
    pub fn batch_decrypt(&self, req: &Vec<BatchDecryptRequestItem>) -> Result<Vec<BatchResponseItem<DecryptResponse>>> {
        self.execute::<OperationBatchDecrypt>(req, (), None)
    }
}

pub struct OperationBatchEncrypt;
#[allow(unused)]
impl Operation for OperationBatchEncrypt {
    type PathParams = ();
    type QueryParams = ();
    type Body = Vec<BatchEncryptRequestItem>;
    type Output = Vec<BatchResponseItem<EncryptResponse>>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/batch/encrypt".to_string()
    }
}

impl SdkmsClient {
    pub fn batch_encrypt(&self, req: &Vec<BatchEncryptRequestItem>) -> Result<Vec<BatchResponseItem<EncryptResponse>>> {
        self.execute::<OperationBatchEncrypt>(req, (), None)
    }
}

pub struct OperationBatchSign;
#[allow(unused)]
impl Operation for OperationBatchSign {
    type PathParams = ();
    type QueryParams = ();
    type Body = Vec<SignRequest>;
    type Output = Vec<BatchResponseItem<SignResponse>>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/batch/sign".to_string()
    }
}

impl SdkmsClient {
    pub fn batch_sign(&self, req: &Vec<SignRequest>) -> Result<Vec<BatchResponseItem<SignResponse>>> {
        self.execute::<OperationBatchSign>(req, (), None)
    }
    pub fn request_approval_to_batch_sign(
        &self, req: &Vec<SignRequest>,
        description: Option<String>) -> Result<PendingApproval<OperationBatchSign>> {
        self.request_approval::<OperationBatchSign>(req, (), None, description)
    }
}

pub struct OperationBatchVerify;
#[allow(unused)]
impl Operation for OperationBatchVerify {
    type PathParams = ();
    type QueryParams = ();
    type Body = Vec<VerifyRequest>;
    type Output = Vec<BatchResponseItem<VerifyResponse>>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/keys/batch/verify".to_string()
    }
}

impl SdkmsClient {
    pub fn batch_verify(&self, req: &Vec<VerifyRequest>) -> Result<Vec<BatchResponseItem<VerifyResponse>>> {
        self.execute::<OperationBatchVerify>(req, (), None)
    }
}

pub struct OperationCreateDigest;
#[allow(unused)]
impl Operation for OperationCreateDigest {
    type PathParams = ();
    type QueryParams = ();
    type Body = DigestRequest;
    type Output = DigestResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/digest".to_string()
    }
}

impl SdkmsClient {
    pub fn create_digest(&self, req: &DigestRequest) -> Result<DigestResponse> {
        self.execute::<OperationCreateDigest>(req, (), None)
    }
}

pub struct OperationDecapsulate;
#[allow(unused)]
impl Operation for OperationDecapsulate {
    type PathParams = ();
    type QueryParams = ();
    type Body = DecapsulateKeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/decapsulate".to_string()
    }
}

impl SdkmsClient {
    pub fn decapsulate(&self, req: &DecapsulateKeyRequest) -> Result<Sobject> {
        self.execute::<OperationDecapsulate>(req, (), None)
    }
    pub fn request_approval_to_decapsulate(
        &self, req: &DecapsulateKeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationDecapsulate>> {
        self.request_approval::<OperationDecapsulate>(req, (), None, description)
    }
}

pub struct OperationDecrypt;
#[allow(unused)]
impl Operation for OperationDecrypt {
    type PathParams = ();
    type QueryParams = ();
    type Body = DecryptRequest;
    type Output = DecryptResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/decrypt".to_string()
    }
}

impl SdkmsClient {
    pub fn decrypt(&self, req: &DecryptRequest) -> Result<DecryptResponse> {
        self.execute::<OperationDecrypt>(req, (), None)
    }
    pub fn request_approval_to_decrypt(
        &self, req: &DecryptRequest,
        description: Option<String>) -> Result<PendingApproval<OperationDecrypt>> {
        self.request_approval::<OperationDecrypt>(req, (), None, description)
    }
}

pub struct OperationDecryptFinal;
#[allow(unused)]
impl Operation for OperationDecryptFinal {
    type PathParams = ();
    type QueryParams = ();
    type Body = DecryptFinalRequest;
    type Output = DecryptFinalResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/decrypt/final".to_string()
    }
}

impl SdkmsClient {
    pub fn decrypt_final(&self, req: &DecryptFinalRequest) -> Result<DecryptFinalResponse> {
        self.execute::<OperationDecryptFinal>(req, (), None)
    }
}

pub struct OperationDecryptInit;
#[allow(unused)]
impl Operation for OperationDecryptInit {
    type PathParams = ();
    type QueryParams = ();
    type Body = DecryptInitRequest;
    type Output = DecryptInitResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/decrypt/init".to_string()
    }
}

impl SdkmsClient {
    pub fn decrypt_init(&self, req: &DecryptInitRequest) -> Result<DecryptInitResponse> {
        self.execute::<OperationDecryptInit>(req, (), None)
    }
}

pub struct OperationDecryptUpdate;
#[allow(unused)]
impl Operation for OperationDecryptUpdate {
    type PathParams = ();
    type QueryParams = ();
    type Body = DecryptUpdateRequest;
    type Output = DecryptUpdateResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/decrypt/update".to_string()
    }
}

impl SdkmsClient {
    pub fn decrypt_update(&self, req: &DecryptUpdateRequest) -> Result<DecryptUpdateResponse> {
        self.execute::<OperationDecryptUpdate>(req, (), None)
    }
}

pub struct OperationDerive;
#[allow(unused)]
impl Operation for OperationDerive {
    type PathParams = ();
    type QueryParams = ();
    type Body = DeriveKeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/derive".to_string()
    }
}

impl SdkmsClient {
    pub fn derive(&self, req: &DeriveKeyRequest) -> Result<Sobject> {
        self.execute::<OperationDerive>(req, (), None)
    }
    pub fn request_approval_to_derive(
        &self, req: &DeriveKeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationDerive>> {
        self.request_approval::<OperationDerive>(req, (), None, description)
    }
}

pub struct OperationEncapsulate;
#[allow(unused)]
impl Operation for OperationEncapsulate {
    type PathParams = ();
    type QueryParams = ();
    type Body = EncapsulateKeyRequest;
    type Output = EncapsulateKeyResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/encapsulate".to_string()
    }
}

impl SdkmsClient {
    pub fn encapsulate(&self, req: &EncapsulateKeyRequest) -> Result<EncapsulateKeyResponse> {
        self.execute::<OperationEncapsulate>(req, (), None)
    }
    pub fn request_approval_to_encapsulate(
        &self, req: &EncapsulateKeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationEncapsulate>> {
        self.request_approval::<OperationEncapsulate>(req, (), None, description)
    }
}

pub struct OperationEncrypt;
#[allow(unused)]
impl Operation for OperationEncrypt {
    type PathParams = ();
    type QueryParams = ();
    type Body = EncryptRequest;
    type Output = EncryptResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/encrypt".to_string()
    }
}

impl SdkmsClient {
    pub fn encrypt(&self, req: &EncryptRequest) -> Result<EncryptResponse> {
        self.execute::<OperationEncrypt>(req, (), None)
    }
    pub fn request_approval_to_encrypt(
        &self, req: &EncryptRequest,
        description: Option<String>) -> Result<PendingApproval<OperationEncrypt>> {
        self.request_approval::<OperationEncrypt>(req, (), None, description)
    }
}

pub struct OperationEncryptFinal;
#[allow(unused)]
impl Operation for OperationEncryptFinal {
    type PathParams = ();
    type QueryParams = ();
    type Body = EncryptFinalRequest;
    type Output = EncryptFinalResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/encrypt/final".to_string()
    }
}

impl SdkmsClient {
    pub fn encrypt_final(&self, req: &EncryptFinalRequest) -> Result<EncryptFinalResponse> {
        self.execute::<OperationEncryptFinal>(req, (), None)
    }
}

pub struct OperationEncryptInit;
#[allow(unused)]
impl Operation for OperationEncryptInit {
    type PathParams = ();
    type QueryParams = ();
    type Body = EncryptInitRequest;
    type Output = EncryptInitResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/encrypt/init".to_string()
    }
}

impl SdkmsClient {
    pub fn encrypt_init(&self, req: &EncryptInitRequest) -> Result<EncryptInitResponse> {
        self.execute::<OperationEncryptInit>(req, (), None)
    }
}

pub struct OperationEncryptUpdate;
#[allow(unused)]
impl Operation for OperationEncryptUpdate {
    type PathParams = ();
    type QueryParams = ();
    type Body = EncryptUpdateRequest;
    type Output = EncryptUpdateResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/encrypt/update".to_string()
    }
}

impl SdkmsClient {
    pub fn encrypt_update(&self, req: &EncryptUpdateRequest) -> Result<EncryptUpdateResponse> {
        self.execute::<OperationEncryptUpdate>(req, (), None)
    }
}

pub struct OperationMac;
#[allow(unused)]
impl Operation for OperationMac {
    type PathParams = ();
    type QueryParams = ();
    type Body = MacRequest;
    type Output = MacResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/mac".to_string()
    }
}

impl SdkmsClient {
    pub fn mac(&self, req: &MacRequest) -> Result<MacResponse> {
        self.execute::<OperationMac>(req, (), None)
    }
    pub fn request_approval_to_mac(
        &self, req: &MacRequest,
        description: Option<String>) -> Result<PendingApproval<OperationMac>> {
        self.request_approval::<OperationMac>(req, (), None, description)
    }
}

pub struct OperationMacVerify;
#[allow(unused)]
impl Operation for OperationMacVerify {
    type PathParams = ();
    type QueryParams = ();
    type Body = VerifyMacRequest;
    type Output = VerifyResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/macverify".to_string()
    }
}

impl SdkmsClient {
    pub fn mac_verify(&self, req: &VerifyMacRequest) -> Result<VerifyResponse> {
        self.execute::<OperationMacVerify>(req, (), None)
    }
}

pub struct OperationSign;
#[allow(unused)]
impl Operation for OperationSign {
    type PathParams = ();
    type QueryParams = ();
    type Body = SignRequest;
    type Output = SignResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/sign".to_string()
    }
}

impl SdkmsClient {
    pub fn sign(&self, req: &SignRequest) -> Result<SignResponse> {
        self.execute::<OperationSign>(req, (), None)
    }
    pub fn request_approval_to_sign(
        &self, req: &SignRequest,
        description: Option<String>) -> Result<PendingApproval<OperationSign>> {
        self.request_approval::<OperationSign>(req, (), None, description)
    }
}

pub struct OperationTransform;
#[allow(unused)]
impl Operation for OperationTransform {
    type PathParams = ();
    type QueryParams = ();
    type Body = TransformKeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/transform".to_string()
    }
}

impl SdkmsClient {
    pub fn transform(&self, req: &TransformKeyRequest) -> Result<Sobject> {
        self.execute::<OperationTransform>(req, (), None)
    }
    pub fn request_approval_to_transform(
        &self, req: &TransformKeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationTransform>> {
        self.request_approval::<OperationTransform>(req, (), None, description)
    }
}

pub struct OperationUnwrap;
#[allow(unused)]
impl Operation for OperationUnwrap {
    type PathParams = ();
    type QueryParams = ();
    type Body = UnwrapKeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/unwrapkey".to_string()
    }
}

impl SdkmsClient {
    pub fn unwrap(&self, req: &UnwrapKeyRequest) -> Result<Sobject> {
        self.execute::<OperationUnwrap>(req, (), None)
    }
    pub fn request_approval_to_unwrap(
        &self, req: &UnwrapKeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUnwrap>> {
        self.request_approval::<OperationUnwrap>(req, (), None, description)
    }
}

pub struct OperationVerify;
#[allow(unused)]
impl Operation for OperationVerify {
    type PathParams = ();
    type QueryParams = ();
    type Body = VerifyRequest;
    type Output = VerifyResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/verify".to_string()
    }
}

impl SdkmsClient {
    pub fn verify(&self, req: &VerifyRequest) -> Result<VerifyResponse> {
        self.execute::<OperationVerify>(req, (), None)
    }
}

pub struct OperationWrap;
#[allow(unused)]
impl Operation for OperationWrap {
    type PathParams = ();
    type QueryParams = ();
    type Body = WrapKeyRequest;
    type Output = WrapKeyResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/crypto/v1/wrapkey".to_string()
    }
}

impl SdkmsClient {
    pub fn wrap(&self, req: &WrapKeyRequest) -> Result<WrapKeyResponse> {
        self.execute::<OperationWrap>(req, (), None)
    }
    pub fn request_approval_to_wrap(
        &self, req: &WrapKeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationWrap>> {
        self.request_approval::<OperationWrap>(req, (), None, description)
    }
}

