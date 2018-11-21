/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// A cryptographic algorithm.
#[derive(Debug, Eq, PartialEq, Copy, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Algorithm {
    Aes,
    Des,
    Des3,
    Rsa,
    Ec,
    Hmac,
}

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

/// A request to encrypt data using a symmetric or asymmetric key.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct EncryptRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub alg: Algorithm,
    pub plain: Blob,
    /// Mode is required for symmetric algorithms.
    #[serde(default)]
    pub mode: Option<CryptMode>,
    /// Initialization vector is optional and will be randomly generated if not specified.
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Authenticated data is only applicable when using GCM mode.
    #[serde(default)]
    pub ad: Option<Blob>,
    /// Tag length is only applicable when using GCM mode.
    #[serde(default)]
    pub tag_len: Option<usize>,
}

/// Result of an encryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct EncryptResponse {
    /// Key id is returned for non-transient keys.
    #[serde(default)]
    pub kid: Option<Uuid>,
    pub cipher: Blob,
    /// Initialization vector is only returned for symmetric encryption.
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Tag is only returned for symmetric encryption with GCM mode.
    #[serde(default)]
    pub tag: Option<Blob>,
}

/// Initialize multi-part encryption. AEAD ciphers are not currently supported in this mode.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptInitRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub alg: Algorithm,
    /// Mode is required for symmetric encryption.
    #[serde(default)]
    pub mode: Option<CipherMode>,
    #[serde(default)]
    pub iv: Option<Blob>,
}

/// Result of initializing multi-part encryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct EncryptInitResponse {
    /// Key id is returned for non-transient keys.
    #[serde(default)]
    pub kid: Option<Uuid>,
    /// Initialization vector is only returned for symmetric encryption.
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Opaque data, not to be interpreted or modified by the client and must be provided with next request.
    pub state: Blob,
}

/// Multi-part encryption request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptUpdateRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub plain: Blob,
    pub state: Blob,
}

/// Result of multi-part encryption.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptUpdateResponse {
    pub cipher: Blob,
    /// Opaque data, not to be interpreted or modified by the client and must be provided with next request.
    pub state: Blob,
}

/// Finalize a multi-part encryption.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptFinalRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub state: Blob,
}

/// Final result of a multi-part encryption.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptFinalResponse {
    pub cipher: Blob,
}

/// A request to decrypt data using a symmetric or asymmetric key.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    #[serde(default)]
    pub alg: Option<Algorithm>,
    pub cipher: Blob,
    /// Mode is required for symmetric algorithms.
    #[serde(default)]
    pub mode: Option<CryptMode>,
    /// Initialization vector is required for symmetric algorithms.
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Authenticated data is only applicable when using GCM mode.
    #[serde(default)]
    pub ad: Option<Blob>,
    /// Tag is only applicable when using GCM mode.
    #[serde(default)]
    pub tag: Option<Blob>,
}

/// Result of a decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptResponse {
    /// The key id of the key used to decrypt. Returned for non-transient keys.
    #[serde(default)]
    pub kid: Option<Uuid>,
    pub plain: Blob,
}

/// Initialize multi-part decryption. AEAD ciphers are not currently supported in this mode.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptInitRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    #[serde(default)]
    pub alg: Option<Algorithm>,
    /// Mode is required for symmetric algorithms.
    #[serde(default)]
    pub mode: Option<CipherMode>,
    /// Initialization vector is required for symmetric algorithms.
    #[serde(default)]
    pub iv: Option<Blob>,
}

/// Result of initializing multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptInitResponse {
    /// The key id is returned for non-transient keys.
    #[serde(default)]
    pub kid: Option<Uuid>,
    /// Opaque data, not to be interpreted or modified by the client and must be provided with next request.
    pub state: Blob,
}

/// Multi-part decryption request.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptUpdateRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub cipher: Blob,
    pub state: Blob,
}

/// Result of multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptUpdateResponse {
    pub plain: Blob,
    /// Opaque data, not to be interpreted or modified by the client and must be provided with next request.
    pub state: Blob,
}

/// Finalize a multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptFinalRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub state: Blob,
}

/// Final result of a multi-part decryption.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DecryptFinalResponse {
    pub plain: Blob,
}

/// Request to compute the hash of arbitrary data.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DigestRequest {
    pub alg: DigestAlgorithm,
    pub data: Blob,
}

/// Result of a hash operation.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct DigestResponse {
    pub digest: Blob,
}

/// Request for HMAC or CMAC operation.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MacRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    #[serde(default)]
    pub alg: Option<DigestAlgorithm>,
    pub data: Blob,
}

/// Result of HMAC or CMAC operation.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MacResponse {
    #[serde(default)]
    pub kid: Option<Uuid>,
    /// This field is retained for backward compatibility in API for HMAC.
    #[serde(default)]
    pub digest: Option<Blob>,
    /// The MAC generated for the input data.
    pub mac: Blob,
}

/// Rquest to verify a MAC value.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyMacRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    /// Algorithm is required for HMAC.
    #[serde(default)]
    pub alg: Option<DigestAlgorithm>,
    pub data: Blob,
    /// This field is deprecated. Instead you should use the `mac` field.
    #[serde(default)]
    pub digest: Option<Blob>,
    /// Either `digest` or `mac` should be specified.
    #[serde(default)]
    pub mac: Option<Blob>,
}

/// Encodes the mechanism to be used when deriving a new key from an existing key.
/// Currently, the only supported mechanism is encrypting data to derive the new key.
/// Other mechanisms may be added in the future.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DeriveKeyMechanism {
    EncryptData(EncryptRequest),
}

/// Request to derive a key.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeriveKeyRequest {
    #[serde(default)]
    pub activation_date: Option<Time>,
    #[serde(default)]
    pub deactivation_date: Option<Time>,
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    /// Name of the derived key. Key names must be unique within an account.
    pub name: Option<String>,
    /// Group ID of the security group that this security object should belong to. The user or
    /// application creating this security object must be a member of this group. If no group is
    /// specified, the default group for the requesting application will be used.
    #[serde(default)]
    pub group_id: Option<Uuid>,
    /// Type of key to be derived.
    pub key_type: ObjectType,
    /// Key size of the derived key in bits.
    pub key_size: u32,
    /// Mechanism to use for key derivation.
    pub mechanism: DeriveKeyMechanism,
    /// Whether the derived key should have cryptographic operations enabled.
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    /// User-defined metadata for this key stored as key-value pairs.
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    /// Optional array of key operations to be enabled for this security object. If not
    /// provided the service will provide a default set of key operations. Note that if you
    /// provide an empty array, all key operations will be disabled.
    #[serde(default)]
    pub key_ops: Option<KeyOperations>,
    #[serde(default)]
    pub state: Option<SobjectState>,
    /// If set to true, the derived key will be transient.
    #[serde(default)]
    pub transient: Option<bool>,
}

/// Mechanism to use for key agreement.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AgreeKeyMechanism {
    DiffieHellman,
}

/// Request to perform key agreement.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgreeKeyRequest {
    #[serde(default)]
    pub activation_date: Option<Time>,
    #[serde(default)]
    pub deactivation_date: Option<Time>,
    pub private_key: SobjectDescriptor,
    pub public_key: SobjectDescriptor,
    /// Mechanism to use for key derivation.
    pub mechanism: AgreeKeyMechanism,
    /// Name of the agreed-upon key. Key names must be unique within an account.
    /// The name is ignored for transient keys.
    pub name: Option<String>,
    /// Group ID of the security group that this security object should belong to. The user or
    /// application creating this security object must be a member of this group. If no group is
    /// specified, the default group for the requesting application will be used.
    #[serde(default)]
    pub group_id: Option<Uuid>,
    /// Type of key to be derived. NB. for security reasons, you shouldn't specify anything but HMAC or Secret.
    pub key_type: ObjectType,
    /// Key size in bits. If less than the output size of the algorithm, the secret's most-significant bits will be truncated.
    pub key_size: u32,
    pub enabled: bool,
    #[serde(default)]
    pub description: Option<String>,
    /// User-defined metadata for this key stored as key-value pairs.
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    /// Optional array of key operations to be enabled for this security object. If not
    /// provided the service will provide a default set of key operations. Note that if you
    /// provide an empty array, all key operations will be disabled.
    #[serde(default)]
    pub key_ops: Option<KeyOperations>,
    #[serde(default)]
    pub state: Option<SobjectState>,
    /// If set to true, the resulting key will be transient.
    pub transient: bool,
}

/// `CipherMode` or `RsaEncryptionPadding`, depending on the encryption algorithm.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CryptMode {
    Symmetric(CipherMode),
    Rsa(RsaEncryptionPadding),
}

/// Type of padding to use for RSA encryption. The use of PKCS#1 v1.5 padding is strongly
/// discouraged, because of its susceptibility to Bleichenbacher's attack. The padding specified
/// must adhere to the key's encryption policy. If not specified, the default based on the key's
/// policy will be used.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaEncryptionPadding {
    /// Optimal Asymmetric Encryption Padding (PKCS#1 v2.1).
    Oaep { mgf: Mgf },
    /// PKCS#1 v1.5 padding.
    Pkcs1V15 {},
}

/// Request to perform key wrapping.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WrapKeyRequest {
    /// The wrapping key.
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    /// The key to be wrapped.
    #[serde(default)]
    pub subject: Option<SobjectDescriptor>,
    /// Id of the key to be wrapped (legacy, mutually exclusive with `subject`).
    #[serde(default)]
    pub kid: Option<Uuid>,
    pub alg: Algorithm,
    /// Mode is required for symmetric algorithms.
    #[serde(default)]
    pub mode: Option<CryptMode>,
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Authenticated data is only applicable if mode is GCM.
    #[serde(default)]
    pub ad: Option<Blob>,
    /// Tag length is required when mode is GCM.
    #[serde(default)]
    pub tag_len: Option<usize>,
}

/// Result of key wrapping operation.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct WrapKeyResponse {
    pub wrapped_key: Blob,
    /// Initialization vector is only returned for symmetric algorithms.
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Tag is only returned for symmetric algorithm with GCM mode.
    #[serde(default)]
    pub tag: Option<Blob>,
}

/// Request to perform key unwrapping.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct UnwrapKeyRequest {
    #[serde(default)]
    pub key: Option<SobjectDescriptor>,
    pub alg: Algorithm,
    /// Object type of the key being unwrapped.
    pub obj_type: ObjectType,
    #[serde(default)]
    pub rsa: Option<RsaOptions>,
    /// A Security Object previously wrapped with another key.
    pub wrapped_key: Blob,
    /// Mode is required for symmetric algorithms.
    #[serde(default)]
    pub mode: Option<CryptMode>,
    /// Initialization vector is required for symmetric algorithms.
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Authenticated data is only applicable if mode is GCM.
    #[serde(default)]
    pub ad: Option<Blob>,
    /// Tag is required if mode is GCM.
    #[serde(default)]
    pub tag: Option<Blob>,
    /// Name to be given to the resulting security object if persisted.
    pub name: Option<String>,
    /// Group ID of the security group that the resulting security object should belong to. The user or
    /// application creating this security object must be a member of this group. If no group is
    /// specified, the default group for the requesting application will be used.
    #[serde(default)]
    pub group_id: Option<Uuid>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    /// User-defined metadata for the resulting key stored as key-value pairs.
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    /// Optional array of key operations to be enabled for the resulting security object. If not
    /// provided the service will provide a default set of key operations. Note that if you provide
    /// an empty array, all key operations will be disabled.
    #[serde(default)]
    pub key_ops: Option<KeyOperations>,
    #[serde(default)]
    pub transient: Option<bool>,
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/encrypt")
    }
}

impl SdkmsClient {
    pub fn encrypt(&self, req: &EncryptRequest) -> Result<EncryptResponse> {
        self.execute::<OperationEncrypt>(req, (), &())
    }
    pub fn request_approval_to_encrypt(
        &self,
        req: &EncryptRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationEncrypt>> {
        self.request_approval::<OperationEncrypt>(req, (), &(), description)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/encrypt/init")
    }
}

impl SdkmsClient {
    pub fn encrypt_init(&self, req: &EncryptInitRequest) -> Result<EncryptInitResponse> {
        self.execute::<OperationEncryptInit>(req, (), &())
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/encrypt/update")
    }
}

impl SdkmsClient {
    pub fn encrypt_update(&self, req: &EncryptUpdateRequest) -> Result<EncryptUpdateResponse> {
        self.execute::<OperationEncryptUpdate>(req, (), &())
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/encrypt/final")
    }
}

impl SdkmsClient {
    pub fn encrypt_final(&self, req: &EncryptFinalRequest) -> Result<EncryptFinalResponse> {
        self.execute::<OperationEncryptFinal>(req, (), &())
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/decrypt")
    }
}

impl SdkmsClient {
    pub fn decrypt(&self, req: &DecryptRequest) -> Result<DecryptResponse> {
        self.execute::<OperationDecrypt>(req, (), &())
    }
    pub fn request_approval_to_decrypt(
        &self,
        req: &DecryptRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationDecrypt>> {
        self.request_approval::<OperationDecrypt>(req, (), &(), description)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/decrypt/init")
    }
}

impl SdkmsClient {
    pub fn decrypt_init(&self, req: &DecryptInitRequest) -> Result<DecryptInitResponse> {
        self.execute::<OperationDecryptInit>(req, (), &())
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/decrypt/update")
    }
}

impl SdkmsClient {
    pub fn decrypt_update(&self, req: &DecryptUpdateRequest) -> Result<DecryptUpdateResponse> {
        self.execute::<OperationDecryptUpdate>(req, (), &())
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/decrypt/final")
    }
}

impl SdkmsClient {
    pub fn decrypt_final(&self, req: &DecryptFinalRequest) -> Result<DecryptFinalResponse> {
        self.execute::<OperationDecryptFinal>(req, (), &())
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/sign")
    }
}

impl SdkmsClient {
    pub fn sign(&self, req: &SignRequest) -> Result<SignResponse> {
        self.execute::<OperationSign>(req, (), &())
    }
    pub fn request_approval_to_sign(
        &self,
        req: &SignRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationSign>> {
        self.request_approval::<OperationSign>(req, (), &(), description)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/verify")
    }
}

impl SdkmsClient {
    pub fn verify(&self, req: &VerifyRequest) -> Result<VerifyResponse> {
        self.execute::<OperationVerify>(req, (), &())
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/wrapkey")
    }
}

impl SdkmsClient {
    pub fn wrap(&self, req: &WrapKeyRequest) -> Result<WrapKeyResponse> {
        self.execute::<OperationWrap>(req, (), &())
    }
    pub fn request_approval_to_wrap(
        &self,
        req: &WrapKeyRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationWrap>> {
        self.request_approval::<OperationWrap>(req, (), &(), description)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/unwrapkey")
    }
}

impl SdkmsClient {
    pub fn unwrap(&self, req: &UnwrapKeyRequest) -> Result<Sobject> {
        self.execute::<OperationUnwrap>(req, (), &())
    }
    pub fn request_approval_to_unwrap(
        &self,
        req: &UnwrapKeyRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationUnwrap>> {
        self.request_approval::<OperationUnwrap>(req, (), &(), description)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/mac")
    }
}

impl SdkmsClient {
    pub fn mac(&self, req: &MacRequest) -> Result<MacResponse> {
        self.execute::<OperationMac>(req, (), &())
    }
    pub fn request_approval_to_mac(
        &self,
        req: &MacRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationMac>> {
        self.request_approval::<OperationMac>(req, (), &(), description)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/macverify")
    }
}

impl SdkmsClient {
    pub fn mac_verify(&self, req: &VerifyMacRequest) -> Result<VerifyResponse> {
        self.execute::<OperationMacVerify>(req, (), &())
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/derive")
    }
}

impl SdkmsClient {
    pub fn derive(&self, req: &DeriveKeyRequest) -> Result<Sobject> {
        self.execute::<OperationDerive>(req, (), &())
    }
    pub fn request_approval_to_derive(
        &self,
        req: &DeriveKeyRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationDerive>> {
        self.request_approval::<OperationDerive>(req, (), &(), description)
    }
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/agree")
    }
}

impl SdkmsClient {
    pub fn agree(&self, req: &AgreeKeyRequest) -> Result<Sobject> {
        self.execute::<OperationAgree>(req, (), &())
    }
    pub fn request_approval_to_agree(
        &self,
        req: &AgreeKeyRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationAgree>> {
        self.request_approval::<OperationAgree>(req, (), &(), description)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/crypto/v1/digest")
    }
}

impl SdkmsClient {
    pub fn create_digest(&self, req: &DigestRequest) -> Result<DigestResponse> {
        self.execute::<OperationCreateDigest>(req, (), &())
    }
}
